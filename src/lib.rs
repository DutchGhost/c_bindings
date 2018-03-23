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

pub fn safe_atoi(s: &str) -> u64 {
    let b = s.as_bytes();
    let mut len = s.len();
    let mut i = 20 - len;
    let mut idx = 0;
    let mut result = 0;
    unsafe {
        while len >= 4 {
            let d1 = (b.get_unchecked(idx) - b'0') as u64;
            let mut r1 = d1 * POW10.get_unchecked(i);

            let d2 = (b.get_unchecked(idx + 1) - b'0') as u64;
            let mut r2 = d2 * POW10.get_unchecked(i + 1);

            let d3 = (b.get_unchecked(idx + 2) - b'0') as u64;
            let mut r3 = d3 * POW10.get_unchecked(i + 2);

            let d4 = (b.get_unchecked(idx + 3) - b'0') as u64;
            let mut r4 = d4 * POW10.get_unchecked(i + 3);

            i += 4;
            idx += 4;
            len -= 4;

            result += r1 + r3 + r2 + r4;
        }

        for _ in 0..len {
            let d = (b.get_unchecked(idx) - b'0') as u64;
            let r = d * POW10.get_unchecked(i);

            idx += 1;
            i += 1;

            result += r;
        }

        return result
    }
//     let mut length = s.len();
//     let mut result: u64 = 0;
//     unsafe {
//         for (chunk, pow) in s.as_bytes().chunks(4).zip(POW10[20 - length..].chunks(4)) {
//             let chunklen = chunk.len();

//             if chunklen < 1 { return result }
//             let mut d1 = (chunk.get_unchecked(0) - b'0') as u64;
//             let r1 = d1 * pow.get_unchecked(0);

//             if chunklen < 2 { return result + r1 }            
//             let mut d2 = (chunk.get_unchecked(1) - b'0') as u64;
//             let r2= d2 * pow.get_unchecked(1);

//             if chunklen < 3 { return result + r1 + r2 }            
//             let mut d3 = (chunk.get_unchecked(2) - b'0') as u64;
//             let r3 = d3 * pow.get_unchecked(2);

//             if chunklen < 4 { return result + r1 + r2 + r3}            
//             let mut d3 = (chunk.get_unchecked(3) - b'0') as u64;
//             let r4 = d3 * pow.get_unchecked(3);

//             result += r1 + r2 + r3 + r4;
//         }
//     }

//     return result;
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
                let mut n = test::black_box(item.parse::<u64>());
            }
        })
    }
    #[bench]
    fn rust_atoi_parse(b: &mut test::Bencher) {
        let s = ["123498", "987234", "8907239874", "982734", "123876", "10987", "84750"];
        b.iter(|| {
            for item in s.iter() {
                let mut n = test::black_box(rust_atoi(item));
            }
        })
    }

    #[bench]
    fn rust_safe_atoi_parse(b: &mut test::Bencher) {
        let s = ["123498", "987234", "8907239874", "982734", "123876", "10987", "84750"];
        b.iter(|| {
            for item in s.iter() {
                let mut n = test::black_box(safe_atoi(item));
            }
        })
    }
}