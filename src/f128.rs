// Copyright © 2018–2022 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

//! Constants specific to the [`F128`] quadruple-precision floating-point type.
//!
//! Mathematically significant numbers are provided in the [`consts`] sub-module.
//!
//! For constants related to the floating-point representation itself, see the
//! associated constants defined directly on the [`F128`] type.

use crate::F128;
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    num::FpCategory,
    ops::Neg,
};

const PREC: u32 = 113;
const EXP_BITS: u32 = u128::BITS - PREC;
const EXP_BIAS: u32 = (1 << (EXP_BITS - 1)) - 1;
const SIGN_MASK: u128 = 1 << (u128::BITS - 1);
const EXP_MASK: u128 = ((1 << EXP_BITS) - 1) << (PREC - 1);
const MANT_MASK: u128 = (1 << (PREC - 1)) - 1;

pub(crate) mod private {
    /// The bit representation of a *binary128* floating-point number (`f128`).
    ///
    /// This type can be used to
    ///
    ///   * convert between fixed-point numbers and the bit representation of
    ///     128-bit floating-point numbers.
    ///   * compare fixed-point numbers and the bit representation of 128-bit
    ///     floating-point numbers.
    ///
    /// Please see [<i>Quadruple-precision floating-point format</i> on
    /// Wikipedia][quad] for more information on *binary128*.
    ///
    /// *See also the <code>[fixed]::[f128]::[consts]</code> module.*
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::{types::I16F16, F128};
    /// assert_eq!(I16F16::ONE.to_num::<F128>(), F128::ONE);
    /// assert_eq!(I16F16::from_num(F128::ONE), I16F16::ONE);
    ///
    /// // fixed-point numbers can be compared directly to F128 values
    /// assert!(I16F16::from_num(1.5) > F128::ONE);
    /// assert!(I16F16::from_num(0.5) < F128::ONE);
    /// ```
    ///
    /// [consts]: crate::f128::consts
    /// [f128]: crate::f128
    /// [fixed]: crate
    /// [quad]: https://en.wikipedia.org/wiki/Quadruple-precision_floating-point_format
    #[derive(Clone, Copy, Default, Debug)]
    pub struct F128 {
        pub(crate) bits: u128,
    }
}

impl F128 {
    /// Zero.
    pub const ZERO: F128 = F128::from_bits(0);
    /// Negative zero (&minus;0).
    pub const NEG_ZERO: F128 = F128::from_bits(SIGN_MASK);
    /// One.
    pub const ONE: F128 = F128::from_bits((EXP_BIAS as u128) << (PREC - 1));
    /// Negative one (&minus;1).
    pub const NEG_ONE: F128 = F128::from_bits(SIGN_MASK | F128::ONE.to_bits());

    /// Smallest positive subnormal number.
    ///
    /// Equal to 2<sup>[`MIN_EXP`]&nbsp;&minus;&nbsp;[`MANTISSA_DIGITS`]</sup>.
    ///
    /// [`MANTISSA_DIGITS`]: Self::MANTISSA_DIGITS
    /// [`MIN_EXP`]: Self::MIN_EXP
    pub const MIN_POSITIVE_SUB: F128 = F128::from_bits(1);

    /// Smallest positive normal number.
    ///
    /// Equal to 2<sup>[`MIN_EXP`]&nbsp;&minus;&nbsp;1</sup>.
    ///
    /// [`MIN_EXP`]: Self::MIN_EXP
    pub const MIN_POSITIVE: F128 = F128::from_bits(MANT_MASK + 1);

    /// Largest finite number.
    ///
    /// Equal to
    /// (1&nbsp;&minus;&nbsp;2<sup>&minus;[`MANTISSA_DIGITS`]</sup>)&nbsp;2<sup>[`MAX_EXP`]</sup>.
    ///
    /// [`MANTISSA_DIGITS`]: Self::MANTISSA_DIGITS
    /// [`MAX_EXP`]: Self::MAX_EXP
    pub const MAX: F128 = F128::from_bits(EXP_MASK - 1);

    /// Smallest finite number (&minus;[`MAX`]).
    ///
    /// [`MAX`]: Self::MAX
    pub const MIN: F128 = F128::from_bits(SIGN_MASK | F128::MAX.to_bits());

    /// Infinity (∞).
    pub const INFINITY: F128 = F128::from_bits(EXP_MASK);

    /// Negative infinity (&minus;∞).
    pub const NEG_INFINITY: F128 = F128::from_bits(SIGN_MASK | EXP_MASK);

    /// NaN.
    pub const NAN: F128 = F128::from_bits(EXP_MASK | (1u128 << (PREC - 2)));

    /// The radix or base of the internal representation (2).
    pub const RADIX: u32 = 2;

    /// Number of significant digits in base 2.
    pub const MANTISSA_DIGITS: u32 = PREC;

    /// Maximum <i>x</i> such that any decimal number with <i>x</i> significant
    /// digits can be converted to [`F128`] and back without loss.
    ///
    /// Equal to
    /// floor(log<sub>10</sub>&nbsp;2<sup>[`MANTISSA_DIGITS`]&nbsp;&minus;&nbsp;1</sup>).
    ///
    /// [`MANTISSA_DIGITS`]: Self::MANTISSA_DIGITS
    pub const DIGITS: u32 = 33;

    /// The difference between 1 and the next larger representable number.
    ///
    /// Equal to 2<sup>1&nbsp;&minus;&nbsp;[`MANTISSA_DIGITS`]</sup>.
    ///
    /// [`MANTISSA_DIGITS`]: Self::MANTISSA_DIGITS
    pub const EPSILON: F128 = F128::from_bits(((EXP_BIAS - (PREC - 1)) as u128) << (PREC - 1));

    /// If <i>x</i>&nbsp;=&nbsp;`MIN_EXP`, then normal numbers
    /// ≥&nbsp;0.5&nbsp;×&nbsp;2<sup><i>x</i></sup>.
    pub const MIN_EXP: i32 = 3 - F128::MAX_EXP;

    /// If <i>x</i>&nbsp;=&nbsp;`MAX_EXP`, then normal numbers
    /// <&nbsp;1&nbsp;×&nbsp;2<sup><i>x</i></sup>.
    pub const MAX_EXP: i32 = EXP_BIAS as i32 + 1;

    /// Minimum <i>x</i> for which 10<sup><i>x</i></sup> is in the normal range
    /// of [`F128`].
    ///
    /// Equal to ceil(log<sub>10</sub>&nbsp;[`MIN_POSITIVE`]).
    ///
    /// [`MIN_POSITIVE`]: Self::MIN_POSITIVE
    pub const MIN_10_EXP: i32 = -4931;

    /// Maximum <i>x</i> for which 10<sup><i>x</i></sup> is in the normal range
    /// of [`F128`].
    ///
    /// Equal to floor(log<sub>10</sub>&nbsp;[`MAX`]).
    ///
    /// [`MAX`]: Self::MAX
    pub const MAX_10_EXP: i32 = 4932;

    /// Raw transmutation from [`u128`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::F128;
    /// let infinity_bits = 0x7FFF_u128 << 112;
    /// assert!(F128::from_bits(infinity_bits - 1).is_finite());
    /// assert!(!F128::from_bits(infinity_bits).is_finite());
    /// ```
    #[inline]
    pub const fn from_bits(bits: u128) -> F128 {
        F128 { bits }
    }

    /// Raw transmutation to [`u128`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::F128;
    /// assert_eq!(F128::ONE.to_bits(), 0x3FFF_u128 << 112);
    /// assert_ne!(F128::ONE.to_bits(), 1u128);
    /// ```
    #[inline]
    pub const fn to_bits(self) -> u128 {
        self.bits
    }

    /// Creates a number from a byte array in big-endian byte order.
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; 16]) -> F128 {
        F128::from_bits(u128::from_be_bytes(bytes))
    }

    /// Creates a number from a byte array in little-endian byte order.
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; 16]) -> F128 {
        F128::from_bits(u128::from_le_bytes(bytes))
    }

    /// Creates a number from a byte array in native-endian byte order.
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; 16]) -> F128 {
        F128::from_bits(u128::from_ne_bytes(bytes))
    }

    /// Returns the memory representation of the number as a byte array in
    /// big-endian byte order.
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.to_bits().to_be_bytes()
    }

    /// Returns the memory representation of the number as a byte array in
    /// little-endian byte order.
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 16] {
        self.to_bits().to_le_bytes()
    }

    /// Returns the memory representation of the number as a byte array in
    /// native-endian byte order.
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; 16] {
        self.to_bits().to_ne_bytes()
    }

    /// Returns [`true`] if the number is NaN.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::NAN.is_nan());
    ///
    /// assert!(!F128::ONE.is_nan());
    /// assert!(!F128::INFINITY.is_nan());
    /// assert!(!F128::NEG_INFINITY.is_nan());
    /// ```
    #[inline]
    pub const fn is_nan(self) -> bool {
        (self.to_bits() & !SIGN_MASK) > EXP_MASK
    }

    /// Returns [`true`] if the number is infinite.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::INFINITY.is_infinite());
    /// assert!(F128::NEG_INFINITY.is_infinite());
    ///
    /// assert!(!F128::ONE.is_infinite());
    /// assert!(!F128::NAN.is_infinite());
    /// ```
    #[inline]
    pub const fn is_infinite(self) -> bool {
        (self.to_bits() & !SIGN_MASK) == EXP_MASK
    }

    /// Returns [`true`] if the number is neither infinite nor NaN.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::ONE.is_finite());
    /// assert!(F128::MAX.is_finite());
    ///
    /// assert!(!F128::INFINITY.is_finite());
    /// assert!(!F128::NEG_INFINITY.is_finite());
    /// assert!(!F128::NAN.is_finite());
    /// ```
    #[inline]
    pub const fn is_finite(self) -> bool {
        (self.to_bits() & EXP_MASK) != EXP_MASK
    }

    /// Returns [`true`] if the number is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::ZERO.is_zero());
    /// assert!(F128::NEG_ZERO.is_zero());
    ///
    /// assert!(!F128::MIN_POSITIVE_SUB.is_zero());
    /// assert!(!F128::NAN.is_zero());
    /// ```
    #[inline]
    pub const fn is_zero(self) -> bool {
        (self.to_bits() & !SIGN_MASK) == 0
    }

    /// Returns [`true`] if the number is subnormal.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::MIN_POSITIVE_SUB.is_subnormal());
    ///
    /// assert!(!F128::ZERO.is_subnormal());
    /// assert!(!F128::MIN_POSITIVE.is_subnormal());
    /// ```
    #[inline]
    pub const fn is_subnormal(self) -> bool {
        let abs = self.to_bits() & !SIGN_MASK;
        0 < abs && abs < F128::MIN_POSITIVE.to_bits()
    }

    /// Returns [`true`] if the number is neither zero, infinite, subnormal, or NaN.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::MIN.is_normal());
    /// assert!(F128::MIN_POSITIVE.is_normal());
    /// assert!(F128::MAX.is_normal());
    ///
    /// assert!(!F128::ZERO.is_normal());
    /// assert!(!F128::MIN_POSITIVE_SUB.is_normal());
    /// assert!(!F128::INFINITY.is_normal());
    /// assert!(!F128::NAN.is_normal());
    /// ```
    #[inline]
    pub const fn is_normal(self) -> bool {
        let abs = self.to_bits() & !SIGN_MASK;
        F128::MIN_POSITIVE.to_bits() <= abs && abs <= F128::MAX.to_bits()
    }

    /// Returns the floating point category of the number.
    ///
    /// If only one property is going to be tested, it is generally faster to
    /// use the specific predicate instead.
    ///
    /// # Example
    ///
    /// ```rust
    /// use core::num::FpCategory;
    /// use fixed::F128;
    ///
    /// assert_eq!(F128::ZERO.classify(), FpCategory::Zero);
    /// assert_eq!(F128::MIN_POSITIVE_SUB.classify(), FpCategory::Subnormal);
    /// assert_eq!(F128::MIN_POSITIVE.classify(), FpCategory::Normal);
    /// assert_eq!(F128::INFINITY.classify(), FpCategory::Infinite);
    /// assert_eq!(F128::NAN.classify(), FpCategory::Nan);
    /// ```
    #[inline]
    pub const fn classify(self) -> FpCategory {
        let exp = self.to_bits() & EXP_MASK;
        let mant = self.to_bits() & MANT_MASK;
        if exp == 0 {
            if mant == 0 {
                FpCategory::Zero
            } else {
                FpCategory::Subnormal
            }
        } else if exp == EXP_MASK {
            if mant == 0 {
                FpCategory::Infinite
            } else {
                FpCategory::Nan
            }
        } else {
            FpCategory::Normal
        }
    }

    /// Returns the absolute value of the number.
    ///
    /// The only difference possible between the input value and the returned
    /// value is in the sign bit, which is always cleared in the return value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// // -0 == +0, but -0 bits != +0 bits
    /// assert_eq!(F128::NEG_ZERO, F128::ZERO);
    /// assert_ne!(F128::NEG_ZERO.to_bits(), F128::ZERO.to_bits());
    /// assert_eq!(F128::NEG_ZERO.abs().to_bits(), F128::ZERO.to_bits());
    ///
    /// assert_eq!(F128::NEG_INFINITY.abs(), F128::INFINITY);
    /// assert_eq!(F128::MIN.abs(), F128::MAX);
    ///
    /// assert!(F128::NAN.abs().is_nan());
    /// ```
    #[inline]
    pub const fn abs(self) -> F128 {
        F128::from_bits(self.to_bits() & !SIGN_MASK)
    }

    /// Returns a number that represents the sign of the input value.
    ///
    ///   * 1 if the number is positive, +0, or +∞
    ///   * &minus;1 if the number is negative, &minus;0, or &minus;∞
    ///   * NaN if the number is NaN
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert_eq!(F128::ONE.signum(), F128::ONE);
    /// assert_eq!(F128::INFINITY.signum(), F128::ONE);
    /// assert_eq!(F128::NEG_ZERO.signum(), F128::NEG_ONE);
    /// assert_eq!(F128::MIN.signum(), F128::NEG_ONE);
    ///
    /// assert!(F128::NAN.signum().is_nan());
    /// ```
    #[inline]
    pub const fn signum(self) -> F128 {
        if self.is_nan() {
            self
        } else if self.is_sign_positive() {
            F128::ONE
        } else {
            F128::NEG_ONE
        }
    }

    /// Returns a number composed of the magnitude of `self` and the sign of `sign`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert_eq!(F128::ONE.copysign(F128::NEG_ZERO), F128::NEG_ONE);
    /// assert_eq!(F128::ONE.copysign(F128::ZERO), F128::ONE);
    /// assert_eq!(F128::NEG_ONE.copysign(F128::NEG_INFINITY), F128::NEG_ONE);
    /// assert_eq!(F128::NEG_ONE.copysign(F128::INFINITY), F128::ONE);
    ///
    /// assert!(F128::NAN.copysign(F128::ONE).is_nan());
    /// assert!(F128::NAN.copysign(F128::ONE).is_sign_positive());
    /// assert!(F128::NAN.copysign(F128::NEG_ONE).is_sign_negative());
    /// ```
    #[inline]
    pub const fn copysign(self, sign: F128) -> F128 {
        F128::from_bits((self.to_bits() & !SIGN_MASK) | (sign.to_bits() & SIGN_MASK))
    }

    /// Returns [`true`] if the number has a positive sign, including +0, +∞,
    /// and NaN without a negative sign bit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::ZERO.is_sign_positive());
    /// assert!(F128::MAX.is_sign_positive());
    /// assert!(F128::INFINITY.is_sign_positive());
    ///
    /// assert!(!F128::NEG_ZERO.is_sign_positive());
    /// assert!(!F128::MIN.is_sign_positive());
    /// assert!(!F128::NEG_INFINITY.is_sign_positive());
    /// ```
    #[inline]
    pub const fn is_sign_positive(self) -> bool {
        (self.to_bits() & SIGN_MASK) == 0
    }

    /// Returns [`true`] if the number has a negative sign, including &minus;0,
    /// &minus;∞, and NaN with a negative sign bit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fixed::F128;
    ///
    /// assert!(F128::NEG_ZERO.is_sign_negative());
    /// assert!(F128::MIN.is_sign_negative());
    /// assert!(F128::NEG_INFINITY.is_sign_negative());
    ///
    /// assert!(!F128::ZERO.is_sign_negative());
    /// assert!(!F128::MAX.is_sign_negative());
    /// assert!(!F128::INFINITY.is_sign_negative());
    /// ```
    #[inline]
    pub const fn is_sign_negative(self) -> bool {
        (self.to_bits() & SIGN_MASK) != 0
    }

    /// Returns the ordering between `self` and `other`.
    ///
    /// Unlike the [`PartialOrd`] implementation, this method always returns an
    /// order in the following sequence:
    ///
    ///   * NaN with the sign bit set
    ///   * &minus;∞
    ///   * negative normal numbers
    ///   * negative subnormal numbers
    ///   * &minus;0
    ///   * +0
    ///   * positive subnormal numbers
    ///   * positive normal numbers
    ///   * +∞
    ///   * NaN with the sign bit cleared
    ///
    /// # Example
    ///
    /// ```rust
    /// use core::cmp::Ordering;
    /// use fixed::F128;
    ///
    /// let neg_nan = F128::NAN.copysign(F128::NEG_ONE);
    /// let pos_nan = F128::NAN.copysign(F128::ONE);
    /// let neg_inf = F128::NEG_INFINITY;
    /// let pos_inf = F128::INFINITY;
    /// let neg_zero = F128::NEG_ZERO;
    /// let pos_zero = F128::ZERO;
    ///
    /// assert_eq!(neg_nan.total_cmp(&neg_inf), Ordering::Less);
    /// assert_eq!(pos_nan.total_cmp(&pos_inf), Ordering::Greater);
    /// assert_eq!(neg_zero.total_cmp(&pos_zero), Ordering::Less);
    /// ```
    #[inline]
    pub const fn total_cmp(&self, other: &F128) -> Ordering {
        let a = self.to_bits();
        let b = other.to_bits();
        match (self.is_sign_negative(), other.is_sign_negative()) {
            (false, false) => cmp_bits(a, b),
            (true, true) => cmp_bits(b, a),
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
        }
    }
}

const fn cmp_bits(a: u128, b: u128) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

impl PartialEq for F128 {
    #[inline]
    fn eq(&self, other: &F128) -> bool {
        if self.is_nan() || other.is_nan() {
            return false;
        }
        let a = self.to_bits();
        let b = other.to_bits();
        // handle zero
        if ((a | b) & !SIGN_MASK) == 0 {
            return true;
        }
        a == b
    }
}

impl PartialOrd for F128 {
    #[inline]
    fn partial_cmp(&self, other: &F128) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            return None;
        }
        let a = self.to_bits();
        let b = other.to_bits();
        // handle zero
        if ((a | b) & !SIGN_MASK) == 0 {
            return Some(Ordering::Equal);
        }
        match (self.is_sign_negative(), other.is_sign_negative()) {
            (false, false) => a.partial_cmp(&b),
            (true, true) => b.partial_cmp(&a),
            (false, true) => Some(Ordering::Greater),
            (true, false) => Some(Ordering::Less),
        }
    }
}

impl Hash for F128 {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut bits = self.to_bits();
        if bits == F128::NEG_ZERO.to_bits() {
            bits = 0;
        }
        bits.hash(state);
    }
}

impl Neg for F128 {
    type Output = F128;
    #[inline]
    fn neg(self) -> F128 {
        F128::from_bits(self.to_bits() ^ SIGN_MASK)
    }
}

/*
```rust
use core::{cmp::Ord, convert::TryFrom};
use rug::{
    float::{Constant, Round},
    Assign, Float, Integer,
};

fn decimal_string(val: &Float, prec: i32) -> String {
    let log10 = val.clone().log10();
    let floor_log10 = log10.to_i32_saturating_round(Round::Down).unwrap();
    let shift = u32::try_from(prec - 1 - floor_log10).unwrap();
    let val = val.clone() * Integer::from(Integer::u_pow_u(10, shift));
    let int = val.to_integer_round(Round::Down).unwrap().0;
    let padding = "0".repeat(usize::try_from(-floor_log10.min(0)).unwrap());
    let mut s = format!("{}{}", padding, int);
    s.insert(1, '.');
    s
}

fn hex_bits(bits: u128) -> String {
    let mut s = format!("0x{:016X}", bits);
    for i in 0..7 {
        s.insert(6 + 5 * i, '_');
    }
    s
}

fn print(doc: &str, name: &str, val: Float) {
    println!();
    println!("    /// {} = {}…", doc, decimal_string(&val, 6));
    println!("    // {} = {}...", name, decimal_string(&val, 40));
    let round = Float::with_val(113, &val);

    let sign_bit = if round.is_sign_negative() {
        1u128 << 127
    } else {
        0
    };

    let unbiased_exp = round.get_exp().unwrap();
    assert!(-16_381 <= unbiased_exp && unbiased_exp <= 16_384);
    let exp_bits = u128::from((unbiased_exp + 16_382).unsigned_abs()) << 112;

    let unshifted_mant = round.get_significand().unwrap();
    let mant = unshifted_mant.clone() >> (unshifted_mant.significant_bits() - 113);
    let mant_128 = mant.to_u128_wrapping();
    assert_eq!(mant_128 >> 112, 1);
    let mant_bits = mant_128 & ((1 << 112) - 1);

    println!(
        "    pub const {name}: F128 = F128::from_bits({});",
        hex_bits(sign_bit | exp_bits | mant_bits)
    );
}

fn float<T>(t: T) -> Float
where
    Float: Assign<T>,
{
    Float::with_val(1000, t)
}

fn main() {
    println!("/// Basic mathematical constants.");
    println!("pub mod consts {{");
    println!("    use crate::F128;");
    print("Archimedes’ constant, π", "PI", float(Constant::Pi));
    print("A turn, τ", "TAU", float(Constant::Pi) * 2);
    print("π/2", "FRAC_PI_2", float(Constant::Pi) / 2);
    print("π/3", "FRAC_PI_3", float(Constant::Pi) / 3);
    print("π/4", "FRAC_PI_4", float(Constant::Pi) / 4);
    print("π/6", "FRAC_PI_6", float(Constant::Pi) / 6);
    print("π/8", "FRAC_PI_8", float(Constant::Pi) / 8);
    print("1/π", "FRAC_1_PI", 1 / float(Constant::Pi));
    print("2/π", "FRAC_2_PI", 2 / float(Constant::Pi));
    print("2/√π", "FRAC_2_SQRT_PI", 2 / float(Constant::Pi).sqrt());
    print("√2", "SQRT_2", float(2).sqrt());
    print("1/√2", "FRAC_1_SQRT_2", float(0.5).sqrt());
    print("Euler’s number, e", "E", float(1).exp());
    print("log<sub>2</sub> 10", "LOG2_10", float(10).log2());
    print("log<sub>2</sub> e", "LOG2_E", float(1).exp().log2());
    print("log<sub>10</sub> 2", "LOG10_2", float(2).log10());
    print("log<sub>10</sub> e", "LOG10_E", float(1).exp().log10());
    print("ln 2", "LN_2", float(2).ln());
    print("ln 10", "LN_10", float(10).ln());
    println!("}}");
}
```
*/

/// Basic mathematical constants.
pub mod consts {
    use crate::F128;

    /// Archimedes’ constant, π = 3.14159…
    // PI = 3.141592653589793238462643383279502884197...
    pub const PI: F128 = F128::from_bits(0x4000_921F_B544_42D1_8469_898C_C517_01B8);

    /// A turn, τ = 6.28318…
    // TAU = 6.283185307179586476925286766559005768394...
    pub const TAU: F128 = F128::from_bits(0x4001_921F_B544_42D1_8469_898C_C517_01B8);

    /// π/2 = 1.57079…
    // FRAC_PI_2 = 1.570796326794896619231321691639751442098...
    pub const FRAC_PI_2: F128 = F128::from_bits(0x3FFF_921F_B544_42D1_8469_898C_C517_01B8);

    /// π/3 = 1.04719…
    // FRAC_PI_3 = 1.047197551196597746154214461093167628065...
    pub const FRAC_PI_3: F128 = F128::from_bits(0x3FFF_0C15_2382_D736_5846_5BB3_2E0F_567B);

    /// π/4 = 0.785398…
    // FRAC_PI_4 = 0.7853981633974483096156608458198757210492...
    pub const FRAC_PI_4: F128 = F128::from_bits(0x3FFE_921F_B544_42D1_8469_898C_C517_01B8);

    /// π/6 = 0.523598…
    // FRAC_PI_6 = 0.5235987755982988730771072305465838140328...
    pub const FRAC_PI_6: F128 = F128::from_bits(0x3FFE_0C15_2382_D736_5846_5BB3_2E0F_567B);

    /// π/8 = 0.392699…
    // FRAC_PI_8 = 0.3926990816987241548078304229099378605246...
    pub const FRAC_PI_8: F128 = F128::from_bits(0x3FFD_921F_B544_42D1_8469_898C_C517_01B8);

    /// 1/π = 0.318309…
    // FRAC_1_PI = 0.3183098861837906715377675267450287240689...
    pub const FRAC_1_PI: F128 = F128::from_bits(0x3FFD_45F3_06DC_9C88_2A53_F84E_AFA3_EA6A);

    /// 2/π = 0.636619…
    // FRAC_2_PI = 0.6366197723675813430755350534900574481378...
    pub const FRAC_2_PI: F128 = F128::from_bits(0x3FFE_45F3_06DC_9C88_2A53_F84E_AFA3_EA6A);

    /// 2/√π = 1.12837…
    // FRAC_2_SQRT_PI = 1.128379167095512573896158903121545171688...
    pub const FRAC_2_SQRT_PI: F128 = F128::from_bits(0x3FFF_20DD_7504_29B6_D11A_E3A9_14FE_D7FE);

    /// √2 = 1.41421…
    // SQRT_2 = 1.414213562373095048801688724209698078569...
    pub const SQRT_2: F128 = F128::from_bits(0x3FFF_6A09_E667_F3BC_C908_B2FB_1366_EA95);

    /// 1/√2 = 0.707106…
    // FRAC_1_SQRT_2 = 0.7071067811865475244008443621048490392848...
    pub const FRAC_1_SQRT_2: F128 = F128::from_bits(0x3FFE_6A09_E667_F3BC_C908_B2FB_1366_EA95);

    /// Euler’s number, e = 2.71828…
    // E = 2.718281828459045235360287471352662497757...
    pub const E: F128 = F128::from_bits(0x4000_5BF0_A8B1_4576_9535_5FB8_AC40_4E7A);

    /// log<sub>2</sub> 10 = 3.32192…
    // LOG2_10 = 3.321928094887362347870319429489390175864...
    pub const LOG2_10: F128 = F128::from_bits(0x4000_A934_F097_9A37_15FC_9257_EDFE_9B60);

    /// log<sub>2</sub> e = 1.44269…
    // LOG2_E = 1.442695040888963407359924681001892137426...
    pub const LOG2_E: F128 = F128::from_bits(0x3FFF_7154_7652_B82F_E177_7D0F_FDA0_D23A);

    /// log<sub>10</sub> 2 = 0.301029…
    // LOG10_2 = 0.3010299956639811952137388947244930267681...
    pub const LOG10_2: F128 = F128::from_bits(0x3FFD_3441_3509_F79F_EF31_1F12_B358_16F9);

    /// log<sub>10</sub> e = 0.434294…
    // LOG10_E = 0.4342944819032518276511289189166050822943...
    pub const LOG10_E: F128 = F128::from_bits(0x3FFD_BCB7_B152_6E50_E32A_6AB7_555F_5A68);

    /// ln 2 = 0.693147…
    // LN_2 = 0.6931471805599453094172321214581765680755...
    pub const LN_2: F128 = F128::from_bits(0x3FFE_62E4_2FEF_A39E_F357_93C7_6730_07E6);

    /// ln 10 = 2.30258…
    // LN_10 = 2.302585092994045684017991454684364207601...
    pub const LN_10: F128 = F128::from_bits(0x4000_26BB_1BBB_5551_582D_D4AD_AC57_05A6);
}

#[cfg(test)]
mod tests {
    use crate::{traits::FromFixed, F128};
    use half::{bf16, f16};

    // Apart from F128 include f16, bf16, f32, f64 as a sanity check for the tests.

    struct Params {
        mantissa_digits: u32,
        min_exp: i32,
        max_exp: i32,
        digits: u32,
        min_10_exp: i32,
        max_10_exp: i32,
    }

    impl Params {
        #[track_caller]
        fn check(self) {
            let p = f64::from(self.mantissa_digits);
            let e_min = f64::from(self.min_exp);
            let e_max = f64::from(self.max_exp);
            assert_eq!(self.digits, ((p - 1.) * 2f64.log10()).floor() as u32);
            assert_eq!(self.min_10_exp, ((e_min - 1.) * 2f64.log10()).ceil() as i32);
            assert_eq!(
                self.max_10_exp,
                ((-(-p).exp2()).ln_1p() / 10f64.ln() + e_max * 2f64.log10()).floor() as i32
            );
        }
    }

    #[test]
    fn decimal_constants_f16() {
        let params = Params {
            mantissa_digits: f16::MANTISSA_DIGITS,
            min_exp: f16::MIN_EXP,
            max_exp: f16::MAX_EXP,
            digits: f16::DIGITS,
            min_10_exp: f16::MIN_10_EXP,
            max_10_exp: f16::MAX_10_EXP,
        };
        params.check();
    }

    #[test]
    fn decimal_constants_bf16() {
        let params = Params {
            mantissa_digits: bf16::MANTISSA_DIGITS,
            min_exp: bf16::MIN_EXP,
            max_exp: bf16::MAX_EXP,
            digits: bf16::DIGITS,
            min_10_exp: bf16::MIN_10_EXP,
            max_10_exp: bf16::MAX_10_EXP,
        };
        params.check();
    }

    #[test]
    fn decimal_constants_f32() {
        let params = Params {
            mantissa_digits: f32::MANTISSA_DIGITS,
            min_exp: f32::MIN_EXP,
            max_exp: f32::MAX_EXP,
            digits: f32::DIGITS,
            min_10_exp: f32::MIN_10_EXP,
            max_10_exp: f32::MAX_10_EXP,
        };
        params.check();
    }

    #[test]
    fn decimal_constants_f64() {
        let params = Params {
            mantissa_digits: f64::MANTISSA_DIGITS,
            min_exp: f64::MIN_EXP,
            max_exp: f64::MAX_EXP,
            digits: f64::DIGITS,
            min_10_exp: f64::MIN_10_EXP,
            max_10_exp: f64::MAX_10_EXP,
        };
        params.check();
    }

    #[test]
    fn decimal_constants_f128() {
        let params = Params {
            mantissa_digits: F128::MANTISSA_DIGITS,
            min_exp: F128::MIN_EXP,
            max_exp: F128::MAX_EXP,
            digits: F128::DIGITS,
            min_10_exp: F128::MIN_10_EXP,
            max_10_exp: F128::MAX_10_EXP,
        };
        params.check();
    }

    #[test]
    fn math_constants() {
        use crate::{consts as fix, f128::consts as f128};
        assert_eq!(f128::PI, F128::from_fixed(fix::PI));
        assert_eq!(f128::TAU, F128::from_fixed(fix::TAU));
        assert_eq!(f128::FRAC_PI_2, F128::from_fixed(fix::FRAC_PI_2));
        assert_eq!(f128::FRAC_PI_3, F128::from_fixed(fix::FRAC_PI_3));
        assert_eq!(f128::FRAC_PI_4, F128::from_fixed(fix::FRAC_PI_4));
        assert_eq!(f128::FRAC_PI_6, F128::from_fixed(fix::FRAC_PI_6));
        assert_eq!(f128::FRAC_PI_8, F128::from_fixed(fix::FRAC_PI_8));
        assert_eq!(f128::FRAC_1_PI, F128::from_fixed(fix::FRAC_1_PI));
        assert_eq!(f128::FRAC_2_PI, F128::from_fixed(fix::FRAC_2_PI));
        assert_eq!(f128::FRAC_2_SQRT_PI, F128::from_fixed(fix::FRAC_2_SQRT_PI));
        assert_eq!(f128::SQRT_2, F128::from_fixed(fix::SQRT_2));
        assert_eq!(f128::FRAC_1_SQRT_2, F128::from_fixed(fix::FRAC_1_SQRT_2));
        assert_eq!(f128::E, F128::from_fixed(fix::E));
        assert_eq!(f128::LOG2_10, F128::from_fixed(fix::LOG2_10));
        assert_eq!(f128::LOG2_E, F128::from_fixed(fix::LOG2_E));
        assert_eq!(f128::LOG10_2, F128::from_fixed(fix::LOG10_2));
        assert_eq!(f128::LOG10_E, F128::from_fixed(fix::LOG10_E));
        assert_eq!(f128::LN_2, F128::from_fixed(fix::LN_2));
        assert_eq!(f128::LN_10, F128::from_fixed(fix::LN_10));
    }
}
