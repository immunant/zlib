pub use crate::__stddef_size_t_h::size_t;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off64_t;
pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::Z_NULL;

pub const BASE: ::core::ffi::c_uint = 65521;
pub const NMAX: ::core::ffi::c_int = 5552;

pub fn adler32_z(
    mut adler: crate::stdlib::uLong,
    buf: Option<&[crate::stdlib::Bytef]>,
) -> crate::stdlib::uLong {
    let Some(buf) = buf else {
        return 1;
    };
    let base = BASE as crate::stdlib::uLong;
    let mut sum2 = (adler >> 16) & 0xffff;
    adler &= 0xffff;
    if buf.len() == 1 {
        adler = adler.wrapping_add(buf[0] as crate::stdlib::uLong);
        if adler >= base {
            adler = adler.wrapping_sub(base);
        }
        sum2 = sum2.wrapping_add(adler);
        if sum2 >= base {
            sum2 = sum2.wrapping_sub(base);
        }
        return adler | sum2 << 16;
    }
    for block in buf.chunks(NMAX as usize) {
        for &byte in block {
            adler = adler.wrapping_add(byte as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler);
        }
        adler %= base;
        sum2 %= base;
    }
    adler | sum2 << 16
}

pub fn adler32(
    adler: crate::stdlib::uLong,
    buf: Option<&[crate::stdlib::Bytef]>,
) -> crate::stdlib::uLong {
    adler32_z(adler, buf)
}

#[export_name = "adler32_z"]
pub unsafe extern "C" fn adler32_z_ffi(
    mut adler: crate::stdlib::uLong,
    mut buf: *const crate::stdlib::Bytef,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::uLong {
    if buf.is_null() {
        adler32_z(adler, None)
    } else {
        adler32_z(
            adler,
            Some(unsafe { ::core::slice::from_raw_parts(buf, len) }),
        )
    }
}

#[export_name = "adler32"]
pub unsafe extern "C" fn adler32_ffi(
    mut adler: crate::stdlib::uLong,
    mut buf: *const crate::stdlib::Bytef,
    mut len: crate::stdlib::uInt,
) -> crate::stdlib::uLong {
    if buf.is_null() {
        adler32(adler, None)
    } else {
        adler32(
            adler,
            Some(unsafe { ::core::slice::from_raw_parts(buf, len as crate::stdlib::z_size_t) }),
        )
    }
}

fn adler32_combine_(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off64_t,
) -> crate::stdlib::uLong {
    let mut sum1: ::core::ffi::c_ulong = 0;
    let mut sum2: ::core::ffi::c_ulong = 0;
    let mut rem: ::core::ffi::c_uint = 0;
    if len2 < 0 as crate::stdlib::off64_t {
        return 0xffffffff as crate::stdlib::uLong;
    }
    len2 %= BASE as crate::stdlib::off64_t;
    rem = len2 as ::core::ffi::c_uint;
    sum1 = (adler1 & 0xffff as crate::stdlib::uLong) as ::core::ffi::c_ulong;
    sum2 = (rem as ::core::ffi::c_ulong).wrapping_mul(sum1);
    sum2 = sum2.wrapping_rem(BASE as ::core::ffi::c_ulong);
    sum1 = sum1.wrapping_add(
        (adler2 & 0xffff as crate::stdlib::uLong)
            .wrapping_add(BASE as crate::stdlib::uLong)
            .wrapping_sub(1 as crate::stdlib::uLong) as ::core::ffi::c_ulong,
    );
    sum2 = sum2.wrapping_add(
        (adler1 >> 16 as ::core::ffi::c_int & 0xffff as crate::stdlib::uLong)
            .wrapping_add(adler2 >> 16 as ::core::ffi::c_int & 0xffff as crate::stdlib::uLong)
            .wrapping_add(BASE as crate::stdlib::uLong)
            .wrapping_sub(rem as crate::stdlib::uLong) as ::core::ffi::c_ulong,
    );
    if sum1 >= BASE as ::core::ffi::c_ulong {
        sum1 = sum1.wrapping_sub(BASE as ::core::ffi::c_ulong);
    }
    if sum1 >= BASE as ::core::ffi::c_ulong {
        sum1 = sum1.wrapping_sub(BASE as ::core::ffi::c_ulong);
    }
    if sum2 >= (BASE as ::core::ffi::c_ulong) << 1 as ::core::ffi::c_int {
        sum2 = sum2.wrapping_sub((BASE as ::core::ffi::c_ulong) << 1 as ::core::ffi::c_int);
    }
    if sum2 >= BASE as ::core::ffi::c_ulong {
        sum2 = sum2.wrapping_sub(BASE as ::core::ffi::c_ulong);
    }
    sum1 as crate::stdlib::uLong | (sum2 as crate::stdlib::uLong) << 16 as ::core::ffi::c_int
}

pub fn adler32_combine(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off_t,
) -> crate::stdlib::uLong {
    adler32_combine_(adler1, adler2, len2 as crate::stdlib::off64_t)
}

#[export_name = "adler32_combine"]
pub unsafe extern "C" fn adler32_combine_ffi(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off_t,
) -> crate::stdlib::uLong {
    adler32_combine(adler1, adler2, len2)
}

pub fn adler32_combine64(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off64_t,
) -> crate::stdlib::uLong {
    adler32_combine_(adler1, adler2, len2)
}

#[export_name = "adler32_combine64"]
pub unsafe extern "C" fn adler32_combine64_ffi(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off64_t,
) -> crate::stdlib::uLong {
    adler32_combine64(adler1, adler2, len2)
}
