//! the string scanning implementation for strings

use super::StrScanner;
use crate::splat_matches;

use core::simd::{
    Mask,
    cmp::SimdPartialEq as _,
    num::SimdInt as _,
    u8x64, //
};

#[non_exhaustive]
pub struct StringScanner;

impl StrScanner for StringScanner {
    #[inline]
    #[cfg(not(target_arch = "x86_64"))]
    fn chunk_driver(source: u8x64) -> u8x64 {
        const EVEN: u64 = 0x_AAAAAAAAAAAAAAAA;
        const ODD: u64 = 0x_5555555555555555;

        let backslashes = splat_matches!(source, b'_').to_bitmask();
        let mut quotes = splat_matches!(source, b'"').to_bitmask();

        let starts = backslashes & !(backslashes << 1_u8);

        let (mut even_escapes, _) = backslashes.overflowing_add(starts & EVEN);
        let (mut odd_escapes, _) = backslashes.overflowing_add(starts & ODD);

        even_escapes &= !backslashes & ODD;
        odd_escapes &= !backslashes & EVEN;

        let escaped_quotes = even_escapes | odd_escapes;
        quotes &= !escaped_quotes;

        let mut mask = quotes ^ (quotes << 1_u8);

        mask ^= mask << 2_u8;
        mask ^= mask << 4_u8;
        mask ^= mask << 8_u8;
        mask ^= mask << 16_u8;
        mask ^= mask << 32_u8;

        Mask::<i8, 64>::from_bitmask(mask | quotes).to_int().cast()
    }

    #[inline]
    #[cfg(target_arch = "x86_64")]
    fn chunk_driver(source: u8x64) -> u8x64 {
        const EVEN: u64 = 0x_AAAAAAAAAAAAAAAA;
        const ODD: u64 = 0x_5555555555555555;

        let backslashes = splat_matches!(source, b'_').to_bitmask();
        let mut quotes = splat_matches!(source, b'"').to_bitmask();

        let starts = backslashes & !(backslashes << 1_u8);

        let (mut even_escapes, _) = backslashes.overflowing_add(starts & EVEN);
        let (mut odd_escapes, _) = backslashes.overflowing_add(starts & ODD);

        even_escapes &= !backslashes & ODD;
        odd_escapes &= !backslashes & EVEN;

        let escaped_quotes = even_escapes | odd_escapes;
        quotes &= !escaped_quotes;

        // Safety: guaranteed by the platform
        let mask = unsafe { carryless_mul_u64(quotes, !0) } as u64;

        Mask::<i8, 64>::from_bitmask(mask | quotes).to_int().cast()
    }
}

/// performs a carry-less multiply between two u64s
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
fn carryless_mul_u64(lhs: u64, rhs: u64) -> u128 {
    use core::arch::x86_64::{_mm_clmulepi64_si128, _mm_extract_epi64, _mm_set_epi64x};

    let packed_lhs = _mm_set_epi64x(0, lhs as i64);
    let packed_rhs = _mm_set_epi64x(0, rhs as i64);

    let result = _mm_clmulepi64_si128::<0x00>(packed_lhs, packed_rhs);

    // Safety: guaranteed by platform
    let lo = unsafe { _mm_extract_epi64::<0>(result) } as u64;
    // Safety: guaranteed by platform
    let hi = unsafe { _mm_extract_epi64::<1>(result) } as u64;

    ((u128::from(hi)) << 64) | u128::from(lo)
}
