use core::iter::Iterator;
use core::marker::PhantomData;

use crate::codemap::Encoding;

pub struct EncodeGenericIterator<E, I>
where
    E: Encoding,
    I: Iterator<Item = u8>,
{
    buf: u8,
    rem: u32,
    shift: u32,
    data: I,
    codemap: PhantomData<E>,
}

impl<E, I> EncodeGenericIterator<E, I>
where
    E: Encoding,
    I: Iterator<Item = u8>,
{
    pub fn new(data: I) -> Self {
        Self {
            buf: 0,
            rem: 0,
            shift: 0,
            data,
            codemap: PhantomData,
        }
    }

    pub fn as_char_iter(self) -> core::iter::Map<EncodeGenericIterator<E, I>, impl FnMut(u8) -> char> {
        self.map(|x| x as char)
    }
}

impl<E, I> Iterator for EncodeGenericIterator<E, I>
where
    E: Encoding,
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf > 0 {
            let value = self.buf;
            self.buf = 0;
            return Some(value);
        }

        loop {
            if let Some(b) = self.data.next() {
                self.rem |= (b as u32) << self.shift;
                self.shift += 8;

                if self.shift > 13 {
                    let mut key: u32 = self.rem & 8191;
                    if key > 88 {
                        self.rem >>= 13;
                        self.shift -= 13;
                    } else {
                        key = self.rem & 16383;
                        self.rem >>= 14;
                        self.shift -= 14;
                    }

                    self.buf = E::encode_byte((key / 91) as u8);

                    return Some(E::encode_byte((key % 91) as u8));
                }
            } else if self.shift > 0 {
                let value = E::encode_byte((self.rem % 91) as u8);
                if self.shift > 7 || self.rem > 90 {
                    self.buf = E::encode_byte((self.rem / 91) as u8);
                }

                self.shift = 0;
                self.rem = 0;

                return Some(value);
            } else {
                return None;
            }
        }
    }
}
