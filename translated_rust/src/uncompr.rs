pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate;
pub use crate::src::inflate::inflateEnd;
pub use crate::src::inflate::inflateInit_;
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
pub fn uncompress2_z(
    mut dest: Option<&mut [crate::stdlib::Bytef]>,
    source: Option<&[crate::stdlib::Bytef]>,
) -> (
    ::core::ffi::c_int,
    crate::stdlib::z_size_t,
    crate::stdlib::z_size_t,
) {
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
    let mut len = source.as_ref().map_or(0, |source| source.len()) as crate::stdlib::z_size_t;
    let mut left = dest.as_ref().map_or(0, |dest| dest.len()) as crate::stdlib::z_size_t;
    stream.next_in = source.map_or(::core::ptr::null_mut(), |source| {
        source.as_ptr() as *mut crate::stdlib::Bytef
    });
    stream.avail_in = 0 as crate::stdlib::uInt;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = crate::src::inflate::inflateInit_(
        Some(&mut stream),
        Some(&crate::zlib_h::ZLIB_VERSION[0]),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        crate::src::inflate::InflateInitMode::Zlib,
    );
    if err != crate::zlib_h::Z_OK {
        return (err, 0, 0);
    }
    stream.next_out = dest.as_deref_mut().map_or(
        &raw mut stream.reserved as *mut crate::stdlib::Bytef,
        |dest| dest.as_mut_ptr(),
    );
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
        err = crate::src::inflate::inflate(&mut stream, crate::zlib_h::Z_NO_FLUSH);
        if err != crate::zlib_h::Z_OK {
            break;
        }
    }
    len = len.wrapping_add(stream.avail_in as crate::stdlib::z_size_t);
    left = left.wrapping_add(stream.avail_out as crate::stdlib::z_size_t);
    let used = source.as_ref().map_or(0, |source| source.len()) as crate::stdlib::z_size_t;
    let written = dest.as_ref().map_or(0, |dest| dest.len()) as crate::stdlib::z_size_t;
    let used = used.wrapping_sub(len);
    let written = written.wrapping_sub(left);
    crate::src::inflate::inflateEnd(&mut stream);
    return (
        if err == crate::zlib_h::Z_STREAM_END {
            crate::zlib_h::Z_OK
    } else if err == crate::zlib_h::Z_NEED_DICT {
        crate::zlib_h::Z_DATA_ERROR
    } else if err == crate::zlib_h::Z_BUF_ERROR && len == 0 as crate::stdlib::z_size_t {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        err
        },
        used,
        written,
    );
}
#[export_name = "uncompress2_z"]

pub unsafe extern "C" fn uncompress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if destLen.is_null() || sourceLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = *destLen;
    let source_len = *sourceLen;
    if dest_len != 0 && dest.is_null() || source_len != 0 && source.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = (!dest.is_null()).then(|| ::core::slice::from_raw_parts_mut(dest, dest_len));
    let source = (!source.is_null()).then(|| ::core::slice::from_raw_parts(source, source_len));
    let (ret, used, written) = uncompress2_z(dest, source);
    *sourceLen = used;
    *destLen = written;
    ret
}
pub fn uncompress2(
    dest: Option<&mut [crate::stdlib::Bytef]>,
    source: Option<&[crate::stdlib::Bytef]>,
) -> (
    ::core::ffi::c_int,
    crate::stdlib::uLong,
    crate::stdlib::uLongf,
) {
    let (ret, used, written) = uncompress2_z(dest, source);
    (ret, used as crate::stdlib::uLong, written as crate::stdlib::uLong as crate::stdlib::uLongf)
}
#[export_name = "uncompress2"]

pub unsafe extern "C" fn uncompress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if destLen.is_null() || sourceLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = *destLen;
    let source_len = *sourceLen;
    if dest_len != 0 && dest.is_null() || source_len != 0 && source.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = (!dest.is_null()).then(|| {
        ::core::slice::from_raw_parts_mut(dest, dest_len as crate::stdlib::z_size_t)
    });
    let source = (!source.is_null()).then(|| {
        ::core::slice::from_raw_parts(source, source_len as crate::stdlib::z_size_t)
    });
    let (ret, used, written) = uncompress2(dest, source);
    *sourceLen = used;
    *destLen = written;
    ret
}
#[export_name = "uncompress_z"]

pub unsafe extern "C" fn uncompress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = *destLen;
    if dest_len != 0 && dest.is_null() || sourceLen != 0 && source.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = (!dest.is_null()).then(|| ::core::slice::from_raw_parts_mut(dest, dest_len));
    let source = (!source.is_null()).then(|| ::core::slice::from_raw_parts(source, sourceLen));
    let (ret, _, written) = uncompress2_z(dest, source);
    *destLen = written;
    ret
}
#[export_name = "uncompress"]

pub unsafe extern "C" fn uncompress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = *destLen;
    if dest_len != 0 && dest.is_null() || sourceLen != 0 && source.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = (!dest.is_null()).then(|| {
        ::core::slice::from_raw_parts_mut(dest, dest_len as crate::stdlib::z_size_t)
    });
    let source = (!source.is_null()).then(|| {
        ::core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t)
    });
    let (ret, _, written) = uncompress2(dest, source);
    *destLen = written;
    ret
}
