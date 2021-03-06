#![feature(test)]
#[allow(non_snake_case)]

extern crate libc;
extern crate test;

use libc::uint64_t;

const MINUS: u8 = 48;

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
    fn c_atoi(b: *const u8, e: uint64_t, result: *mut uint64_t) -> uint64_t;

}

/// Returns the number of leading zero's of an unsigned 64-bit integer.
/// clzl of 0 is Undefined Behaviour, so return 64 instead.
/// 
/// # Examples
/// ```
/// extern crate c_bindings;
/// use c_bindings::crust_clzl;
/// 
/// fn main() {
///     assert_eq!(crust_clzl(1), 63);
///     assert_eq!(crust_clzl(2), 62);
///     assert_eq!(crust_clzl(8), 60);
///     assert_eq!(crust_clzl(std::u64::MAX), 0);
///     assert_eq!(crust_clzl(0), 64);
/// }
/// ```
#[inline]
pub fn crust_clzl(x: u64) -> u64 {
    
    //clzl of 0 is UB. return 64 instead, since there are 64 zero's.
    if x == 0 { return 64 }
    unsafe {
        c_clzl(x)
    }
}

/// Converts a `str` into a u64.
/// 
/// # Examples
/// ```
/// extern crate c_bindings;
/// use c_bindings::crust_atoi;
/// 
/// fn main() {
///     assert_eq!(crust_atoi("98765"), Ok(98765));
/// }
/// ```
pub fn crust_atoi(s: &str) -> Result<u64, ()> {
    let mut result = 0;
    unsafe {
        if c_atoi(s.as_ptr(), s.len() as u64, &mut result as *mut _) == 0 {
            Ok(result)
        }
        else {
            Err(())
        }
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
    ///     assert_eq!(u32::atoi("5432!"), Err(()));
    /// }
    /// ```
    fn atoi(s: &str) -> Result<Self, ()> where Self: Sized;
}

macro_rules! atoi_unroll {
    ($d:ident, $r:ident, $bytes:expr, $idx:expr, $offset:expr, $TABLE:ident) => (
        let $d = ($bytes.get_unchecked($offset).wrapping_sub(MINUS)) as Self;
        
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

                //Initialize all variables.
                let mut result: Self = 0;
                let mut bytes: &[u8] = s.as_bytes();
                let mut len: usize = s.len();
                let mut idx: usize = 20 - len;

                unsafe {

                    //as long as there are more than 4 digits,
                    //do this loop.
                    //lookup the digit of the current idx, convert to Self (u8, u16, u32...),
                    //check if valid,
                    //multiply with the correct power of 10 from the lookup table.
                    //update the result so far, the length, the idx.
                    //lastly update the slice, basically truncate the four digits that are just processed
                    while len >= 4 {
                        atoi_unroll!(d1, r1, bytes, idx, 0, $table_name);
                        atoi_unroll!(d2, r2, bytes, idx, 1, $table_name);
                        atoi_unroll!(d3, r3, bytes, idx, 2, $table_name);
                        atoi_unroll!(d4, r4, bytes, idx, 3, $table_name); 
                        
                        result += r1 + r2 + r3 + r4;
                        len -= 4;

                        idx += 4;
                        bytes = &bytes[4..];
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
    
    
//     let mut result = 0;
//     let mut bytes = s.as_bytes();
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
//             bytes = &bytes[4..];
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
    const s: [&str; 40] = [
        "123498", "987234", "890774", "982734", "123876", "10987", "84750", "97824", "12789367", "98274",
        "786349", "98726734", "897", "98723", "734", "1237", "975", "830", "93674", "10008",
        "1234", "97234", "870774", "988394", "153176", "50927", "85740", "17234", "17", "10",
        "286343", "78223798", "697", "72733", "764", "4137", "345", "530", "13274", "100865"
        ];
    #[test]
    fn test_crust_clzl() {
        assert_eq!(crust_clzl(1), 63);
        assert_eq!(crust_clzl(2), 62);
        assert_eq!(crust_clzl(8), 60);
        assert_eq!(crust_clzl(std::u64::MAX), 0);
        assert_eq!(crust_clzl(0), 64);
    }

    #[test]
    fn test_crust_atoi() {
        let st = "100";

        assert_eq!(crust_atoi(st), Ok(100));

        let ss = "1234";
        assert_eq!(crust_atoi(ss), Ok(1234));

        let sss = "4";
        assert_eq!(crust_atoi(sss), Ok(4));
    }

    #[test]
    fn test_rust_atoi() {
        assert_eq!(u32::atoi("987654"), Ok(987654));
        assert_eq!(u16::atoi("1234"), Ok(1234));
        assert_eq!(u64::atoi("123"), Ok(123));
        assert_eq!(u32::atoi("12"), Ok(12u32));
        assert_eq!(u16::atoi("1e3"), Err(()));
    }

    #[bench]
    fn rust_str_parse(b: &mut test::Bencher) {
        b.iter(|| {
            for item in s.iter() {
                let _n = match test::black_box(item.parse::<u64>()) {
                    Ok(parsed) => parsed,
                    Err(_) => panic!(),
                };
            }
        })
    }
    #[bench]
    fn c_atoi_parse(b: &mut test::Bencher) {
        b.iter(|| {
            for item in s.iter() {
                let _n = match test::black_box(crust_atoi(item)) {
                    Ok(parsed) => parsed,
                    Err(_) => panic!()
                };
            }
        })
    }

    #[bench]
    fn rust_atoi_parse(b: &mut test::Bencher) {
        b.iter(|| {
            for item in s.iter() {
                let _n = match test::black_box(u64::atoi(item)) {
                    Ok(parsed) => parsed,
                    Err(_) => panic!()
                };
            }
        })
    }
}