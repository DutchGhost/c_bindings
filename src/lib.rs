extern crate libc;

use libc::uint64_t;

#[link(name = "c_bindings", kind="static")]
extern {
    fn c_clzl(x: uint64_t) -> uint64_t;
}

/// Returns the number of leading zero's of an unsigned 64-bit integer.
/// clzl of 0 is Undefined Behaviour, so return 0 instead.
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
///     assert_eq!(rust_clzl(0), 0);
/// }
/// ```
#[inline]
pub fn rust_clzl(x: u64) -> u64 {
    
    //clzl of 0 is UB. return 0 instead.
    if x == 0 { return 0 }
    unsafe {
        c_clzl(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_clzl() {
        assert_eq!(rust_clzl(1), 63);
        assert_eq!(rust_clzl(2), 62);
        assert_eq!(rust_clzl(8), 60);
        assert_eq!(rust_clzl(std::u64::MAX), 0);
        assert_eq!(rust_clzl(0), 0);
    }
}