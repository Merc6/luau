//! definition of the `splat_matches` macro

/// uses `matches!` syntax to produce masks by splatting values
#[macro_export]
macro_rules! splat_matches {
    // Base cases
    ( $v:expr, $lit:literal ) => {
        $v.simd_eq(core::simd::Simd::splat($lit))
    };

    ( $v:expr, $start:literal..$end:literal ) => {
        $v.simd_ge(core::simd::Simd::splat($start))
            & $v.simd_lt(core::simd::Simd::splat($end))
    };

    ( $v:expr, $start:literal..=$end:literal ) => {
        $v.simd_ge(core::simd::Simd::splat($start))
            & $v.simd_le(core::simd::Simd::splat($end))
    };


    ( $v:expr, $start:literal..=$end:literal | $($rest:tt)+ ) => {
        $crate::splat_matches!($v, $start..=$end)
            | $crate::splat_matches!($v, $($rest)+)
    };

    ( $v:expr, $start:literal..$end:literal | $($rest:tt)+ ) => {
        $crate::splat_matches!($v, $start..$end)
            | $crate::splat_matches!($v, $($rest)+)
    };

    ( $v:expr, $lit:literal | $($rest:tt)+ ) => {
        $crate::splat_matches!($v, $lit)
            | $crate::splat_matches!($v, $($rest)+)
    };
}
