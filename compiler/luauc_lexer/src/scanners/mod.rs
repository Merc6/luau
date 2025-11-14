//! all of the lexical scanners

mod identifier;
mod string;
// mod whitespace;

use core::simd::{
    Mask,
    num::SimdUint as _,
    u8x64, //
};

pub use self::{
    identifier::IdentifierScanner,
    string::StringScanner,
    // whitespace::WhitespaceScanner, //
};

/// scans a 64-byte string slice
pub trait StrScanner {
    fn chunk_driver(source: u8x64) -> u8x64;

    /// the core of `StrScanner`, `source` is a 64-byte slice of a string
    /// it should return the bit-mask for where all
    #[inline]
    #[must_use]
    fn driver(source: u8x64) -> u64 {
        Mask::from_int(Self::chunk_driver(source).cast::<i8>()).to_bitmask()
    }
}
