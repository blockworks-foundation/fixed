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

use core::{marker::PhantomData, slice};

#[derive(Clone, Copy, Debug)]
pub struct Bytes<'a> {
    ptr: *const u8,
    len: usize,
    phantom: PhantomData<&'a [u8]>,
}

#[derive(Clone, Copy, Debug)]
pub struct BytesSeps<'a, const SEP: u8> {
    bytes: Bytes<'a>,
    seps: usize,
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
}

impl<'a, const SEP: u8> BytesSeps<'a, SEP> {
    // Only checked in debug mode
    #[inline]
    pub const fn new(bytes: Bytes<'a>, seps: usize) -> BytesSeps<'a, SEP> {
        #[cfg(debug_assertions)]
        {
            let mut count = 0;
            let mut i = 0;
            while i < bytes.len() {
                let byte = bytes.get(i);
                i += 1;
                if byte == SEP {
                    count += 1;
                }
            }
            assert!(count == seps);
        }
        BytesSeps { bytes, seps }
    }

    // Length of slice minus seps
    #[inline]
    pub const fn len(self) -> usize {
        self.bytes.len() - self.seps
    }

    // Only seps is still empty
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn bytes_inc_seps(self) -> Bytes<'a> {
        self.bytes
    }

    // Split the first i bytes which are not SEP, together with any SEP bytes
    // touching them, into the first part, with the remaining bytes in the
    // second part. Any SEP bytes right after the ith non-SEP byte are included
    // in the first part, not the second.
    //
    // For example, with SEP = b'_'
    // split_sep((b"a__b__c__", 6), 2) -> ((b"a__b__", 4), (b"c__", 2))
    #[inline]
    pub const fn split(self, i: usize) -> (BytesSeps<'a, SEP>, BytesSeps<'a, SEP>) {
        let mut split_i = 0;
        let mut rem = i;
        while rem > 0 {
            assert!(split_i < self.bytes.len, "index out of bounds");
            let ptr = self.bytes.ptr.wrapping_add(split_i);
            // SAFETY: points to a valid slice, and bounds checked
            let byte = unsafe { *ptr };
            if byte != SEP {
                rem -= 1;
            }
            split_i += 1;
        }
        while split_i < self.bytes.len {
            let ptr = self.bytes.ptr.wrapping_add(split_i);
            // SAFETY: points to a valid slice, and bounds checked
            let byte = unsafe { *ptr };
            if byte != SEP {
                break;
            }
            split_i += 1;
        }
        let (begin, end) = self.bytes.split(split_i);
        let begin_seps = split_i - i;
        let end_seps = self.seps - begin_seps;
        (
            BytesSeps {
                bytes: begin,
                seps: begin_seps,
            },
            BytesSeps {
                bytes: end,
                seps: end_seps,
            },
        )
    }
}
