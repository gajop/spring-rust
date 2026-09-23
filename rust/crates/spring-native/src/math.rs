//! Deterministic `f32` maths for synced native-module code.
//!
//! `f32::sin` and friends call the platform C library, whose results differ
//! between glibc, MSVC and macOS, so synced code using them can desync
//! between players on different systems. These functions come from the
//! pure-Rust `libm` crate instead, which produces the same bits everywhere.
//! (Core-Wasm modules don't need this: their maths is compiled into the
//! module and is identical on every host.)
//!
//! Basic arithmetic and `sqrt` are exact IEEE operations and already agree.

macro_rules! unary {
    ($($(#[$doc:meta])* $name:ident => $libm:ident;)+) => {
        $(
            $(#[$doc])*
            #[inline]
            pub fn $name(x: f32) -> f32 {
                libm::$libm(x)
            }
        )+
    };
}

macro_rules! binary {
    ($($(#[$doc:meta])* $name:ident => $libm:ident;)+) => {
        $(
            $(#[$doc])*
            #[inline]
            pub fn $name(x: f32, y: f32) -> f32 {
                libm::$libm(x, y)
            }
        )+
    };
}

unary! {
    sin => sinf;
    cos => cosf;
    tan => tanf;
    asin => asinf;
    acos => acosf;
    atan => atanf;
    sinh => sinhf;
    cosh => coshf;
    tanh => tanhf;
    exp => expf;
    exp2 => exp2f;
    /// Natural logarithm.
    ln => logf;
    log2 => log2f;
    log10 => log10f;
    sqrt => sqrtf;
    cbrt => cbrtf;
    floor => floorf;
    ceil => ceilf;
    /// Rounds half away from zero, like `f32::round`.
    round => roundf;
    trunc => truncf;
    /// Gauss error function.
    erf => erff;
}

binary! {
    /// `atan2(y, x)`.
    atan2 => atan2f;
    pow => powf;
    hypot => hypotf;
    /// Remainder with the sign of `x`, like `%`.
    fmod => fmodf;
}

/// `(sin(x), cos(x))`.
#[inline]
pub fn sin_cos(x: f32) -> (f32, f32) {
    libm::sincosf(x)
}

#[cfg(test)]
mod tests {
    #[test]
    fn matches_known_values() {
        assert_eq!(super::sin(0.0), 0.0);
        assert_eq!(super::cos(0.0), 1.0);
        assert_eq!(super::atan2(0.0, 1.0), 0.0);
        assert_eq!(super::sqrt(9.0), 3.0);
        assert_eq!(super::hypot(3.0, 4.0), 5.0);
        let (s, c) = super::sin_cos(0.5);
        assert_eq!((s, c), (super::sin(0.5), super::cos(0.5)));
    }
}
