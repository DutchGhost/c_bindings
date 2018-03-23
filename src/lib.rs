#![feature(test)]
#![feature(iterator_step_by)]

extern crate libc;
extern crate test;

use libc::uint64_t;

const POW10: [u64; 20] = [
        10000000000000000000,
        1000000000000000000,
        100000000000000000,
        10000000000000000,
        1000000000000000,
        100000000000000,
        10000000000000,
        1000000000000,
        100000000000,
        10000000000,
        1000000000,
        100000000,
        10000000,
        1000000,
        100000,
        10000,
        1000,
        100,
        10,
        1,
    ];

#[link(name = "c_bindings", kind="static")]
extern {
    #[inline]
    fn c_clzl(x: uint64_t) -> uint64_t;
    fn c_atoi(b: *const u8, e: uint64_t) -> uint64_t;

}

/// Returns the number of leading zero's of an unsigned 64-bit integer.
/// clzl of 0 is Undefined Behaviour, so return 64 instead.
/// 
/// # Examples
/// ```
/// extern crate c_bindings;
/// use c_bindings::rust_clzl;
/// 
/// fn main() {
///     assert_eq!(rust_clzl(1), 63);
///     assert_eq!(rust_clzl(2), 62);
///     assert_eq!(rust_clzl(8), 60);
///     assert_eq!(rust_clzl(std::u64::MAX), 0);
///     assert_eq!(rust_clzl(0), 64);
/// }
/// ```
#[inline]
pub fn rust_clzl(x: u64) -> u64 {
    
    //clzl of 0 is UB. return 64 instead, since there are 64 zero's.
    if x == 0 { return 64 }
    unsafe {
        c_clzl(x)
    }
}

/// Converts a `str` into a u64.
/// # Safety
/// It's required for all the characters to be valid digits.
/// 
/// # Examples
/// ```
/// extern crate c_bindings;
/// use c_bindings::rust_atoi;
/// 
/// fn main() {
///     assert_eq!(rust_atoi("98765"), 98765);
/// }
/// ```
pub fn rust_atoi(s: &str) -> u64 {
    unsafe {
        c_atoi(s.as_ptr(), s.len() as u64)
    }
}

macro_rules! atoi_unroll {
    ($d:ident, $r:ident, $bytes:expr, $i:expr, $idx:expr) => (
        let $d = ($bytes.get_unchecked($idx) - 48) as u64;
        let $r = $d * POW10.get_unchecked($i + $idx);
    )
}

#[inline(always)]
pub fn safe_atoi(s: &str) -> u64 {
    
    let BYTE_SLICE = s.as_bytes();
    let SLICE_LEN = s.len();
    
    let mut result = 0;
    let mut bytes = BYTE_SLICE;
    let mut len = s.len();
    let mut i = 20 - len;

    unsafe {
        while len >= 4 {
            atoi_unroll!(d1, r1, bytes, i, 0);
            atoi_unroll!(d2, r2, bytes, i, 1);
            atoi_unroll!(d3, r3, bytes, i, 2);
            atoi_unroll!(d4, r4, bytes, i, 3); 
            
            result += r1 + r2 + r3 + r4;
            len -= 4;

            i += 4;
            bytes = &BYTE_SLICE[SLICE_LEN - len..];
        }

        for fix in 0..len {
            atoi_unroll!(d, r, bytes, i, fix);
            result += r;
        }
        return result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    #[test]
    fn test_rust_clzl() {
        assert_eq!(rust_clzl(1), 63);
        assert_eq!(rust_clzl(2), 62);
        assert_eq!(rust_clzl(8), 60);
        assert_eq!(rust_clzl(std::u64::MAX), 0);
        assert_eq!(rust_clzl(0), 64);
    }

    #[test]
    fn test_rust_atoi() {
        let s = "100";

        assert_eq!(rust_atoi(s), 100);

        let ss = "1234";
        assert_eq!(rust_atoi(ss), 1234);

        let sss = "4";
        assert_eq!(rust_atoi(sss), 4);
    }

    #[test]
    fn test_safe_atoi() {
        assert_eq!(safe_atoi("1234"), 1234);
        assert_eq!(safe_atoi("123"), 123);

        assert_eq!(safe_atoi("12"), 12);
    }

    #[bench]
    fn rust_native_parse(b: &mut test::Bencher) {
        let s = ["123498", "987234", "8907239874", "982734", "123876", "10987", "84750"];
        b.iter(|| {
            for item in s.iter() {
                let _n = test::black_box(item.parse::<u64>());
            }
        })
    }
    #[bench]
    fn rust_atoi_parse(b: &mut test::Bencher) {
        let s = ["123498", "987234", "8907239874", "982734", "123876", "10987", "84750"];
        b.iter(|| {
            for item in s.iter() {
                let _n = test::black_box(rust_atoi(item));
            }
        })
    }

    #[bench]
    fn rust_safe_atoi_parse(b: &mut test::Bencher) {
        let s = ["123498", "987234", "8907239874", "982734", "123876", "10987", "84750"];
        b.iter(|| {
            for item in s.iter() {
                let _n = test::black_box(safe_atoi(item));
            }
        })
    }
}