// Copyright © 2018–2023 Trevor Spiteri

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

use core::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct Bytes<'a> {
    ptr: *const u8,
    len: usize,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> Bytes<'a> {
    pub const EMPTY: Bytes<'a> = Bytes::new(&[]);

    #[inline]
    pub const fn new(bytes: &'a [u8]) -> Bytes<'a> {
        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn len(self) -> usize {
        self.len
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    #[inline]
    pub const fn get(self, i: usize) -> u8 {
        assert!(i < self.len, "index out of bounds");
        let ptr = self.ptr.wrapping_add(i);
        // SAFETY: points to a valid slice, and bounds already checked
        unsafe { *ptr }
    }

    #[inline]
    pub const fn split(self, i: usize) -> (Bytes<'a>, Bytes<'a>) {
        let end_len = match self.len().checked_sub(i) {
            Some(s) => s,
            None => panic!("index out of bounds"),
        };
        (
            Bytes {
                ptr: self.ptr,
                len: i,
                phantom: PhantomData,
            },
            Bytes {
                ptr: self.ptr.wrapping_add(i),
                len: end_len,
                phantom: PhantomData,
            },
        )
    }

    #[inline]
    pub const fn split_first(self) -> Option<(u8, Bytes<'a>)> {
        if self.is_empty() {
            None
        } else {
            let (first, rest) = self.split(1);
            Some((first.get(0), rest))
        }
    }
}

// Kept trimmed: no underscores at beginning or end of slice
#[derive(Clone, Copy, Debug)]
pub struct DigitsUnds<'a> {
    ptr: *const u8,
    digits: usize,
    unds: usize,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> DigitsUnds<'a> {
    pub const EMPTY: DigitsUnds<'a> = DigitsUnds::new(Bytes::EMPTY);

    #[inline]
    pub const fn new(bytes: Bytes<'a>) -> DigitsUnds<'a> {
        let mut ptr = bytes.ptr;
        let mut digits = 0;
        let mut unds = 0;
        let mut pending_unds = 0;
        let mut rem_bytes = bytes;
        while let Some((byte, rem)) = rem_bytes.split_first() {
            rem_bytes = rem;

            if byte == b'_' {
                pending_unds += 1;
            } else {
                if digits == 0 {
                    ptr = ptr.wrapping_add(pending_unds);
                } else {
                    unds += pending_unds;
                }
                digits += 1;
                pending_unds = 0;
            }
        }
        DigitsUnds {
            ptr,
            digits,
            unds,
            phantom: PhantomData,
        }
    }

    #[inline]
    const fn bytes(self) -> Bytes<'a> {
        Bytes {
            ptr: self.ptr,
            len: self.digits + self.unds,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn n_digits(self) -> usize {
        self.digits
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.digits == 0
    }

    #[inline]
    pub const fn split(self, digit_index: usize) -> (DigitsUnds<'a>, DigitsUnds<'a>) {
        let last_digits = match self.digits.checked_sub(digit_index) {
            Some(s) => s,
            None => panic!("index out of bounds"),
        };
        if last_digits == 0 {
            return (self, DigitsUnds::EMPTY);
        }

        let mut remaining_digits = digit_index;
        let mut unds = 0;
        let mut index = 0;
        while remaining_digits > 0 {
            let ptr = self.ptr.wrapping_add(index);
            // SAFETY: there must be at least i digits, so ptr is in range
            let byte = unsafe { *ptr };
            if byte != b'_' {
                remaining_digits -= 1;
            } else {
                unds += 1;
            }
            index += 1;
        }
        let first = DigitsUnds {
            ptr: self.ptr,
            digits: digit_index,
            unds,
            phantom: PhantomData,
        };

        // we need to reduce seps by numbers of SEP bytes between first part and last part
        let mut remaining_unds = self.unds - unds;
        loop {
            let ptr = self.ptr.wrapping_add(index);
            // SAFETY: there must be at least 1 more digit, otherwise we
            // would have returned earlier in `last_digits == 0` condition.
            let byte = unsafe { *ptr };
            if byte != b'_' {
                return (
                    first,
                    DigitsUnds {
                        ptr,
                        digits: last_digits,
                        unds: remaining_unds,
                        phantom: PhantomData,
                    },
                );
            }
            remaining_unds -= 1;
            index += 1;
        }
    }

    #[inline]
    pub const fn split_first(self) -> Option<(u8, DigitsUnds<'a>)> {
        if self.is_empty() {
            return None;
        }
        // first byte is never underscore
        let first = self.bytes().get(0);
        debug_assert!(first != b'_');
        let rem_digits = self.digits - 1;
        if rem_digits == 0 {
            return Some((first, DigitsUnds::EMPTY));
        }

        // index is 1 as [0] is the first digit
        let mut index = 1;
        // we need to reduce unds by numbers of underscores between first digit and remainder
        let mut rem_unds = self.unds;
        loop {
            let ptr = self.ptr.wrapping_add(index);
            // SAFETY: there must be at least 1 more digit, otherwise we
            // would have returned earlier in `rem_digits == 0` condition.
            let byte = unsafe { *ptr };
            if byte != b'_' {
                return Some((
                    first,
                    DigitsUnds {
                        ptr,
                        digits: rem_digits,
                        unds: rem_unds,
                        phantom: PhantomData,
                    },
                ));
            }
            rem_unds -= 1;
            index += 1;
        }
    }

    #[inline]
    pub const fn split_last(self) -> Option<(DigitsUnds<'a>, u8)> {
        if self.is_empty() {
            return None;
        }
        // last byte is never underscore
        let last = self.bytes().get(self.digits + self.unds - 1);
        debug_assert!(last != b'_');
        let rem_digits = self.digits - 1;
        if rem_digits == 0 {
            return Some((DigitsUnds::EMPTY, last));
        }

        // index is digits + unds - 2 as [digits + unds - 1] is the last digit
        let mut index = self.digits + self.unds - 2;
        // we need to reduce unds by numbers of underscores between last digit and remainder
        let mut rem_unds = self.unds;
        loop {
            let check_ptr = self.ptr.wrapping_add(index);
            // SAFETY: there must be at least 1 more digit, otherwise we
            // would have returned earlier in `rem_digits == 0` condition.
            let byte = unsafe { *check_ptr };
            if byte != b'_' {
                return Some((
                    DigitsUnds {
                        ptr: self.ptr,
                        digits: rem_digits,
                        unds: rem_unds,
                        phantom: PhantomData,
                    },
                    last,
                ));
            }
            rem_unds -= 1;
            index += 1;
        }
    }

    const fn split_leading_zeros(self) -> (usize, DigitsUnds<'a>) {
        let mut zeros = 0;
        let mut rem = self;
        while let Some((b'0', rest)) = rem.split_first() {
            zeros += 1;
            rem = rest;
        }
        return (zeros, rem);
    }

    const fn split_trailing_zeros(self) -> (DigitsUnds<'a>, usize) {
        let mut zeros = 0;
        let mut rem = self;
        while let Some((rest, b'0')) = rem.split_last() {
            zeros += 1;
            rem = rest;
        }
        return (rem, zeros);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DigitsExp<'a> {
    leading_zeros: usize,
    part1: DigitsUnds<'a>,
    part2: DigitsUnds<'a>,
    trailing_zeros: usize,
}

impl<'a> DigitsExp<'a> {
    const EMPTY: DigitsExp<'a> = DigitsExp {
        leading_zeros: 0,
        part1: DigitsUnds::EMPTY,
        part2: DigitsUnds::EMPTY,
        trailing_zeros: 0,
    };

    #[inline]
    const fn new1(digits: DigitsUnds<'a>) -> DigitsExp<'a> {
        let (leading_zeros, rest) = digits.split_leading_zeros();
        let (rest, trailing_zeros) = rest.split_trailing_zeros();
        DigitsExp {
            leading_zeros,
            part1: rest,
            part2: DigitsUnds::EMPTY,
            trailing_zeros,
        }
    }

    // digits1 and digits2 should be in the same block, so length sum won't overflow
    #[inline]
    const fn new2(digits1: DigitsUnds<'a>, digits2: DigitsUnds<'a>) -> DigitsExp<'a> {
        let (mut leading_zeros, mut digits1) = digits1.split_leading_zeros();
        let digits2 = if digits1.is_empty() {
            let (more_leading_zeros, new_digits1) = digits2.split_leading_zeros();
            leading_zeros += more_leading_zeros;
            digits1 = new_digits1;
            DigitsUnds::EMPTY
        } else {
            digits2
        };
        let (digits2, mut trailing_zeros) = digits2.split_trailing_zeros();
        if digits2.is_empty() {
            let (new_digits1, more_trailing_zeros) = digits1.split_trailing_zeros();
            trailing_zeros += more_trailing_zeros;
            digits1 = new_digits1;
        }
        DigitsExp {
            leading_zeros,
            part1: digits1,
            part2: digits2,
            trailing_zeros,
        }
    }

    // exp.unsigned_abs() must fit in usize, and results must have lengths that fit in usize
    pub const fn new_int_frac(
        int: DigitsUnds<'a>,
        frac: DigitsUnds<'a>,
        exp: i32,
    ) -> Option<(DigitsExp<'a>, DigitsExp<'a>)> {
        let (mut int, mut frac) = if exp == 0 {
            (DigitsExp::new1(int), DigitsExp::new1(frac))
        } else if exp < 0 {
            let abs_exp = exp.unsigned_abs() as usize;
            if abs_exp as u32 != exp.unsigned_abs() || abs_exp > usize::MAX - frac.n_digits() {
                return None;
            }
            if abs_exp < int.n_digits() {
                let int = int.split(int.n_digits() - abs_exp);
                (DigitsExp::new1(int.0), DigitsExp::new2(int.1, frac))
            } else {
                let mut frac = DigitsExp::new2(int, frac);
                frac.leading_zeros += abs_exp - int.n_digits();
                (DigitsExp::EMPTY, frac)
            }
        } else {
            // exp > 0
            let abs_exp = exp.unsigned_abs() as usize;
            if abs_exp as u32 != exp.unsigned_abs() || abs_exp > usize::MAX - int.n_digits() {
                return None;
            }
            if abs_exp < frac.n_digits() {
                let frac = frac.split(abs_exp);
                (DigitsExp::new2(int, frac.0), DigitsExp::new1(frac.1))
            } else {
                let mut int = DigitsExp::new2(int, frac);
                int.trailing_zeros += abs_exp - frac.n_digits();
                (int, DigitsExp::EMPTY)
            }
        };
        int.leading_zeros = 0;
        if int.part1.is_empty() && int.part2.is_empty() {
            int.trailing_zeros = 0;
        }
        frac.trailing_zeros = 0;
        if frac.part2.is_empty() && frac.part1.is_empty() {
            frac.leading_zeros = 0;
        }
        Some((int, frac))
    }

    #[inline]
    pub const fn n_digits(self) -> usize {
        self.leading_zeros + self.part1.n_digits() + self.part2.n_digits() + self.trailing_zeros
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.n_digits() == 0
    }

    #[inline]
    pub const fn split(self, mut digit_index: usize) -> (DigitsExp<'a>, DigitsExp<'a>) {
        if digit_index <= self.leading_zeros {
            return (
                DigitsExp {
                    leading_zeros: digit_index,
                    part1: DigitsUnds::EMPTY,
                    part2: DigitsUnds::EMPTY,
                    trailing_zeros: 0,
                },
                DigitsExp {
                    leading_zeros: self.leading_zeros - digit_index,
                    part1: self.part1,
                    part2: self.part2,
                    trailing_zeros: self.trailing_zeros,
                },
            );
        }
        digit_index -= self.leading_zeros;
        if digit_index <= self.part1.n_digits() {
            let part1 = self.part1.split(digit_index);
            return (
                DigitsExp {
                    leading_zeros: self.leading_zeros,
                    part1: part1.0,
                    part2: DigitsUnds::EMPTY,
                    trailing_zeros: 0,
                },
                DigitsExp {
                    leading_zeros: 0,
                    part1: part1.1,
                    part2: self.part2,
                    trailing_zeros: self.trailing_zeros,
                },
            );
        }
        digit_index -= self.part1.n_digits();
        if digit_index <= self.part2.n_digits() {
            let part2 = self.part2.split(digit_index);
            return (
                DigitsExp {
                    leading_zeros: self.leading_zeros,
                    part1: self.part1,
                    part2: part2.0,
                    trailing_zeros: 0,
                },
                DigitsExp {
                    leading_zeros: 0,
                    part1: DigitsUnds::EMPTY,
                    part2: part2.1,
                    trailing_zeros: self.trailing_zeros,
                },
            );
        }
        digit_index -= self.part2.n_digits();
        if digit_index <= self.trailing_zeros {
            return (
                DigitsExp {
                    leading_zeros: self.leading_zeros,
                    part1: self.part1,
                    part2: self.part2,
                    trailing_zeros: digit_index,
                },
                DigitsExp {
                    leading_zeros: self.trailing_zeros - digit_index,
                    part1: DigitsUnds::EMPTY,
                    part2: DigitsUnds::EMPTY,
                    trailing_zeros: 0,
                },
            );
        }
        panic!("index out of bounds");
    }

    // no automatic renormalization done after split_first
    #[inline]
    pub const fn split_first(self) -> Option<(u8, DigitsExp<'a>)> {
        if self.leading_zeros > 0 {
            return Some((
                b'0',
                DigitsExp {
                    leading_zeros: self.leading_zeros - 1,
                    ..self
                },
            ));
        }
        match self.part1.split_first() {
            Some((first, rest)) => {
                return Some((
                    first,
                    DigitsExp {
                        part1: rest,
                        ..self
                    },
                ));
            }
            None => {}
        }
        match self.part2.split_first() {
            Some((first, rest)) => {
                return Some((
                    first,
                    DigitsExp {
                        part2: rest,
                        ..self
                    },
                ));
            }
            None => {}
        }
        if self.trailing_zeros > 0 {
            return Some((
                b'0',
                DigitsExp {
                    trailing_zeros: self.trailing_zeros - 1,
                    ..self
                },
            ));
        }
        None
    }
}
