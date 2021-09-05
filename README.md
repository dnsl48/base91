# basE91 implementation for Rust

[![CircleCI](https://circleci.com/gh/dnsl48/base91/tree/master.svg?style=svg)](https://circleci.com/gh/dnsl48/base91/tree/master) [![Documentation](https://docs.rs/base91/badge.svg)](https://docs.rs/base91/) [![Current Version on crates.io](https://img.shields.io/crates/v/base91.svg)](https://crates.io/crates/base91/) [![MIT / Apache2 License](https://img.shields.io/badge/license-MIT%20/%20Apache2-blue.svg)]()

basE91 is an advanced method for encoding binary data as ASCII characters. It is similar to base64, but is more efficient. The overhead produced by basE91 depends on the input data. It amounts at most to 23% (versus 33% for base64) and can range down to 14%, which typically occurs on 0-byte blocks.

For more information see http://base91.sourceforge.net/

Joachim Henke's [basE91 encoding](http://base91.sourceforge.net) implementation for Rust.

Also supports [xml-friendly base91](https://github.com/r-lyeh-archived/base)
implementation by [r-lyeh](https://github.com/r-lyeh), which supports quoting and is
JSON, XML and TSV friendly.

# Library features

 - no_std
 - Encode/Decode iterators
 - Canonical (Joachim Hanke) [implementation](http://base91.sourceforge.net)
 - Alternative XML, JSON and TSV friendly ([r-lyeh](https://github.com/r-lyeh)) [implementation](https://github.com/r-lyeh-archived/base)

# Examples

## Using helpers

```rust
let source = "This is a random binary string";

let encode_as_vec: Vec<u8> = base91::slice_encode(source.as_bytes());
let decode_as_vec: Vec<u8> = base91::slice_decode(&encode_as_vec);

assert_eq!(
    "nX,<:WRT%yxtYQ:mbr4JB*H[LR/@Qj{o0aU=Z",
    String::from_utf8_lossy(&encode_as_vec)
);

assert_eq!(
    "This is a random binary string",
    String::from_utf8_lossy(&decode_as_vec)
);
```


## Using Iterators

```rust
let source = "This is a random binary string";

let as_vec: Vec<u8> = base91::EncodeIterator::new(source.bytes()).collect();
let as_str: String = base91::EncodeIterator::new(source.bytes()).as_char_iter().collect();

assert_eq!(
    "nX,<:WRT%yxtYQ:mbr4JB*H[LR/@Qj{o0aU=Z",
    String::from_utf8_lossy(&as_vec)
);

assert_eq!(
    "nX,<:WRT%yxtYQ:mbr4JB*H[LR/@Qj{o0aU=Z",
    as_str
);
```

## Using Xml Friendly or another specific variation

Warning!
  If you disable the feature `canonical` and enable the feature `xml-friendly`, then
  `base91::EncodeIterator` and `base91::DecodeIterator` will silently use XmlFriendly variant.
  In turn, all the helper functions use the iterators internally (`slice_encode` and `slice_decode`).

```rust
use base91::iter::{EncodeGenericIterator, DecodeGenericIterator};
use base91::codemap::XmlFriendly;

let source = "This is a random binary string";

let encode_as_vec: Vec<u8> = EncodeGenericIterator::<XmlFriendly, _>::new(source.bytes()).collect();
let decode_as_vec: Vec<u8> = DecodeGenericIterator::<XmlFriendly, _>::new(encode_as_vec.iter().copied()).collect();

assert_eq!(
    "nX,-:WRT%yxtYQ:mbr4JB*H[LR/@Qj{o0aU=Z",
    String::from_utf8_lossy(&encode_as_vec)
);

assert_eq!(
    "This is a random binary string",
    String::from_utf8_lossy(&decode_as_vec)
);
```

# License

The current Rust implementation is provided under the dual `MIT/Apache2` license.

# Acknowledgement

The original basE91 has been developed by Joachim Henke, and is released as free software under the terms of the BSD license
