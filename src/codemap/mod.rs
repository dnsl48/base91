#[cfg(feature = "canonical")]
pub mod canonical;

#[cfg(feature = "xml-friendly")]
pub mod xml_friendly;

#[cfg(feature = "canonical")]
pub use canonical::Canonical;

#[cfg(feature = "xml-friendly")]
pub use xml_friendly::XmlFriendly;

pub trait Encoding {
    fn encode_byte(key: u8) -> u8;

    fn decode_byte(key: u8) -> u8;
}
