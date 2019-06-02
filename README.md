# basE91 implementation for Rust

[![CircleCI](https://circleci.com/gh/dnsl48/base91/tree/master.svg?style=svg)](https://circleci.com/gh/dnsl48/base91/tree/master) [![Documentation](https://docs.rs/base91/badge.svg)](https://docs.rs/base91/) [![Current Version on crates.io](https://img.shields.io/crates/v/base91.svg)](https://crates.io/crates/base91/) [![MIT / Apache2 License](https://img.shields.io/badge/license-MIT%20/%20Apache2-blue.svg)]()

basE91 is an advanced method for encoding binary data as ASCII characters. It is similar to base64, but is more efficient. The overhead produced by basE91 depends on the input data. It amounts at most to 23% (versus 33% for base64) and can range down to 14%, which typically occurs on 0-byte blocks.

For more information see http://base91.sourceforge.net/

# License

The current Rust implementation is provided under the dual `MIT/Apache2` license.

# Acknowledgement

The original basE91 has been developed by Joachim Henke, and is released as free software under the terms of the BSD license
