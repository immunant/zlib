pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::deflate;
pub use crate::src::deflate::deflateEnd;
pub use crate::src::deflate::deflateInit_;
pub use crate::src::deflate::internal_state;
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
pub use crate::zlib_h::Z_DEFAULT_COMPRESSION;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
pub unsafe extern "C" fn compress2_z(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
    mut level: ::core::ffi::c_int,
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
    let mut left: crate::stdlib::z_size_t = 0;
    if sourceLen > 0 as crate::stdlib::z_size_t && source.is_null()
        || destLen.is_null()
        || *destLen > 0 as crate::stdlib::z_size_t && dest.is_null()
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    left = *destLen;
    *destLen = 0 as crate::stdlib::z_size_t;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = crate::src::deflate::deflateInit_(
        &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
        level,
        crate::zlib_h::ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
    );
    if err != crate::zlib_h::Z_OK {
        return err;
    }
    stream.next_out = dest;
    stream.avail_out = 0 as crate::stdlib::uInt;
    stream.next_in = source as *mut crate::stdlib::Bytef;
    stream.avail_in = 0 as crate::stdlib::uInt;
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
            stream.avail_in = if sourceLen > max as crate::stdlib::z_size_t {
                max
            } else {
                sourceLen as crate::stdlib::uInt
            };
            sourceLen = sourceLen.wrapping_sub(stream.avail_in as crate::stdlib::z_size_t);
        }
        err = crate::src::deflate::deflate(
            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
            if sourceLen != 0 {
                crate::zlib_h::Z_NO_FLUSH
            } else {
                crate::zlib_h::Z_FINISH
            },
        );
        if !(err == crate::zlib_h::Z_OK) {
            break;
        }
    }
    *destLen = stream.next_out.offset_from(dest) as ::core::ffi::c_long as crate::stdlib::z_size_t;
    crate::src::deflate::deflateEnd(&raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s);
    return if err == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else {
        err
    };
}
#[export_name = "compress2_z"]

pub unsafe extern "C" fn compress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    compress2_z(dest, destLen, source, sourceLen, level)
}
pub unsafe extern "C" fn compress2(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut got: crate::stdlib::z_size_t = *destLen as crate::stdlib::z_size_t;
    ret = compress2_z(
        dest,
        &raw mut got,
        source,
        sourceLen as crate::stdlib::z_size_t,
        level,
    );
    *destLen = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    return ret;
}
#[export_name = "compress2"]

pub unsafe extern "C" fn compress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    compress2(dest, destLen, source, sourceLen, level)
}
pub unsafe extern "C" fn compress_z(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    return compress2_z(
        dest,
        destLen,
        source,
        sourceLen,
        crate::zlib_h::Z_DEFAULT_COMPRESSION,
    );
}
#[export_name = "compress_z"]

pub unsafe extern "C" fn compress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    compress_z(dest, destLen, source, sourceLen)
}
pub unsafe extern "C" fn compress(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    return compress2(
        dest,
        destLen,
        source,
        sourceLen,
        crate::zlib_h::Z_DEFAULT_COMPRESSION,
    );
}
#[export_name = "compress"]

pub unsafe extern "C" fn compress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    compress(dest, destLen, source, sourceLen)
}
pub fn compressBound_z(sourceLen: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
    let bound: crate::stdlib::z_size_t = sourceLen
        .wrapping_add(sourceLen >> 12 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 14 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 25 as ::core::ffi::c_int)
        .wrapping_add(13 as crate::stdlib::z_size_t);
    return if bound < sourceLen {
        -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
    } else {
        bound
    };
}
#[export_name = "compressBound_z"]

pub unsafe extern "C" fn compressBound_z_ffi(
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    compressBound_z(sourceLen)
}
pub fn compressBound(sourceLen: crate::stdlib::uLong) -> crate::stdlib::uLong {
    let bound: crate::stdlib::z_size_t = compressBound_z(sourceLen as crate::stdlib::z_size_t);
    return if bound != bound {
        -1 as ::core::ffi::c_int as crate::stdlib::uLong
    } else {
        bound as crate::stdlib::uLong
    };
}
#[export_name = "compressBound"]

pub unsafe extern "C" fn compressBound_ffi(
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    compressBound(sourceLen)
}
