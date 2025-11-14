//! the string scanning implementation for identifier scanning

use super::StrScanner;
use crate::splat_matches;

use core::simd::{
    cmp::{
        SimdPartialEq as _,
        SimdPartialOrd as _, //
    },
    u8x64,
};

#[non_exhaustive]
pub struct IdentifierScanner;

impl StrScanner for IdentifierScanner {
    #[inline]
    fn chunk_driver(source: u8x64) -> u8x64 {
        let letters = splat_matches!(source, b'a'..=b'z' | b'A'..=b'Z');
        let underscores = splat_matches!(source, b'_');
        let numbers = splat_matches!(source, b'0'..=b'9');

        let alpha_sep = letters | underscores;

        let body = alpha_sep | numbers;
        let start = alpha_sep;

        let mut identifiers = body.shift_elements_right::<1>(false) & start;
        identifiers |= identifiers.shift_elements_left::<1>(false);

        identifiers.select(source, u8x64::splat(0))
    }
}
