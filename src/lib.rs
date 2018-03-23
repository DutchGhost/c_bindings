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

macro_rules! unroll {
    ($d1:ident, $r1:ident, $i:expr, $ptr:expr) => (
        let $d1 = (*$ptr - b'0') as u64;
        let $r1 = $d1 * POW10.get_unchecked($i);
        $ptr = $ptr.add(1);
    )
}

#[inline(always)]
pub fn safe_atoi(s: &str) -> u64 {
    // let mut result = 0;
    
    // unsafe {
    //     let mut ptr = s.as_ptr();
    //     let mut len = s.len();
    //     let mut i = 20 - len;

    //     while len >= 4 {
    //         unroll!(d1, r1, i, ptr);
    //         unroll!(d2, r2, i + 1, ptr);
    //         unroll!(d3, r3, i + 2, ptr);
    //         unroll!(d4, r4, i + 3, ptr);

    //         result += r1 + r2 + r3 + r4;
    //         i += 4;
    //         len -= 4;
    //     }

    //     for _ in 0..len {
    //         let d = (*ptr - b'0') as u64;
    //         let r = d * POW10.get_unchecked(i);
    //         result += r;
    //         i += 1;
    //         ptr = ptr.add(1);
    //     }

    //     result
    // }
    
    let BYTES_ONCE = s.as_bytes();
    let mut result = 0;
    let mut bytes = BYTES_ONCE;
    let mut len = s.len();
    let l = s.len();
    let mut i = 20 - len;

    unsafe {
        while len >= 4 {
            let d1 = (bytes.get_unchecked(0) - 48) as u64;
            let r1 = d1 * POW10.get_unchecked(i);

            let d2 = (bytes.get_unchecked(1) - 48) as u64;
            let r2 = d2 * POW10.get_unchecked(i + 1);

            let d3 = (bytes.get_unchecked(2) - 48) as u64;
            let r3 = d3 * POW10.get_unchecked(i + 2);

            let d4 = (bytes.get_unchecked(3) - 48) as u64;
            let r4 = d4 * POW10.get_unchecked(i + 3);
            
            result += r1 + r2 + r3 + r4;
            len -= 4;

            i += 4;
            bytes = &BYTES_ONCE[l - len..];
        }
        for fix in 0..len {
            let d = (bytes.get_unchecked(fix) - 48) as u64;
            let r = d * POW10.get_unchecked(i + fix);
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