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
use core::slice;

#[derive(Clone, Copy, Debug)]
pub struct Bytes<'a> {
    ptr: *const u8,
    len: usize,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> Bytes<'a> {
    #[inline]
    pub const fn new(bytes: &'a [u8]) -> Bytes<'a> {
        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            phantom: PhantomData,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn slice(self) -> &'a [u8] {
        // SAFETY: points to a valid slice
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
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
    pub const fn bytes(self) -> Bytes<'a> {
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
        let len = self.digits + self.unds;
        let last_digits = match self.digits.checked_sub(digit_index) {
            Some(s) => s,
            None => panic!("index out of bounds"),
        };
        if last_digits == 0 {
            return (
                self,
                DigitsUnds {
                    ptr: self.ptr.wrapping_add(len),
                    digits: 0,
                    unds: 0,
                    phantom: PhantomData,
                },
            );
        }

        let mut remaining_digits = digit_index;
        let mut unds = 0;
        let mut index = 0;
        while remaining_digits > 0 {
            let ptr = self.ptr.wrapping_add(index);
            // SAFETY: there must be at least i digit, so ptr is in range
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
            None
        } else {
            let (first, rest) = self.split(1);
            Some((first.bytes().get(0), rest))
        }
    }
}
