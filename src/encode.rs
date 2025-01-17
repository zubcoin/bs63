//! Functions for encoding into Base63 encoded strings.

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

use crate::Check;
#[cfg(feature = "check")]
use crate::CHECKSUM_LEN;

use crate::Alphabet;

/// A builder for setting up the alphabet and output of a base63 encode.
#[allow(missing_debug_implementations)]
pub struct EncodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a Alphabet,
    check: Check,
}

/// A specialized [`Result`](core::result::Result) type for [`bs63::encode`](module@crate::encode)
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that could occur when encoding a Base63 encoded string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// The output buffer was too small to contain the entire input.
    BufferTooSmall,
}

/// Represents a buffer that can be encoded into. See [`EncodeBuilder::into`] and the provided
/// implementations for more details.
pub trait EncodeTarget {
    /// Encodes into this buffer, provides the maximum length for implementations that wish to
    /// preallocate space, along with a function that will encode ASCII bytes into the buffer and
    /// return the length written to it.
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize>;
}

impl<T: EncodeTarget + ?Sized> EncodeTarget for &mut T {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        T::encode_with(self, max_len, f)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
impl EncodeTarget for Vec<u8> {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let original = self.len();
        self.resize(original + max_len, 0);
        let len = f(&mut self[original..])?;
        self.truncate(original + len);
        Ok(len)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
impl EncodeTarget for String {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let mut output = core::mem::take(self).into_bytes();
        let len = output.encode_with(max_len, f)?;
        *self = String::from_utf8(output).unwrap();
        Ok(len)
    }
}

impl EncodeTarget for [u8] {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        let _ = max_len;
        f(&mut *self)
    }
}

impl EncodeTarget for str {
    fn encode_with(
        &mut self,
        max_len: usize,
        f: impl for<'a> FnOnce(&'a mut [u8]) -> Result<usize>,
    ) -> Result<usize> {
        struct Guard<'a>(&'a mut [u8]);

        impl Drop for Guard<'_> {
            fn drop(&mut self) {
                let mut index = 0;
                loop {
                    match core::str::from_utf8(&self.0[index..]) {
                        Ok(_) => return,
                        Err(e) => {
                            index += e.valid_up_to();
                            if let Some(len) = e.error_len() {
                                for i in &mut self.0[index..index + len] {
                                    *i = 0;
                                }
                                index += len;
                            } else {
                                for i in &mut self.0[index..] {
                                    *i = 0;
                                }
                                index += self.0[index..].len();
                            }
                        }
                    }
                }
            }
        }

        let _ = max_len;

        let guard = Guard(unsafe { self.as_bytes_mut() });
        f(&mut *guard.0)
    }
}

impl<'a, I: AsRef<[u8]>> EncodeBuilder<'a, I> {
    /// Setup encoder for the given string using the given alphabet.
    /// Preferably use [`bs63::encode`](crate::encode()) instead of this
    /// directly.
    pub fn new(input: I, alpha: &'a Alphabet) -> EncodeBuilder<'a, I> {
        EncodeBuilder {
            input,
            alpha,
            check: Check::Disabled,
        }
    }

    /// Setup encoder for the given string using default prepared alphabet.
    pub(crate) fn from_input(input: I) -> EncodeBuilder<'static, I> {
        EncodeBuilder {
            input,
            alpha: Alphabet::DEFAULT,
            check: Check::Disabled,
        }
    }

    /// Change the alphabet that will be used for encoding.
    pub fn with_alphabet(self, alpha: &'a Alphabet) -> EncodeBuilder<'a, I> {
        EncodeBuilder { alpha, ..self }
    }

    /// Include checksum calculated using the [Base63Check][] algorithm when
    /// encoding.
    ///
    /// [Base63Check]: https://en.bitcoin.it/wiki/Base63Check_encoding
    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    pub fn with_check(self) -> EncodeBuilder<'a, I> {
        let check = Check::Enabled(None);
        EncodeBuilder { check, ..self }
    }

    /// Include checksum calculated using the [Base63Check][] algorithm and
    /// version when encoding.
    ///
    /// [Base63Check]: https://en.bitcoin.it/wiki/Base63Check_encoding
    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    pub fn with_check_version(self, expected_ver: u8) -> EncodeBuilder<'a, I> {
        let check = Check::Enabled(Some(expected_ver));
        EncodeBuilder { check, ..self }
    }

    /// Encode into a new owned string.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn into_string(self) -> String {
        let mut output = String::new();
        self.into(&mut output).unwrap();
        output
    }

    /// Encode into a new owned vector.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn into_vec(self) -> Vec<u8> {
        let mut output = Vec::new();
        self.into(&mut output).unwrap();
        output
    }

    /// Encode into the given buffer.
    ///
    /// Returns the length written into the buffer.
    ///
    /// If the buffer is resizeable it will be extended and the new data will be written to the end
    /// of it.
    ///
    /// If the buffer is not resizeable bytes after the final character will be left alone, except
    /// up to 3 null bytes may be written to an `&mut str` to overwrite remaining characters of a
    /// partially overwritten multi-byte character.
    ///
    /// See the documentation for [`bs63::encode`](crate::encode()) for an
    /// explanation of the errors that may occur.
    pub fn into(self, mut output: impl EncodeTarget) -> Result<usize> {
        match self.check {
            Check::Disabled => {
                let max_encoded_len = (self.input.as_ref().len() / 5 + 1) * 8;
                output.encode_with(max_encoded_len, |output| {
                    encode_into(self.input.as_ref(), output, self.alpha)
                })
            }
            #[cfg(feature = "check")]
            Check::Enabled(version) => {
                let max_encoded_len = ((self.input.as_ref().len() + CHECKSUM_LEN) / 5 + 1) * 8;
                output.encode_with(max_encoded_len, |output| {
                    encode_check_into(self.input.as_ref(), output, &self.alpha, version)
                })
            }
        }
    }
}

fn encode_into<'a, I>(input: I, output: &mut [u8], alpha: &Alphabet) -> Result<usize>
where
    I: Clone + IntoIterator<Item = &'a u8>,
{
    let mut index = 0;
    for &val in input.clone() {
        let mut carry = val as usize;
        for byte in &mut output[..index] {
            carry += (*byte as usize) << 8;
            *byte = (carry % 63) as u8;
            carry /= 63;
        }
        while carry > 0 {
            if index == output.len() {
                return Err(Error::BufferTooSmall);
            }
            output[index] = (carry % 63) as u8;
            index += 1;
            carry /= 63;
        }
    }

    for _ in input.into_iter().take_while(|v| **v == 0) {
        if index == output.len() {
            return Err(Error::BufferTooSmall);
        }
        output[index] = 0;
        index += 1;
    }

    for val in &mut output[..index] {
        *val = alpha.encode[*val as usize];
    }

    output[..index].reverse();
    Ok(index)
}

#[cfg(feature = "check")]
fn encode_check_into(
    input: &[u8],
    output: &mut [u8],
    alpha: &Alphabet,
    version: Option<u8>,
) -> Result<usize> {
    use sha2::{Digest, Sha256};

    let mut first_hash = Sha256::new();
    if let Some(version) = version {
        first_hash.update(&[version; 1]);
    }
    let first_hash = first_hash.chain(input).finalize();
    let second_hash = Sha256::digest(&first_hash);

    let checksum = &second_hash[0..CHECKSUM_LEN];

    encode_into(
        version.iter().chain(input.iter()).chain(checksum.iter()),
        output,
        alpha,
    )
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BufferTooSmall => write!(
                f,
                "buffer provided to encode base63 string into was too small"
            ),
        }
    }
}
