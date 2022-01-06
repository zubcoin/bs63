#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(variant_size_differences)]
#![doc(test(attr(deny(warnings))))]

//! Another [Base58][] codec implementation.
//!
//! Compared to [`base58`][] this is significantly faster at decoding (about
//! 2.4x as fast when decoding 32 bytes), almost the same speed for encoding
//! (about 3% slower when encoding 32 bytes) and doesn't have the 128 byte
//! limitation.
//!
//! Compared to [`rust-base58`][] this is massively faster (over ten times as
//! fast when decoding 32 bytes, almost 40 times as fast when encoding 32
//! bytes) and has no external dependencies.
//!
//! Compared to both this supports a configurable alphabet and user provided
//! buffers for zero-allocation {en,de}coding.
//!
//! [Base58]: https://en.wikipedia.org/wiki/Base58
//! [`base58`]: https://github.com/debris/base58
//! [`rust-base58`]: https://github.com/nham/rust-base58
//!
//! # Features
//!
//!  Feature | Activation         | Effect
//! ---------|--------------------|--------
//!  `std`   | **on**-by-default  | Implement [`Error`](std::error::Error) for error types
//!  `alloc` | implied by `std`   | Support encoding/decoding to [`Vec`](alloc::vec::Vec) and [`String`](alloc::string::String) as appropriate
//!  `check` | **off**-by-default | Integrated support for [Base58Check][]
//!
//! [Base58Check]: https://en.bitcoin.it/wiki/Base58Check_encoding

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod alphabet;
#[doc(inline)]
pub use alphabet::Alphabet;

pub mod decode;
pub mod encode;

#[cfg(feature = "check")]
const CHECKSUM_LEN: usize = 4;

/// Possible check variants.
enum Check {
    Disabled,
    #[cfg(feature = "check")]
    Enabled(Option<u8>),
}

/// Setup decoder for the given string using the [default alphabet][Alphabet::DEFAULT].
pub fn decode<I: AsRef<[u8]>>(input: I) -> decode::DecodeBuilder<'static, I> {
    decode::DecodeBuilder::from_input(input)
}

/// Setup encoder for the given bytes using the [default alphabet][Alphabet::DEFAULT].
pub fn encode<I: AsRef<[u8]>>(input: I) -> encode::EncodeBuilder<'static, I> {
    encode::EncodeBuilder::from_input(input)
}
