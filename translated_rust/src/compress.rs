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
    mut dest: Option<&mut [crate::stdlib::Bytef]>,
    source: Option<&[crate::stdlib::Bytef]>,
    mut level: ::core::ffi::c_int,
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
    let mut err: ::core::ffi::c_int = 0;
    let max: crate::stdlib::uInt = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut left = dest.as_ref().map_or(0, |dest| dest.len()) as crate::stdlib::z_size_t;
    let mut written: crate::stdlib::z_size_t = 0;
    let mut source_len =
        source.as_ref().map_or(0, |source| source.len()) as crate::stdlib::z_size_t;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = crate::src::deflate::deflateInit_(
        Some(&mut stream),
        level,
        Some(&crate::zlib_h::ZLIB_VERSION[0]),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        crate::src::deflate::DeflateInitMode::Zlib,
    );
    if err != crate::zlib_h::Z_OK {
        return (err, 0);
    }
    stream.next_out = dest
        .as_ref()
        .map_or(::core::ptr::null_mut(), |dest| dest.as_ptr().cast_mut());
    stream.avail_out = 0 as crate::stdlib::uInt;
    stream.next_in = source.map_or(::core::ptr::null_mut(), |source| {
        source.as_ptr() as *mut crate::stdlib::Bytef
    });
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
        let avail_out = stream.avail_out;
        // `stream` is initialized above and its input/output cursors are
        // derived only from the borrowed slices retained for this call.
        let input = if stream.avail_in == 0 {
            None
        } else {
            source.and_then(|source| {
                usize::try_from(source_len)
                    .ok()
                    .and_then(|remaining| source.len().checked_sub(remaining))
                    .and_then(|start| start.checked_sub(stream.avail_in as usize))
                    .and_then(|start| {
                        start
                            .checked_add(stream.avail_in as usize)
                            .and_then(|end| source.get(start..end))
                })
            })
        };
        let output = dest.as_deref_mut().and_then(|dest| {
            let start = stream.next_out.addr().checked_sub(dest.as_ptr().addr())?;
            let end = start.checked_add(stream.avail_out as usize)?;
            dest.get_mut(start..end)
        });
        err = crate::src::deflate::deflate(
            &mut stream,
            if source_len != 0 {
                crate::zlib_h::Z_NO_FLUSH
            } else {
                crate::zlib_h::Z_FINISH
            },
            input,
            output,
        );
        written = written
            .wrapping_add(avail_out.wrapping_sub(stream.avail_out) as crate::stdlib::z_size_t);
        if err != crate::zlib_h::Z_OK {
            break;
        }
    }
    // The stream is initialized successfully before the loop, so it has the
    // lifecycle required by the validated cleanup routine.
    crate::src::deflate::deflateEnd(&mut stream);
    return (
        if err == crate::zlib_h::Z_STREAM_END {
            crate::zlib_h::Z_OK
        } else {
            err
        },
        written,
    );
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
        || sourceLen != 0 && source.is_null()
        || unsafe { *destLen != 0 } && dest.is_null()
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = if dest.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts_mut(dest, *destLen) })
    };
    let source = if source.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts(source, sourceLen) })
    };
    let (ret, written) = compress2_z(dest, source, level);
    *destLen = written;
    ret
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
        || sourceLen != 0 && source.is_null()
        || unsafe { *destLen != 0 } && dest.is_null()
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = if dest.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts_mut(dest, *destLen as usize) })
    };
    let source = if source.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts(source, sourceLen as usize) })
    };
    let (ret, written) = compress2_z(dest, source, level);
    *destLen = written as crate::stdlib::uLong as crate::stdlib::uLongf;
    ret
}
#[export_name = "compress_z"]

pub unsafe extern "C" fn compress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if destLen.is_null()
        || sourceLen != 0 && source.is_null()
        || unsafe { *destLen != 0 } && dest.is_null()
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = if dest.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts_mut(dest, *destLen) })
    };
    let source = if source.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts(source, sourceLen) })
    };
    let (ret, written) = compress2_z(dest, source, crate::zlib_h::Z_DEFAULT_COMPRESSION);
    *destLen = written;
    ret
}
#[export_name = "compress"]

pub unsafe extern "C" fn compress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if destLen.is_null()
        || sourceLen != 0 && source.is_null()
        || unsafe { *destLen != 0 } && dest.is_null()
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = if dest.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts_mut(dest, *destLen as usize) })
    };
    let source = if source.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts(source, sourceLen as usize) })
    };
    let (ret, written) = compress2_z(dest, source, crate::zlib_h::Z_DEFAULT_COMPRESSION);
    *destLen = written as crate::stdlib::uLong as crate::stdlib::uLongf;
    ret
}
pub fn compressBound_z(mut sourceLen: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
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
