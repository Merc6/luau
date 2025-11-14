//! the string scanning implementation for whitespace scanning

use super::StrScanner;
use crate::splat_matches;

use core::simd::{
    cmp::{
        SimdPartialEq as _,
        SimdPartialOrd as _, //
    },
    num::SimdInt,
    u8x64,
};

#[non_exhaustive]
pub struct WhitespaceScanner;

impl WhitespaceScanner {
    #[inline]
    #[must_use = "vector is not modified in place"]
    pub fn horizontal_whitespace(vector: u8x64) -> u64 {
        splat_matches!(vector, b' ' | b'\t').to_bitmask()
    }

    #[inline]
    #[must_use = "vector is not modified in place"]
    pub fn vertical_whitespace(vector: u8x64) -> u64 {
        splat_matches!(vector, 0xA..=0xD | 0x85).to_bitmask()
    }
}

impl StrScanner for WhitespaceScanner {
    #[inline]
    fn chunk_driver(source: u8x64) -> u8x64 {
        splat_matches!(source, b' ' | b'\t' | 0xA..=0xD | 0x85)
            .to_int()
            .cast()
    }
}
