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
pub fn compress2_z(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
    level: ::core::ffi::c_int,
) -> (::core::ffi::c_int, crate::stdlib::z_size_t) {
    let mut stream: crate::zlib_h::z_stream = crate::zlib_h::z_stream {
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
    let mut err: ::core::ffi::c_int;
    let max: crate::stdlib::uInt = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut left = dest.len() as crate::stdlib::z_size_t;
    let dest_capacity = left;
    let mut source_left = source.len() as crate::stdlib::z_size_t;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = unsafe {
        crate::src::deflate::deflateInit_(
            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
            level,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        )
    };
    if err != crate::zlib_h::Z_OK {
        return (err, 0);
    }
    stream.next_out = dest.as_mut_ptr();
    stream.avail_out = 0 as crate::stdlib::uInt;
    stream.next_in = source.as_ptr() as *mut crate::stdlib::Bytef;
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
            stream.avail_in = if source_left > max as crate::stdlib::z_size_t {
                max
            } else {
                source_left as crate::stdlib::uInt
            };
            source_left = source_left.wrapping_sub(stream.avail_in as crate::stdlib::z_size_t);
        }
        err = unsafe {
            crate::src::deflate::deflate(
                &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
                if source_left != 0 {
                    crate::zlib_h::Z_NO_FLUSH
                } else {
                    crate::zlib_h::Z_FINISH
                },
            )
        };
        if err != crate::zlib_h::Z_OK {
            break;
        }
    }
    // `left` has not yet been assigned to the stream, while `avail_out` is
    // assigned but not written. Their complement is exactly the portion of
    // the write-only destination slice that the stream consumed.
    let written = dest_capacity
        .wrapping_sub(left)
        .wrapping_sub(stream.avail_out as crate::stdlib::z_size_t);
    unsafe {
        crate::src::deflate::deflateEnd(
            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
    }
    if err == crate::zlib_h::Z_STREAM_END {
        (crate::zlib_h::Z_OK, written)
    } else {
        (err, written)
    }
}
#[export_name = "compress2_z"]

pub unsafe extern "C" fn compress2_z_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::z_size_t,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::z_size_t,
    level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sourceLen != 0 && source.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = unsafe { *destLen };
    if dest_len != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // zlib makes this write before initializing deflate, including when the
    // requested level is invalid.
    unsafe { *destLen = 0 };
    let source = if sourceLen == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(source, sourceLen) }
    };
    let dest = if dest_len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(dest, dest_len) }
    };
    let (result, written) = compress2_z(dest, source, level);
    unsafe { *destLen = written };
    result
}
pub fn compress2(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
    level: ::core::ffi::c_int,
) -> (::core::ffi::c_int, crate::stdlib::uLongf) {
    let (result, written) = compress2_z(dest, source, level);
    (result, written as crate::stdlib::uLongf)
}
#[export_name = "compress2"]

pub unsafe extern "C" fn compress2_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::uLongf,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::uLong,
    level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sourceLen != 0 && source.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = unsafe { *destLen } as crate::stdlib::z_size_t;
    if dest_len != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    unsafe { *destLen = 0 };
    let source = if sourceLen == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t) }
    };
    let dest = if dest_len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(dest, dest_len) }
    };
    let (result, written) = compress2(dest, source, level);
    unsafe { *destLen = written };
    result
}
pub fn compress_z(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
) -> (::core::ffi::c_int, crate::stdlib::z_size_t) {
    compress2_z(dest, source, crate::zlib_h::Z_DEFAULT_COMPRESSION)
}
#[export_name = "compress_z"]

pub unsafe extern "C" fn compress_z_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::z_size_t,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if sourceLen != 0 && source.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = unsafe { *destLen };
    if dest_len != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    unsafe { *destLen = 0 };
    let source = if sourceLen == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(source, sourceLen) }
    };
    let dest = if dest_len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(dest, dest_len) }
    };
    let (result, written) = compress_z(dest, source);
    unsafe { *destLen = written };
    result
}
pub fn compress(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
) -> (::core::ffi::c_int, crate::stdlib::uLongf) {
    compress2(dest, source, crate::zlib_h::Z_DEFAULT_COMPRESSION)
}
#[export_name = "compress"]

pub unsafe extern "C" fn compress_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::uLongf,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if sourceLen != 0 && source.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = unsafe { *destLen } as crate::stdlib::z_size_t;
    if dest_len != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    unsafe { *destLen = 0 };
    let source = if sourceLen == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t) }
    };
    let dest = if dest_len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(dest, dest_len) }
    };
    let (result, written) = compress(dest, source);
    unsafe { *destLen = written };
    result
}
pub fn compressBound_z(source_len: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
    let bound = source_len
        .wrapping_add(source_len >> 12 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 14 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 25 as ::core::ffi::c_int)
        .wrapping_add(13 as crate::stdlib::z_size_t);
    if bound < source_len {
        -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
    } else {
        bound
    }
}
#[export_name = "compressBound_z"]

pub unsafe extern "C" fn compressBound_z_ffi(
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    compressBound_z(sourceLen)
}
pub fn compressBound(mut sourceLen: crate::stdlib::uLong) -> crate::stdlib::uLong {
    let mut bound: crate::stdlib::z_size_t = compressBound_z(sourceLen as crate::stdlib::z_size_t);
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
