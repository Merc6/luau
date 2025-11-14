//! An experimental lexer for the luau language

#![no_std]
#![feature(portable_simd, cold_path, likely_unlikely)]

mod macros;
mod scanner;
mod token_type;

pub mod scanners;

pub use scanner::Scanner;
pub use token_type::TokenType;
