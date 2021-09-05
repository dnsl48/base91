use core::iter::Iterator;
use core::marker::PhantomData;

use crate::codemap::Encoding;

pub struct DecodeGenericIterator<E, I>
where
    E: Encoding,
    I: Iterator<Item = u8>,
{
    buf: i32,
    rem: i32,
    shift: i32,
    data: I,
    codemap: PhantomData<E>,
}

impl<E, I> DecodeGenericIterator<E, I>
where
    E: Encoding,
    I: Iterator<Item = u8>,
{
    pub fn new(data: I) -> Self {
        Self {
            buf: -1,
            rem: 0,
            shift: 0,
            data,
            codemap: PhantomData,
        }
    }
}

impl<E, I> Iterator for DecodeGenericIterator<E, I>
where
    E: Encoding,
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.shift > 7 {
                let result = self.rem as u8;
                self.rem >>= 8;
                self.shift -= 8;
                return Some(result);
            } else if let Some(b) = self.data.next() {
                let key = E::decode_byte(b);

                if key == 91 {
                    continue;
                }

                if self.buf == -1 {
                    self.buf = key as i32;
                } else {
                    self.buf += key as i32 * 91;
                    self.rem |= self.buf << self.shift;
                    self.shift += if (self.buf & 8191) > 88 { 13 } else { 14 };

                    self.buf = -1;

                    let result = self.rem as u8;

                    self.rem >>= 8;
                    self.shift -= 8;

                    return Some(result);
                }
            } else if self.buf != -1 {
                let result = (self.rem | self.buf << self.shift) as u8;
                self.buf = -1;
                return Some(result);
            } else {
                return None;
            }
        }
    }
}
