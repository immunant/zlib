pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::deflateEnd_ffi;
pub use crate::src::deflate::deflate_ffi;
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

fn compress_chunk(remaining: &mut crate::stdlib::z_size_t) -> crate::stdlib::uInt {
    let max = crate::stdlib::uInt::MAX as crate::stdlib::z_size_t;
    let chunk = if *remaining > max {
        crate::stdlib::uInt::MAX
    } else {
        *remaining as crate::stdlib::uInt
    };
    *remaining = (*remaining).wrapping_sub(chunk as crate::stdlib::z_size_t);
    chunk
}

fn compress2_final_status(err: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if err == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else {
        err
    }
}

fn compress2_produced(
    capacity: crate::stdlib::z_size_t,
    unissued: crate::stdlib::z_size_t,
    available: crate::stdlib::uInt,
) -> crate::stdlib::z_size_t {
    capacity.wrapping_sub(unissued.wrapping_add(available as crate::stdlib::z_size_t))
}

fn compress2_arg_capacity(
    source_len: crate::stdlib::z_size_t,
    source_is_null: bool,
    dest_capacity: Option<crate::stdlib::z_size_t>,
    dest_is_null: bool,
) -> Option<crate::stdlib::z_size_t> {
    let dest_capacity = dest_capacity?;
    if source_len > 0 as crate::stdlib::z_size_t && source_is_null
        || dest_capacity > 0 as crate::stdlib::z_size_t && dest_is_null
    {
        None
    } else {
        Some(dest_capacity)
    }
}

#[export_name = "compress2_z"]
pub unsafe extern "C" fn compress2_z_ffi(
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
    let mut capacity: crate::stdlib::z_size_t = 0;
    let mut left: crate::stdlib::z_size_t = 0;
    let dest_capacity = if destLen.is_null() {
        None
    } else {
        Some(*destLen)
    };
    let Some(planned_capacity) =
        compress2_arg_capacity(sourceLen, source.is_null(), dest_capacity, dest.is_null())
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    left = planned_capacity;
    capacity = left;
    *destLen = 0 as crate::stdlib::z_size_t;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = crate::src::deflate::deflateInit__ffi(
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
            stream.avail_out = compress_chunk(&mut left);
        }
        if stream.avail_in == 0 as crate::stdlib::uInt {
            stream.avail_in = compress_chunk(&mut sourceLen);
        }
        err = crate::src::deflate::deflate_ffi(
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
    *destLen = compress2_produced(capacity, left, stream.avail_out);
    crate::src::deflate::deflateEnd_ffi(
        &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
    );
    return compress2_final_status(err);
}
#[export_name = "compress2"]
pub unsafe extern "C" fn compress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut got: crate::stdlib::z_size_t = *destLen as crate::stdlib::z_size_t;
    ret = compress2_z_ffi(
        dest,
        &raw mut got,
        source,
        sourceLen as crate::stdlib::z_size_t,
        level,
    );
    *destLen = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    return ret;
}
#[export_name = "compress_z"]
pub unsafe extern "C" fn compress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    return compress2_z_ffi(
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
    return compress2_ffi(
        dest,
        destLen,
        source,
        sourceLen,
        crate::zlib_h::Z_DEFAULT_COMPRESSION,
    );
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
