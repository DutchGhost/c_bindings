 #![feature(test)]
extern crate libc;
extern crate test;

use libc::uint64_t;

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

pub fn rust_atoi(s: &str) -> u64 {
    unsafe {
        c_atoi(s.as_ptr(), s.len() as u64)
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
}