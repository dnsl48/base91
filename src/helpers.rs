use crate::{DecodeIterator, EncodeIterator};

#[deprecated(since="0.1.0", note="Convert to EncodeIterator::new(data).for_each(out)")]
pub fn iter_encode<I, O>(data: I, out: O)
where
    I: Iterator<Item = u8>,
    O: FnMut(u8),
{
    EncodeIterator::new(data).for_each(out);
}

#[deprecated(since="0.1.0", note="Convert to EncodeIterator::new(data).for_each(out)")]
pub fn iter_decode<I, O>(data: I, out: O)
where
    I: Iterator<Item = u8>,
    O: FnMut(u8),
{
    DecodeIterator::new(data).for_each(out);
}

/// Encode the `value` into ASCII according to base91 method.
/// 
/// Preallocates the destination Vector once and writes the encoded data into it.
#[cfg(any(feature = "std", test))]
pub fn slice_encode(value: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(value.len() * 13 / 10);

    EncodeIterator::new(value.iter().copied()).for_each(|i| result.push(i));

    result
}

/// Decode the `value` into binary data according to base91 method.
/// 
/// Preallocates the destination Vector once and writes the encoded data into it.
#[cfg(any(feature = "std", test))]
pub fn slice_decode(value: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(value.len());

    DecodeIterator::new(value.iter().copied()).for_each(|i| result.push(i));

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn get_pairs() -> Vec<(&'static str, &'static str)> {
        let data = vec![
            ("test", "fPNKd"),
            ("vest", "hPNKd"),
            (
                "5Fq99ztBNtv+NsWSdNS04dnyiC81Qf4dsbz6Y5elKaR+KVsAWoiK0SdBiVg2hC/FXpX0Zozw8Hd4",
                "qRqgWoRZ!L0/|msb}%dHM3;BQJX%1Q$XowN0=kHTcR5<Q81jMgz1qelja%$gNQva~1;1C:Zp>I.E2*Df))Xxc>Gq_JDzbC"
            )
        ];

        data
    }

    #[test]
    fn test_encode() {
        for pair in get_pairs() {
            assert_eq!(
                &String::from_utf8_lossy(&slice_encode(pair.0.as_bytes())[..]),
                pair.1
            );
        }
    }

    #[test]
    fn test_decode() {
        for pair in get_pairs() {
            assert_eq!(
                &String::from_utf8_lossy(&slice_decode(pair.1.as_bytes())[..]),
                pair.0
            );
        }
    }

    #[test]
    fn test_integrity() {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        let mut buf: [u8; 20480] = [0; 20480];

        for _ in 0..512 {
            for i in 0..2560 {
                let mut hasher = RandomState::new().build_hasher();
                hasher.write_u32(1);
                let value = hasher.finish();
                let bytes = value.to_ne_bytes();

                for j in 0..8 {
                    buf[i * 8 + j] = bytes[j];
                }
            }

            let encoded = slice_encode(&buf);
            let decoded = slice_decode(&encoded);

            assert_eq!(&decoded[..], &buf[..]);
        }
    }
}
