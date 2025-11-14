//! the heart of the scanning-pass of the lexer

use crate::scanners::StrScanner;

use core::{
    hint::{
        cold_path,
        likely, //
    },
    simd::u8x64, //
};
use rayon::prelude::*;

#[non_exhaustive]
pub struct Scanner;

impl Scanner {
    /// performs a single-pass scan using the provided scanner
    ///
    /// # Panics
    ///
    /// panics when `source.len()` > `usize::MAX`
    #[inline]
    #[expect(
        clippy::panic,
        reason = "
            should never hit, as it would be trying to allocate too much
            memory, but it doesn't hurt to be pedantic
        "
    )]
    pub fn single_pass_with<S: StrScanner>(source: &mut str) {
        const PARALLEL_THRESHOLD: usize = 1024 * 128; // 128 KB is the best I've found

        let (src, rem) = unsafe { source.as_bytes_mut() }.as_chunks_mut::<64>();
        // add must be checked otherwise UB
        let total_len = src
            .len()
            .checked_add(usize::from(!rem.is_empty()))
            .unwrap_or_else(|| {
                cold_path();
                panic!("`source.len().div_ceil(64)` exceeds what can be held in a `usize`");
            });

        match total_len {
            len if likely(len <= PARALLEL_THRESHOLD) => {
                for chunk in src {
                    S::chunk_driver(u8x64::from_array(*chunk)).copy_to_slice(chunk);
                }
            }
            _ => {
                src.par_iter_mut().for_each(|chunk| {
                    S::chunk_driver(u8x64::from_array(*chunk)).copy_to_slice(chunk);
                });
            }
        }

        if !rem.is_empty() {
            let rem_len = rem.len();
            let src = S::chunk_driver(u8x64::load_or_default(rem)).to_array();
            unsafe { core::ptr::copy_nonoverlapping(src.as_ptr(), rem.as_mut_ptr(), rem_len) };
        }
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;

    use alloc::string::String;
    use core::str::FromStr as _;

    use crate::*;

    #[test]
    fn number_between_next() {
        let mut source = String::from_str("what 65 what").unwrap();
        Scanner::single_pass_with::<scanners::IdentifierScanner>(source.as_mut());

        assert_eq!(source, "what\0\0\0\0what");
    }

    #[test]
    fn leading_number_next() {
        let mut source = String::from_str("65 what what").unwrap();
        Scanner::single_pass_with::<scanners::IdentifierScanner>(source.as_mut());

        assert_eq!(source, "\0\0\0what\0what");
    }

    #[test]
    fn trailing_number_next() {
        let mut source = String::from_str("what what 65").unwrap();
        Scanner::single_pass_with::<scanners::IdentifierScanner>(source.as_mut());

        assert_eq!(source, "what\0what\0\0\0");
    }
}
