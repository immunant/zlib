pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate;
pub use crate::src::inflate::inflateEnd_ffi;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::uLongf;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_NEED_DICT;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
#[export_name = "uncompress2_z"]
pub unsafe extern "C" fn uncompress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    let mut stream: crate::zlib_h::z_stream = crate::zlib_h::z_stream_s {
        next_in: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        state: ::core::ptr::null_mut::<crate::src::deflate::internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut err: ::core::ffi::c_int = 0;
    let max: crate::stdlib::uInt = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut len: crate::stdlib::z_size_t = 0;
    let mut left: crate::stdlib::z_size_t = 0;
    if sourceLen.is_null()
        || *sourceLen > 0 as crate::stdlib::z_size_t && source.is_null()
        || destLen.is_null()
        || *destLen > 0 as crate::stdlib::z_size_t && dest.is_null()
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    len = *sourceLen;
    left = *destLen;
    if left == 0 as crate::stdlib::z_size_t && dest.is_null() {
        dest = &raw mut stream.reserved as *mut crate::stdlib::Bytef;
    }
    stream.next_in = source as *mut crate::stdlib::Bytef;
    stream.avail_in = 0 as crate::stdlib::uInt;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = crate::src::inflate::inflateInit__ffi(
        &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
        crate::zlib_h::ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
    );
    if err != crate::zlib_h::Z_OK {
        return err;
    }
    stream.next_out = dest;
    stream.avail_out = 0 as crate::stdlib::uInt;
    loop {
        if stream.avail_out == 0 as crate::stdlib::uInt {
            stream.avail_out = if left > max as crate::stdlib::z_size_t {
                max
            } else {
                left as crate::stdlib::uInt
            };
            left = left.wrapping_sub(stream.avail_out as crate::stdlib::z_size_t);
        }
        if stream.avail_in == 0 as crate::stdlib::uInt {
            stream.avail_in = if len > max as crate::stdlib::z_size_t {
                max
            } else {
                len as crate::stdlib::uInt
            };
            len = len.wrapping_sub(stream.avail_in as crate::stdlib::z_size_t);
        }
        err = crate::src::inflate::inflate(
            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
            crate::zlib_h::Z_NO_FLUSH,
        );
        if !(err == crate::zlib_h::Z_OK) {
            break;
        }
    }
    len = len.wrapping_add(stream.avail_in as crate::stdlib::z_size_t);
    left = left.wrapping_add(stream.avail_out as crate::stdlib::z_size_t);
    *sourceLen = (*sourceLen).wrapping_sub(len);
    *destLen = (*destLen).wrapping_sub(left);
    crate::src::inflate::inflateEnd_ffi(
        &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
    );
    return if err == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else if err == crate::zlib_h::Z_NEED_DICT {
        crate::zlib_h::Z_DATA_ERROR
    } else if err == crate::zlib_h::Z_BUF_ERROR && len == 0 as crate::stdlib::z_size_t {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        err
    };
}
#[export_name = "uncompress2"]
pub unsafe extern "C" fn uncompress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut got: crate::stdlib::z_size_t = *destLen as crate::stdlib::z_size_t;
    let mut used: crate::stdlib::z_size_t = *sourceLen as crate::stdlib::z_size_t;
    ret = uncompress2_z_ffi(dest, &raw mut got, source, &raw mut used);
    *sourceLen = used as crate::stdlib::uLong;
    *destLen = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    return ret;
}
#[export_name = "uncompress_z"]
pub unsafe extern "C" fn uncompress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    let mut used: crate::stdlib::z_size_t = sourceLen;
    return uncompress2_z_ffi(dest, destLen, source, &raw mut used);
}
#[export_name = "uncompress"]
pub unsafe extern "C" fn uncompress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let mut used: crate::stdlib::uLong = sourceLen;
    return uncompress2_ffi(dest, destLen, source, &raw mut used);
}
