extern crate libc;

use libc::uint64_t;

#[link(name = "c_bindings", kind="static")]
extern {
    fn c_clzl(x: uint64_t) -> uint64_t;
}

#[inline]
pub fn rust_clzl(x: u64) -> u64 {
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
    }
}