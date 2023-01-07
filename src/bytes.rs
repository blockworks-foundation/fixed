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

// Kept trimmed of SEP: no SEP bytes at beginning or end of slice
#[derive(Clone, Copy, Debug)]
pub struct BytesSeps<'a, const SEP: u8> {
    ptr: *const u8,
    not_seps: usize,
    seps: usize,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a, const SEP: u8> BytesSeps<'a, SEP> {
    #[inline]
    pub const fn new(bytes: Bytes<'a>) -> BytesSeps<'a, SEP> {
        let mut ptr = bytes.ptr;
        let mut not_seps = 0;
        let mut seps = 0;
        let mut pending_seps = 0;
        let mut rem_bytes = bytes;
        while let Some((byte, rem)) = rem_bytes.split_first() {
            rem_bytes = rem;

            if byte == SEP {
                pending_seps += 1;
                continue;
            } else {
                if not_seps == 0 {
                    ptr = ptr.wrapping_add(pending_seps);
                } else {
                    seps += pending_seps;
                }
                not_seps += 1;
                pending_seps = 0;
            }
        }
        BytesSeps {
            ptr,
            not_seps,
            seps,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn bytes_inc_seps(self) -> Bytes<'a> {
        Bytes {
            ptr: self.ptr,
            len: self.not_seps + self.seps,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn len(self) -> usize {
        self.not_seps
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.not_seps == 0
    }

    #[inline]
    pub const fn split(self, i: usize) -> (BytesSeps<'a, SEP>, BytesSeps<'a, SEP>) {
        let slice_len = self.not_seps + self.seps;
        let end_not_seps = match self.not_seps.checked_sub(i) {
            Some(s) => s,
            None => panic!("index out of bounds"),
        };
        if end_not_seps == 0 {
            return (
                self,
                BytesSeps {
                    ptr: self.ptr.wrapping_add(slice_len),
                    not_seps: 0,
                    seps: 0,
                    phantom: PhantomData,
                },
            );
        }

        let mut remaining_not_seps = i;
        let mut seps = 0;
        let mut index = 0;
        while remaining_not_seps > 0 {
            let ptr = self.ptr.wrapping_add(index);
            // SAFETY: there must be at least i not_seps, so ptr is in range
            let byte = unsafe { *ptr };
            if byte != SEP {
                remaining_not_seps -= 1;
            } else {
                seps += 1;
            }
            index += 1;
        }
        let begin = BytesSeps {
            ptr: self.ptr,
            not_seps: i,
            seps,
            phantom: PhantomData,
        };

        // we need to reduce seps by numbers of SEP bytes between begin and end
        seps = self.seps - seps;
        loop {
            let ptr = self.ptr.wrapping_add(index);
            // SAFETY: there must be at least 1 more not_seps, otherwise we
            // would have returned earlier in `end_not_seps == 0` condition.
            let byte = unsafe { *ptr };
            if byte != SEP {
                return (
                    begin,
                    BytesSeps {
                        ptr,
                        not_seps: end_not_seps,
                        seps,
                        phantom: PhantomData,
                    },
                );
            }
            seps -= 1;
            index += 1;
        }
    }

    #[inline]
    pub const fn split_first(self) -> Option<(u8, BytesSeps<'a, SEP>)> {
        if self.is_empty() {
            None
        } else {
            let (first, rest) = self.split(1);
            Some((first.bytes_inc_seps().get(0), rest))
        }
    }
}
