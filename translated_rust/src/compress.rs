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
    destLen: &mut crate::stdlib::z_size_t,
    source: &[crate::stdlib::Bytef],
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if *destLen > dest.len() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mut stream: crate::zlib_h::z_stream = crate::zlib_h::z_stream {
        next_in: crate::zlib_h::InputBuffer::default(),
        avail_in: 0,
        total_in: 0,
        next_out: crate::zlib_h::OutputBuffer::default(),
        avail_out: 0,
        total_out: 0,
        msg: None,
        state: None,
        zalloc: None,
        zfree: None,
        opaque: crate::zlib_h::Opaque::default(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut err: ::core::ffi::c_int = 0;
    let max: crate::stdlib::uInt = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut left: crate::stdlib::z_size_t = *destLen;
    *destLen = 0 as crate::stdlib::z_size_t;
    let mut source_len = source.len() as crate::stdlib::z_size_t;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = crate::zlib_h::Opaque::default();
    err = crate::src::deflate::deflateInit_(&mut stream, level);
    if err != crate::zlib_h::Z_OK {
        return err;
    }
    stream.next_out = crate::output_cursor!(dest.as_mut_ptr());
    stream.avail_out = 0 as crate::stdlib::uInt;
    stream.next_in = crate::input_cursor!(source.as_ptr());
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
            stream.avail_in = if source_len > max as crate::stdlib::z_size_t {
                max
            } else {
                source_len as crate::stdlib::uInt
            };
            source_len = source_len.wrapping_sub(stream.avail_in as crate::stdlib::z_size_t);
        }
        let input_start = stream.total_in as usize;
        let output_start = stream.total_out as usize;
        let input_len = stream.avail_in as usize;
        let output_len = stream.avail_out as usize;
        err = crate::src::deflate::deflate_stream(
            &mut stream,
            &source[input_start..input_start + input_len],
            &mut dest[output_start..output_start + output_len],
            if source_len != 0 {
                crate::zlib_h::Z_NO_FLUSH
            } else {
                crate::zlib_h::Z_FINISH
            },
        );
        if err != crate::zlib_h::Z_OK {
            break;
        }
    }
    *destLen = stream.total_out as crate::stdlib::z_size_t;
    crate::src::deflate::deflateEnd(&mut stream);
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
    if destLen.is_null()
        || (sourceLen != 0 && source.is_null())
        || (*destLen != 0 && dest.is_null())
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = if *destLen == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, *destLen as usize)
    };
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen as usize)
    };
    compress2_z(dest, &mut *destLen, source, level)
}
pub fn compress2(
    dest: &mut [crate::stdlib::Bytef],
    destLen: &mut crate::stdlib::uLongf,
    source: &[crate::stdlib::Bytef],
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut got: crate::stdlib::z_size_t = *destLen as crate::stdlib::z_size_t;
    ret = compress2_z(dest, &mut got, source, level);
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
    if destLen.is_null()
        || (sourceLen != 0 && source.is_null())
        || (*destLen != 0 && dest.is_null())
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = if *destLen == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, *destLen as usize)
    };
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen as usize)
    };
    compress2(dest, &mut *destLen, source, level)
}
pub fn compress_z(
    dest: &mut [crate::stdlib::Bytef],
    destLen: &mut crate::stdlib::z_size_t,
    source: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    compress2_z(dest, destLen, source, crate::zlib_h::Z_DEFAULT_COMPRESSION)
}
#[export_name = "compress_z"]

pub unsafe extern "C" fn compress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if destLen.is_null()
        || (sourceLen != 0 && source.is_null())
        || (*destLen != 0 && dest.is_null())
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = if *destLen == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, *destLen as usize)
    };
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen as usize)
    };
    compress_z(dest, &mut *destLen, source)
}
pub fn compress(
    dest: &mut [crate::stdlib::Bytef],
    destLen: &mut crate::stdlib::uLongf,
    source: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    compress2(dest, destLen, source, crate::zlib_h::Z_DEFAULT_COMPRESSION)
}
#[export_name = "compress"]

pub unsafe extern "C" fn compress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if destLen.is_null()
        || (sourceLen != 0 && source.is_null())
        || (*destLen != 0 && dest.is_null())
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = if *destLen == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, *destLen as usize)
    };
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen as usize)
    };
    compress(dest, &mut *destLen, source)
}
pub unsafe extern "C" fn compressBound_z(
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut bound: crate::stdlib::z_size_t = sourceLen
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
pub unsafe extern "C" fn compressBound(
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
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
