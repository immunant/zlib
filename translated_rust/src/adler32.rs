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

pub const BASE: ::core::ffi::c_uint = 65521 as ::core::ffi::c_uint;

pub const NMAX: ::core::ffi::c_int = 5552 as ::core::ffi::c_int;
pub unsafe extern "C" fn adler32_z(
    mut adler: crate::stdlib::uLong,
    mut buf: *const crate::stdlib::Bytef,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::uLong {
    let mut sum2: ::core::ffi::c_ulong = 0;
    let mut n: ::core::ffi::c_uint = 0;
    sum2 = (adler >> 16 as ::core::ffi::c_int & 0xffff as crate::stdlib::uLong)
        as ::core::ffi::c_ulong;
    adler &= 0xffff as crate::stdlib::uLong;
    if len == 1 as crate::stdlib::z_size_t {
        adler = adler
            .wrapping_add(*buf.offset(0 as ::core::ffi::c_int as isize) as crate::stdlib::uLong);
        if adler >= BASE as crate::stdlib::uLong {
            adler = adler.wrapping_sub(BASE as crate::stdlib::uLong);
        }
        sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
        if sum2 >= BASE as ::core::ffi::c_ulong {
            sum2 = sum2.wrapping_sub(BASE as ::core::ffi::c_ulong);
        }
        return adler | (sum2 as crate::stdlib::uLong) << 16 as ::core::ffi::c_int;
    }
    if buf.is_null() {
        return 1 as crate::stdlib::uLong;
    }
    if len < 16 as crate::stdlib::z_size_t {
        loop {
            let c2rust_fresh0 = len;
            len = len.wrapping_sub(1);
            if c2rust_fresh0 == 0 {
                break;
            }
            let c2rust_fresh1 = buf;
            buf = buf.offset(1);
            adler = adler.wrapping_add(*c2rust_fresh1 as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
        }
        if adler >= BASE as crate::stdlib::uLong {
            adler = adler.wrapping_sub(BASE as crate::stdlib::uLong);
        }
        sum2 = sum2.wrapping_rem(BASE as ::core::ffi::c_ulong);
        return adler | (sum2 as crate::stdlib::uLong) << 16 as ::core::ffi::c_int;
    }
    while len >= NMAX as crate::stdlib::z_size_t {
        len = len.wrapping_sub(NMAX as crate::stdlib::z_size_t);
        n = (NMAX / 16 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        loop {
            adler =
                adler.wrapping_add(
                    *buf.offset(0 as ::core::ffi::c_int as isize) as crate::stdlib::uLong
                );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler =
                adler.wrapping_add(
                    *buf.offset(8 as ::core::ffi::c_int as isize) as crate::stdlib::uLong
                );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            buf = buf.offset(16 as ::core::ffi::c_int as isize);
            n = n.wrapping_sub(1);
            if n == 0 {
                break;
            }
        }
        adler = adler.wrapping_rem(BASE as crate::stdlib::uLong);
        sum2 = sum2.wrapping_rem(BASE as ::core::ffi::c_ulong);
    }
    if len != 0 {
        while len >= 16 as crate::stdlib::z_size_t {
            len = len.wrapping_sub(16 as crate::stdlib::z_size_t);
            adler =
                adler.wrapping_add(
                    *buf.offset(0 as ::core::ffi::c_int as isize) as crate::stdlib::uLong
                );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler =
                adler.wrapping_add(
                    *buf.offset(8 as ::core::ffi::c_int as isize) as crate::stdlib::uLong
                );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::uLong,
            );
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = adler.wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ) as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            buf = buf.offset(16 as ::core::ffi::c_int as isize);
        }
        loop {
            let c2rust_fresh2 = len;
            len = len.wrapping_sub(1);
            if c2rust_fresh2 == 0 {
                break;
            }
            let c2rust_fresh3 = buf;
            buf = buf.offset(1);
            adler = adler.wrapping_add(*c2rust_fresh3 as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
        }
        adler = adler.wrapping_rem(BASE as crate::stdlib::uLong);
        sum2 = sum2.wrapping_rem(BASE as ::core::ffi::c_ulong);
    }
    return adler | (sum2 as crate::stdlib::uLong) << 16 as ::core::ffi::c_int;
}
#[export_name = "adler32_z"]

pub unsafe extern "C" fn adler32_z_ffi(
    mut adler: crate::stdlib::uLong,
    mut buf: *const crate::stdlib::Bytef,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::uLong {
    adler32_z(adler, buf, len)
}
pub unsafe extern "C" fn adler32(
    mut adler: crate::stdlib::uLong,
    mut buf: *const crate::stdlib::Bytef,
    mut len: crate::stdlib::uInt,
) -> crate::stdlib::uLong {
    return adler32_z(adler, buf, len as crate::stdlib::z_size_t);
}
#[export_name = "adler32"]

pub unsafe extern "C" fn adler32_ffi(
    mut adler: crate::stdlib::uLong,
    mut buf: *const crate::stdlib::Bytef,
    mut len: crate::stdlib::uInt,
) -> crate::stdlib::uLong {
    adler32(adler, buf, len)
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
    return sum1 as crate::stdlib::uLong
        | (sum2 as crate::stdlib::uLong) << 16 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn adler32_combine(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off_t,
) -> crate::stdlib::uLong {
    return adler32_combine_(adler1, adler2, len2 as crate::stdlib::off64_t);
}
#[export_name = "adler32_combine"]

pub unsafe extern "C" fn adler32_combine_ffi(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off_t,
) -> crate::stdlib::uLong {
    adler32_combine(adler1, adler2, len2)
}
pub unsafe extern "C" fn adler32_combine64(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off64_t,
) -> crate::stdlib::uLong {
    return adler32_combine_(adler1, adler2, len2);
}
#[export_name = "adler32_combine64"]

pub unsafe extern "C" fn adler32_combine64_ffi(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off64_t,
) -> crate::stdlib::uLong {
    adler32_combine64(adler1, adler2, len2)
}
