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

use crate::types::extra::Unsigned;
use crate::{
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
    FixedU8,
};
use core::cmp::Ordering;

macro_rules! diff_sign {
    ($Sig:ident, $Uns:ident($UnsInner:ident)) => {
        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialEq<$Sig<FracRhs>> for $Uns<FracLhs> {
            #[inline]
            fn eq(&self, rhs: &$Sig<FracRhs>) -> bool {
                if rhs.is_negative() {
                    return false;
                }
                let unsigned_rhs = $Uns::<FracRhs>::from_bits(rhs.to_bits() as $UnsInner);
                PartialEq::eq(self, &unsigned_rhs)
            }
        }

        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialEq<$Uns<FracRhs>> for $Sig<FracLhs> {
            #[inline]
            fn eq(&self, rhs: &$Uns<FracRhs>) -> bool {
                if self.is_negative() {
                    return false;
                }
                let unsigned_lhs = $Uns::<FracLhs>::from_bits(self.to_bits() as $UnsInner);
                PartialEq::eq(&unsigned_lhs, rhs)
            }
        }

        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialOrd<$Sig<FracRhs>> for $Uns<FracLhs> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Sig<FracRhs>) -> Option<Ordering> {
                if rhs.is_negative() {
                    return Some(Ordering::Greater);
                }
                let unsigned_rhs = $Uns::<FracRhs>::from_bits(rhs.to_bits() as $UnsInner);
                PartialOrd::partial_cmp(self, &unsigned_rhs)
            }

            #[inline]
            fn lt(&self, rhs: &$Sig<FracRhs>) -> bool {
                if rhs.is_negative() {
                    return false;
                }
                let unsigned_rhs = $Uns::<FracRhs>::from_bits(rhs.to_bits() as $UnsInner);
                PartialOrd::lt(self, &unsigned_rhs)
            }

            #[inline]
            fn le(&self, rhs: &$Sig<FracRhs>) -> bool {
                if rhs.is_negative() {
                    return false;
                }
                let unsigned_rhs = $Uns::<FracRhs>::from_bits(rhs.to_bits() as $UnsInner);
                PartialOrd::le(self, &unsigned_rhs)
            }

            #[inline]
            fn gt(&self, rhs: &$Sig<FracRhs>) -> bool {
                if rhs.is_negative() {
                    return true;
                }
                let unsigned_rhs = $Uns::<FracRhs>::from_bits(rhs.to_bits() as $UnsInner);
                PartialOrd::gt(self, &unsigned_rhs)
            }

            #[inline]
            fn ge(&self, rhs: &$Sig<FracRhs>) -> bool {
                if rhs.is_negative() {
                    return true;
                }
                let unsigned_rhs = $Uns::<FracRhs>::from_bits(rhs.to_bits() as $UnsInner);
                PartialOrd::ge(self, &unsigned_rhs)
            }
        }

        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialOrd<$Uns<FracRhs>> for $Sig<FracLhs> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Uns<FracRhs>) -> Option<Ordering> {
                if self.is_negative() {
                    return Some(Ordering::Less);
                }
                let unsigned_lhs = $Uns::<FracLhs>::from_bits(self.to_bits() as $UnsInner);
                PartialOrd::partial_cmp(&unsigned_lhs, rhs)
            }

            #[inline]
            fn lt(&self, rhs: &$Uns<FracRhs>) -> bool {
                if self.is_negative() {
                    return true;
                }
                let unsigned_lhs = $Uns::<FracLhs>::from_bits(self.to_bits() as $UnsInner);
                PartialOrd::lt(&unsigned_lhs, rhs)
            }

            #[inline]
            fn le(&self, rhs: &$Uns<FracRhs>) -> bool {
                if self.is_negative() {
                    return true;
                }
                let unsigned_lhs = $Uns::<FracLhs>::from_bits(self.to_bits() as $UnsInner);
                PartialOrd::le(&unsigned_lhs, rhs)
            }

            #[inline]
            fn gt(&self, rhs: &$Uns<FracRhs>) -> bool {
                if self.is_negative() {
                    return false;
                }
                let unsigned_lhs = $Uns::<FracLhs>::from_bits(self.to_bits() as $UnsInner);
                PartialOrd::gt(&unsigned_lhs, rhs)
            }

            #[inline]
            fn ge(&self, rhs: &$Uns<FracRhs>) -> bool {
                if self.is_negative() {
                    return false;
                }
                let unsigned_lhs = $Uns::<FracLhs>::from_bits(self.to_bits() as $UnsInner);
                PartialOrd::ge(&unsigned_lhs, rhs)
            }
        }
    };

    ($Sig:ident) => {
        diff_sign! { $Sig, FixedU8(u8) }
        diff_sign! { $Sig, FixedU16(u16) }
        diff_sign! { $Sig, FixedU32(u32) }
        diff_sign! { $Sig, FixedU64(u64) }
        diff_sign! { $Sig, FixedU128(u128) }
    };
}

diff_sign! { FixedI8 }
diff_sign! { FixedI16 }
diff_sign! { FixedI32 }
diff_sign! { FixedI64 }
diff_sign! { FixedI128 }

macro_rules! diff_size {
    ($Nar:ident, $Wid:ident($WidInner:ident)) => {
        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialEq<$Nar<FracRhs>> for $Wid<FracLhs> {
            #[inline]
            fn eq(&self, rhs: &$Nar<FracRhs>) -> bool {
                let widened_rhs = $Wid::<FracRhs>::from_bits(rhs.to_bits() as $WidInner);
                PartialEq::eq(self, &widened_rhs)
            }
        }

        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialEq<$Wid<FracRhs>> for $Nar<FracLhs> {
            #[inline]
            fn eq(&self, rhs: &$Wid<FracRhs>) -> bool {
                let widened_lhs = $Wid::<FracLhs>::from_bits(self.to_bits() as $WidInner);
                PartialEq::eq(&widened_lhs, rhs)
            }
        }

        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialOrd<$Nar<FracRhs>> for $Wid<FracLhs> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Nar<FracRhs>) -> Option<Ordering> {
                let widened_rhs = $Wid::<FracRhs>::from_bits(rhs.to_bits() as $WidInner);
                PartialOrd::partial_cmp(self, &widened_rhs)
            }

            #[inline]
            fn lt(&self, rhs: &$Nar<FracRhs>) -> bool {
                let widened_rhs = $Wid::<FracRhs>::from_bits(rhs.to_bits() as $WidInner);
                PartialOrd::lt(self, &widened_rhs)
            }

            #[inline]
            fn le(&self, rhs: &$Nar<FracRhs>) -> bool {
                let widened_rhs = $Wid::<FracRhs>::from_bits(rhs.to_bits() as $WidInner);
                PartialOrd::le(self, &widened_rhs)
            }

            #[inline]
            fn gt(&self, rhs: &$Nar<FracRhs>) -> bool {
                let widened_rhs = $Wid::<FracRhs>::from_bits(rhs.to_bits() as $WidInner);
                PartialOrd::gt(self, &widened_rhs)
            }

            #[inline]
            fn ge(&self, rhs: &$Nar<FracRhs>) -> bool {
                let widened_rhs = $Wid::<FracRhs>::from_bits(rhs.to_bits() as $WidInner);
                PartialOrd::ge(self, &widened_rhs)
            }
        }

        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialOrd<$Wid<FracRhs>> for $Nar<FracLhs> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Wid<FracRhs>) -> Option<Ordering> {
                let widened_lhs = $Wid::<FracLhs>::from_bits(self.to_bits() as $WidInner);
                PartialOrd::partial_cmp(&widened_lhs, rhs)
            }

            #[inline]
            fn lt(&self, rhs: &$Wid<FracRhs>) -> bool {
                let widened_lhs = $Wid::<FracLhs>::from_bits(self.to_bits() as $WidInner);
                PartialOrd::lt(&widened_lhs, rhs)
            }

            #[inline]
            fn le(&self, rhs: &$Wid<FracRhs>) -> bool {
                let widened_lhs = $Wid::<FracLhs>::from_bits(self.to_bits() as $WidInner);
                PartialOrd::le(&widened_lhs, rhs)
            }

            #[inline]
            fn gt(&self, rhs: &$Wid<FracRhs>) -> bool {
                let widened_lhs = $Wid::<FracLhs>::from_bits(self.to_bits() as $WidInner);
                PartialOrd::gt(&widened_lhs, rhs)
            }

            #[inline]
            fn ge(&self, rhs: &$Wid<FracRhs>) -> bool {
                let widened_lhs = $Wid::<FracLhs>::from_bits(self.to_bits() as $WidInner);
                PartialOrd::ge(&widened_lhs, rhs)
            }
        }
    };
}

diff_size! { FixedI8, FixedI16(i16) }
diff_size! { FixedI8, FixedI32(i32) }
diff_size! { FixedI8, FixedI64(i64) }
diff_size! { FixedI8, FixedI128(i128) }
diff_size! { FixedI16, FixedI32(i32) }
diff_size! { FixedI16, FixedI64(i64) }
diff_size! { FixedI16, FixedI128(i128) }
diff_size! { FixedI32, FixedI64(i64) }
diff_size! { FixedI32, FixedI128(i128) }
diff_size! { FixedI64, FixedI128(i128) }
diff_size! { FixedU8, FixedU16(u16) }
diff_size! { FixedU8, FixedU32(u32) }
diff_size! { FixedU8, FixedU64(u64) }
diff_size! { FixedU8, FixedU128(u128) }
diff_size! { FixedU16, FixedU32(u32) }
diff_size! { FixedU16, FixedU64(u64) }
diff_size! { FixedU16, FixedU128(u128) }
diff_size! { FixedU32, FixedU64(u64) }
diff_size! { FixedU32, FixedU128(u128) }
diff_size! { FixedU64, FixedU128(u128) }

macro_rules! cmp {
    ($Fixed:ident($Inner:ident)) => {
        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialEq<$Fixed<FracRhs>> for $Fixed<FracLhs> {
            #[inline]
            fn eq(&self, rhs: &$Fixed<FracRhs>) -> bool {
                let lhs = self.to_bits();
                let rhs = rhs.to_bits();

                // lhs_extra_frac and nbits are known exactly at compile time,
                // so with optimizations the branch is selected at compile time
                let lhs_extra_frac = FracLhs::to_i32().saturating_sub(FracRhs::to_i32());
                let nbits = $Inner::BITS as i32;

                if lhs_extra_frac <= -nbits {
                    let shifted_rhs = rhs >> (nbits - 1) >> 1;
                    let rhs_is_reduced = rhs != 0;
                    lhs == shifted_rhs && !rhs_is_reduced
                } else if lhs_extra_frac < 0 {
                    let shifted_rhs = rhs >> -lhs_extra_frac;
                    let rhs_is_reduced = rhs != (shifted_rhs << -lhs_extra_frac);
                    lhs == shifted_rhs && !rhs_is_reduced
                } else if lhs_extra_frac == 0 {
                    lhs == rhs
                } else if lhs_extra_frac < nbits {
                    let shifted_lhs = lhs >> lhs_extra_frac;
                    let lhs_is_reduced = lhs != (shifted_lhs << lhs_extra_frac);
                    shifted_lhs == rhs && !lhs_is_reduced
                } else {
                    let shifted_lhs = lhs >> (nbits - 1) >> 1;
                    let lhs_is_reduced = lhs != 0;
                    shifted_lhs == rhs && !lhs_is_reduced
                }
            }
        }

        impl<FracLhs: Unsigned, FracRhs: Unsigned> PartialOrd<$Fixed<FracRhs>> for $Fixed<FracLhs> {
            #[inline]
            fn partial_cmp(&self, rhs: &$Fixed<FracRhs>) -> Option<Ordering> {
                let lhs = self.to_bits();
                let rhs = rhs.to_bits();

                // lhs_extra_frac and nbits are known exactly at compile time,
                // so with optimizations the branch is selected at compile time
                let lhs_extra_frac = FracLhs::to_i32().saturating_sub(FracRhs::to_i32());
                let nbits = $Inner::BITS as i32;
                if lhs_extra_frac <= -nbits {
                    let shifted_rhs = rhs >> (nbits - 1) >> 1;
                    let rhs_is_reduced = rhs != 0;
                    if lhs == shifted_rhs && rhs_is_reduced {
                        Some(Ordering::Less)
                    } else {
                        Some(Ord::cmp(&lhs, &shifted_rhs))
                    }
                } else if lhs_extra_frac < 0 {
                    let shifted_rhs = rhs >> -lhs_extra_frac;
                    let rhs_is_reduced = rhs != (shifted_rhs << -lhs_extra_frac);
                    if lhs == shifted_rhs && rhs_is_reduced {
                        Some(Ordering::Less)
                    } else {
                        Some(Ord::cmp(&lhs, &shifted_rhs))
                    }
                } else if lhs_extra_frac == 0 {
                    Some(Ord::cmp(&lhs, &rhs))
                } else if lhs_extra_frac < nbits {
                    let shifted_lhs = lhs >> lhs_extra_frac;
                    let lhs_is_reduced = lhs != (shifted_lhs << lhs_extra_frac);
                    if shifted_lhs == rhs && lhs_is_reduced {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ord::cmp(&shifted_lhs, &rhs))
                    }
                } else {
                    let shifted_lhs = lhs >> (nbits - 1) >> 1;
                    let lhs_is_reduced = lhs != 0;
                    if shifted_lhs == rhs && lhs_is_reduced {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ord::cmp(&shifted_lhs, &rhs))
                    }
                }
            }

            #[inline]
            fn lt(&self, rhs: &$Fixed<FracRhs>) -> bool {
                let lhs = self.to_bits();
                let rhs = rhs.to_bits();

                // lhs_extra_frac and nbits are known exactly at compile time,
                // so with optimizations the branch is selected at compile time
                let lhs_extra_frac = FracLhs::to_i32().saturating_sub(FracRhs::to_i32());
                let nbits = $Inner::BITS as i32;
                if lhs_extra_frac <= -nbits {
                    let shifted_rhs = rhs >> (nbits - 1) >> 1;
                    let rhs_is_reduced = rhs != 0;
                    (lhs == shifted_rhs && rhs_is_reduced) || lhs < shifted_rhs
                } else if lhs_extra_frac < 0 {
                    let shifted_rhs = rhs >> -lhs_extra_frac;
                    let rhs_is_reduced = rhs != (shifted_rhs << -lhs_extra_frac);
                    (lhs == shifted_rhs && rhs_is_reduced) || lhs < shifted_rhs
                } else if lhs_extra_frac == 0 {
                    lhs < rhs
                } else if lhs_extra_frac < nbits {
                    let shifted_lhs = lhs >> lhs_extra_frac;
                    shifted_lhs < rhs
                } else {
                    let shifted_lhs = lhs >> (nbits - 1) >> 1;
                    shifted_lhs < rhs
                }
            }

            #[inline]
            fn le(&self, rhs: &$Fixed<FracRhs>) -> bool {
                let lhs = self.to_bits();
                let rhs = rhs.to_bits();

                // lhs_extra_frac and nbits are known exactly at compile time,
                // so with optimizations the branch is selected at compile time
                let lhs_extra_frac = FracLhs::to_i32().saturating_sub(FracRhs::to_i32());
                let nbits = $Inner::BITS as i32;
                if lhs_extra_frac <= -nbits {
                    let shifted_rhs = rhs >> (nbits - 1) >> 1;
                    lhs <= shifted_rhs
                } else if lhs_extra_frac < 0 {
                    let shifted_rhs = rhs >> -lhs_extra_frac;
                    lhs <= shifted_rhs
                } else if lhs_extra_frac == 0 {
                    lhs <= rhs
                } else if lhs_extra_frac < nbits {
                    let shifted_lhs = lhs >> lhs_extra_frac;
                    let lhs_is_reduced = lhs != (shifted_lhs << lhs_extra_frac);
                    !(shifted_lhs == rhs && lhs_is_reduced) && shifted_lhs <= rhs
                } else {
                    let shifted_lhs = lhs >> (nbits - 1) >> 1;
                    let lhs_is_reduced = lhs != 0;
                    !(shifted_lhs == rhs && lhs_is_reduced) && shifted_lhs <= rhs
                }
            }

            #[inline]
            fn gt(&self, rhs: &$Fixed<FracRhs>) -> bool {
                let lhs = self.to_bits();
                let rhs = rhs.to_bits();

                // lhs_extra_frac and nbits are known exactly at compile time,
                // so with optimizations the branch is selected at compile time
                let lhs_extra_frac = FracLhs::to_i32().saturating_sub(FracRhs::to_i32());
                let nbits = $Inner::BITS as i32;
                if lhs_extra_frac <= -nbits {
                    let shifted_rhs = rhs >> (nbits - 1) >> 1;
                    lhs > shifted_rhs
                } else if lhs_extra_frac < 0 {
                    let shifted_rhs = rhs >> -lhs_extra_frac;
                    lhs > shifted_rhs
                } else if lhs_extra_frac == 0 {
                    lhs > rhs
                } else if lhs_extra_frac < nbits {
                    let shifted_lhs = lhs >> lhs_extra_frac;
                    let lhs_is_reduced = lhs != (shifted_lhs << lhs_extra_frac);
                    (shifted_lhs == rhs && lhs_is_reduced) || shifted_lhs > rhs
                } else {
                    let shifted_lhs = lhs >> (nbits - 1) >> 1;
                    let lhs_is_reduced = lhs != 0;
                    (shifted_lhs == rhs && lhs_is_reduced) || shifted_lhs > rhs
                }
            }

            #[inline]
            fn ge(&self, rhs: &$Fixed<FracRhs>) -> bool {
                let lhs = self.to_bits();
                let rhs = rhs.to_bits();

                // lhs_extra_frac and nbits are known exactly at compile time,
                // so with optimizations the branch is selected at compile time
                let lhs_extra_frac = FracLhs::to_i32().saturating_sub(FracRhs::to_i32());
                let nbits = $Inner::BITS as i32;
                if lhs_extra_frac <= -nbits {
                    let shifted_rhs = rhs >> (nbits - 1) >> 1;
                    let rhs_is_reduced = rhs != 0;
                    !(lhs == shifted_rhs && rhs_is_reduced) && lhs >= shifted_rhs
                } else if lhs_extra_frac < 0 {
                    let shifted_rhs = rhs >> -lhs_extra_frac;
                    let rhs_is_reduced = rhs != (shifted_rhs << -lhs_extra_frac);
                    !(lhs == shifted_rhs && rhs_is_reduced) && lhs >= shifted_rhs
                } else if lhs_extra_frac == 0 {
                    lhs >= rhs
                } else if lhs_extra_frac < nbits {
                    let shifted_lhs = lhs >> lhs_extra_frac;
                    shifted_lhs >= rhs
                } else {
                    let shifted_lhs = lhs >> (nbits - 1) >> 1;
                    shifted_lhs >= rhs
                }
            }
        }

        impl<Frac: Unsigned> Eq for $Fixed<Frac> {}

        impl<Frac: Unsigned> Ord for $Fixed<Frac> {
            #[inline]
            fn cmp(&self, rhs: &$Fixed<Frac>) -> Ordering {
                let lhs = self.to_bits();
                let rhs = rhs.to_bits();
                Ord::cmp(&lhs, &rhs)
            }
        }
    };
}

cmp! { FixedI8(i8) }
cmp! { FixedI16(i16) }
cmp! { FixedI32(i32) }
cmp! { FixedI64(i64) }
cmp! { FixedI128(i128) }
cmp! { FixedU8(u8) }
cmp! { FixedU16(u16) }
cmp! { FixedU32(u32) }
cmp! { FixedU64(u64) }
cmp! { FixedU128(u128) }
