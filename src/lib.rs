#![feature(test)]
#[allow(non_snake_case)]

extern crate libc;
extern crate test;

use libc::uint64_t;

macro_rules! const_table {
    ($type:ty, $name:ident) => (
        const $name: [$type; 20] = [
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
    )
}
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

/// A trait to convert a `str` into an unsigned integer.
pub trait Atoi {

    /// Performs the convertion.
    /// # Examples
    /// ```
    /// extern crate c_bindings;
    /// use c_bindings::Atoi;
    /// fn main() {
    ///     assert_eq!(u32::atoi("54321"), Ok(54321));
    ///     assert_eq!(u32::atoi("5432e"), Err(()));
    /// }
    /// ```
    fn atoi(s: &str) -> Result<Self, ()> where Self: Sized;
}

macro_rules! atoi_unroll {
    ($d:ident, $r:ident, $bytes:expr, $idx:expr, $offset:expr, $TABLE:ident) => (
        let $d = ($bytes.get_unchecked($offset) - 48) as Self;
        
        //if the digit is greater than 9, something went terribly horribly wrong.
        //return an Err(())
        if $d > 9 {
            return Err(())
        }
        let $r = $d * $TABLE.get_unchecked($idx + $offset);
    )
}

macro_rules! impl_atoi {
    ($int:ty, $table_name:ident) => (

        //set up the constant table for this type.
        const_table!($int, $table_name);

        impl Atoi for $int {

            #[allow(non_snake_case)]
            #[inline]
            fn atoi(s: &str) -> Result<Self, ()> {

                //convert the str into a byte slice,
                //also store the length.
                //These are `constant` variables in this function.
                let BYTE_SLICE: &[u8] = s.as_bytes();
                let SLICE_LEN = s.len();
                
                //Initialize all variables.
                let mut result: Self = 0;
                let mut bytes: &[u8] = BYTE_SLICE;
                let mut len: usize = s.len();
                let mut idx: usize = 20 - len;

                unsafe {

                    //as long as there are more than 4 digits,
                    //do this loop.
                    //lookup the digit of the current idx, convert to Self (u8, u16, u32...),
                    //check if valid,
                    //multiply with the correct power of 10 from the lookup table.
                    //update the result so far, the length, the idx.
                    //lastly update the view in the slice.
                    while len >= 4 {
                        atoi_unroll!(d1, r1, bytes, idx, 0, $table_name);
                        atoi_unroll!(d2, r2, bytes, idx, 1, $table_name);
                        atoi_unroll!(d3, r3, bytes, idx, 2, $table_name);
                        atoi_unroll!(d4, r4, bytes, idx, 3, $table_name); 
                        
                        result += r1 + r2 + r3 + r4;
                        len -= 4;

                        idx += 4;
                        bytes = &BYTE_SLICE[SLICE_LEN - len..];
                    }

                    //A fixup loop, loops for a max of 3 times
                    for offset in 0..len {
                        atoi_unroll!(d, r, bytes, idx, offset, $table_name);
                        result += r;
                    }
                    return Ok(result)
                }
            }
        }
    );
}

impl_atoi!(u8, POW10_U8);
impl_atoi!(u16, POW10_U16);
impl_atoi!(u32, POW10_U32);
impl_atoi!(u64, POW10_U64);
impl_atoi!(usize, POW10_USIZE);

//#[inline(always)]
// pub fn safe_atoi(s: &str) -> Result<u64, ()> {
    
//     let BYTE_SLICE = s.as_bytes();
//     let SLICE_LEN = s.len();
    
//     let mut result = 0;
//     let mut bytes = BYTE_SLICE;
//     let mut len = s.len();
//     let mut idx = 20 - len;

//     unsafe {
//         while len >= 4 {
//             atoi_unroll!(d1, r1, bytes, idx, 0);
//             atoi_unroll!(d2, r2, bytes, idx, 1);
//             atoi_unroll!(d3, r3, bytes, idx, 2);
//             atoi_unroll!(d4, r4, bytes, idx, 3); 
            
//             result += r1 + r2 + r3 + r4;
//             len -= 4;

//             idx += 4;
//             bytes = &BYTE_SLICE[SLICE_LEN - len..];
//         }

//         for offset in 0..len {
//             atoi_unroll!(d, r, bytes, idx, offset);
//             result += r;
//         }
//         return Ok(result)
//     }
// }

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
        assert_eq!(u16::atoi("1234"), Ok(1234));
        assert_eq!(u64::atoi("123"), Ok(123));

        assert_eq!(u32::atoi("12"), Ok(12u32));
        assert_eq!(u16::atoi("1e3"), Err(()));
    }

    #[bench]
    fn rust_native_parse(b: &mut test::Bencher) {
        let s = ["123498", "987234", "890774", "982734", "123876", "10987", "84750"];
        b.iter(|| {
            for item in s.iter() {
                let _n = test::black_box(item.parse::<u64>());
            }
        })
    }
    #[bench]
    fn rust_atoi_parse(b: &mut test::Bencher) {
        let s = ["123498", "987234", "890774", "982734", "123876", "10987", "84750"];
        b.iter(|| {
            for item in s.iter() {
                let _n = test::black_box(rust_atoi(item));
            }
        })
    }

    #[bench]
    fn rust_safe_atoi_parse(b: &mut test::Bencher) {
        let s = ["123498", "987234", "890774", "982734", "123876", "10987", "84750"];
        b.iter(|| {
            for item in s.iter() {
                let _n = test::black_box(u64::atoi(item));
            }
        })
    }
}