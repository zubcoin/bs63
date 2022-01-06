//! Support for configurable alphabets

use core::fmt;

/// Prepared Alphabet for
/// [`EncodeBuilder::with_alphabet`](crate::encode::EncodeBuilder::with_alphabet) and
/// [`DecodeBuilder::with_alphabet`](crate::decode::DecodeBuilder::with_alphabet).
#[derive(Clone, Copy)]
pub struct Alphabet {
    pub(crate) encode: [u8; 63],
    pub(crate) decode: [u8; 128],
}

/// Errors that could occur when preparing a Base63 alphabet.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The alphabet contained a duplicate character at at least 2 indexes.
    DuplicateCharacter {
        /// The duplicate character encountered.
        character: char,
        /// The first index the character was seen at.
        first: usize,
        /// The second index the character was seen at.
        second: usize,
    },

    /// The alphabet contained a multi-byte (or non-utf8) character.
    NonAsciiCharacter {
        /// The index at which the non-ASCII character was seen.
        index: usize,
    },
}

impl Alphabet {
    /// DEFAULT Base63, as used by ZubCoin
    pub const ZUBCOIN: &'static Self =
        &Self::new_unwrap(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz");

    /// The default alphabet used if none is given. Currently is the
    /// [`ZUBCOIN`](Self::ZUBCOIN) alphabet.
    pub const DEFAULT: &'static Self = Self::ZUBCOIN;

    /// A new Alphabet.
    pub const fn new(base: &[u8; 63]) -> Result<Self, Error> {
        let mut encode = [0x00; 63];
        let mut decode = [0xFF; 128];

        let mut i = 0;
        while i < encode.len() {
            if base[i] >= 128 {
                return Err(Error::NonAsciiCharacter { index: i });
            }
            if decode[base[i] as usize] != 0xFF {
                return Err(Error::DuplicateCharacter {
                    character: base[i] as char,
                    first: decode[base[i] as usize] as usize,
                    second: i,
                });
            }
            encode[i] = base[i];
            decode[base[i] as usize] = i as u8;
            i += 1;
        }

        Ok(Self { encode, decode })
    }

    /// Same as [`Self::new`], but gives a panic instead of an [`Err`] on bad input.
    ///
    /// Intended to support usage in `const` context until [`Result::unwrap`] is able to be called.
    pub const fn new_unwrap(base: &[u8; 63]) -> Self {
        let result = Self::new(base);
        #[allow(unconditional_panic)] // https://github.com/rust-lang/rust/issues/78803
        [][match result {
            Ok(alphabet) => return alphabet,
            Err(_) => 0,
        }]
    }
}

impl fmt::Debug for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(s) = core::str::from_utf8(&self.encode) {
            f.debug_tuple("Alphabet").field(&s).finish()
        } else {
            unreachable!()
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DuplicateCharacter {
                character,
                first,
                second,
            } => write!(
                f,
                "alphabet contained a duplicate character `{}` at indexes {} and {}",
                character, first, second,
            ),
            Error::NonAsciiCharacter { index } => {
                write!(f, "alphabet contained a non-ascii character at {}", index)
            }
        }
    }
}

// Force evaluation of the associated constants to make sure they don't error
const _: () = {
    let _ = Alphabet::DEFAULT;
};

#[test]
#[should_panic]
fn test_new_unwrap_does_panic() {
    Alphabet::new_unwrap(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
}
