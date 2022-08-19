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

macro_rules! impl_int_part {
    ($u:ident, $max_table_size:expr) => {
        pub const fn $u(val: $u, base: u32) -> i32 {
            debug_assert!(val > 0);
            debug_assert!(base >= 2);

            let baseu = base as $u;
            if baseu as u32 != base || val < baseu {
                return 0;
            }

            let mut table: [$u; $max_table_size] = [0; $max_table_size];

            let mut i = 0;
            let mut partial_log = 1;
            let mut partial_val = baseu;

            loop {
                let square = match partial_val.checked_mul(partial_val) {
                    Some(s) if s <= val => s,
                    _ => break,
                };
                table[i] = partial_val;
                i += 1;
                partial_log *= 2;
                partial_val = square;
            }
            let mut dlog = partial_log;
            // for not allowed in const fn, so use while
            let mut j = 0;
            while j < i {
                j += 1;
                dlog /= 2;
                if let Some(mid) = partial_val.checked_mul(table[i - j]) {
                    if val >= mid {
                        partial_val = mid;
                        partial_log += dlog;
                    }
                }
            }
            return partial_log;
        }
    };
}

pub mod int_part {
    impl_int_part! { u8, 2 }
    impl_int_part! { u16, 3 }
    impl_int_part! { u32, 4 }
    impl_int_part! { u64, 5 }
    impl_int_part! { u128, 6 }
}
