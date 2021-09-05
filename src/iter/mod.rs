mod decode;
mod encode;

pub use self::decode::DecodeGenericIterator;
pub use self::encode::EncodeGenericIterator;

#[cfg(feature = "canonical")]
pub type EncodeIterator<I> = EncodeGenericIterator<crate::codemap::Canonical, I>;

#[cfg(feature = "canonical")]
pub type DecodeIterator<I> = DecodeGenericIterator<crate::codemap::Canonical, I>;


#[cfg(all(feature = "xml-friendly", not(feature = "canonical")))]
pub type EncodeIterator<I> = EncodeGenericIterator<crate::codemap::XmlFriendly, I>;

#[cfg(all(feature = "xml-friendly", not(feature = "canonical")))]
pub type DecodeIterator<I> = DecodeGenericIterator<crate::codemap::XmlFriendly, I>;