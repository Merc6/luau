//! some epic benches

#![feature(portable_simd)]

use luauc_lexer::{Scanner, scanners};

use divan::{Bencher, counter::BytesCount};
use rand::rngs::SmallRng;
use rand::{Rng as _, SeedableRng as _};

fn main() {
    divan::main();
}

/// generates some pseudo-random input
fn pseudo_random_input(size: usize) -> String {
    let mut rng = SmallRng::seed_from_u64(0x_DEADBEEF);
    let mut buf = Vec::with_capacity(size);
    for _ in 0..size {
        // make something vaguely "source code"-ish
        let b = match rng.random_range(0_u8..100) {
            0..=25 => b'a' + rng.random_range(0..26),
            26..=51 => b'A' + rng.random_range(0..26),
            52..=61 => b'0' + rng.random_range(0..10),
            62 => b'_',
            _ => b' ',
        };
        buf.push(b);
    }
    // Safety: trust me bro ^~^
    unsafe { String::from_utf8_unchecked(buf) }
}

/// benchmark for how fast we can pick identifiers out of source text
#[divan::bench(
    args = [
        1024,
        1024 * 10,
        1024 * 100,
        1024 * 1024,
        1024 * 1024 * 10,
        1024 * 1024 * 128
    ]
)]
#[expect(clippy::single_call_fn, reason = "benchmark")]
fn lex_identifiers_rand_bytes(bencher: Bencher, size: usize) {
    bencher
        .with_inputs(|| pseudo_random_input(size))
        .input_counter(BytesCount::of_str)
        .bench_local_refs(|source| {
            Scanner::single_pass_with::<scanners::IdentifierScanner>(source);
        });
}

// TODO: fix this when everything is ported to new architecture
// #[divan::bench(
//     args = [
//         1024,
//         1024 * 10,
//         1024 * 100,
//         1024 * 1024,
//         1024 * 1024 * 10,
//         1024 * 1024 * 128
//     ]
// )]
// #[expect(clippy::single_call_fn, reason = "benchmark")]
// fn lex_whitespace_rand_bytes(bencher: Bencher, size: usize) {
//     let pattern = make_noisy_input(size);

//     bencher
//         .counter(BytesCount::of_str(&pattern))
//         .bench_local(|| {
//             black_box(Scanner::single_pass_next::<scanners::WhitespaceScanner>(
//                 black_box(&pattern),
//             ))
//         });
// }

/// benchmark for measuring how fast we can pull strings out of a random assortment of bytes
#[divan::bench(
    args = [
        1024,
        1024 * 10,
        1024 * 100,
        1024 * 1024,
        1024 * 1024 * 10,
        1024 * 1024 * 128
    ]
)]
#[expect(clippy::single_call_fn, reason = "benchmark")]
fn lex_strings_rand_bytes(bencher: Bencher, size: usize) {
    bencher
        .with_inputs(|| pseudo_random_input(size))
        .input_counter(BytesCount::of_str)
        .bench_local_refs(|source| {
            Scanner::single_pass_with::<scanners::StringScanner>(source);
        });
}
