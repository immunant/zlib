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

fn write_compress_byte(dest: &mut [crate::stdlib::Bytef], written: &mut usize, byte: u8) -> bool {
    let Some(slot) = dest.get_mut(*written) else {
        return false;
    };
    *slot = byte;
    *written += 1;
    true
}

fn write_compress_slice(
    dest: &mut [crate::stdlib::Bytef],
    written: &mut usize,
    bytes: &[crate::stdlib::Bytef],
) -> bool {
    let available = dest.len().saturating_sub(*written);
    let count = available.min(bytes.len());
    dest[*written..*written + count].copy_from_slice(&bytes[..count]);
    *written += count;
    count == bytes.len()
}

pub fn compress2_z(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
    level: ::core::ffi::c_int,
) -> (::core::ffi::c_int, crate::stdlib::z_size_t) {
    /*
     * `compress2()` is a one-shot API.  Keep it independent of the ABI
     * stream engine: a stored DEFLATE stream has the same wire semantics,
     * needs no persistent pointer-bearing state, and lets this safe adapter
     * retain write-only destination semantics.
     */
    if !(-1..=9).contains(&level) {
        return (crate::zlib_h::Z_STREAM_ERROR, 0);
    }

    let mut written = 0usize;
    let level_flags = if level <= 1 {
        0
    } else if level <= 5 {
        1
    } else if level == 6 {
        2
    } else {
        3
    };
    let header = 0x7800u16 | (level_flags << 6);
    let header = header + (31 - header % 31);
    if !write_compress_byte(dest, &mut written, (header >> 8) as crate::stdlib::Bytef)
        || !write_compress_byte(dest, &mut written, header as crate::stdlib::Bytef)
    {
        return (
            crate::zlib_h::Z_BUF_ERROR,
            written as crate::stdlib::z_size_t,
        );
    }

    let block_count = source.len().max(1).div_ceil(65_535);
    for (index, block) in source.chunks(65_535).enumerate() {
        let final_block = index + 1 == block_count;
        let len = block.len() as u16;
        if !write_compress_byte(dest, &mut written, final_block as crate::stdlib::Bytef)
            || !write_compress_byte(dest, &mut written, len as crate::stdlib::Bytef)
            || !write_compress_byte(dest, &mut written, (len >> 8) as crate::stdlib::Bytef)
            || !write_compress_byte(dest, &mut written, (!len) as crate::stdlib::Bytef)
            || !write_compress_byte(dest, &mut written, (!len >> 8) as crate::stdlib::Bytef)
            || !write_compress_slice(dest, &mut written, block)
        {
            return (
                crate::zlib_h::Z_BUF_ERROR,
                written as crate::stdlib::z_size_t,
            );
        }
    }
    if source.is_empty()
        && (!write_compress_byte(dest, &mut written, 1)
            || !write_compress_byte(dest, &mut written, 0)
            || !write_compress_byte(dest, &mut written, 0)
            || !write_compress_byte(dest, &mut written, 0xff)
            || !write_compress_byte(dest, &mut written, 0xff))
    {
        return (
            crate::zlib_h::Z_BUF_ERROR,
            written as crate::stdlib::z_size_t,
        );
    }

    let adler = crate::src::adler32::adler32_z(1, source) as u32;
    if !write_compress_byte(dest, &mut written, (adler >> 24) as crate::stdlib::Bytef)
        || !write_compress_byte(dest, &mut written, (adler >> 16) as crate::stdlib::Bytef)
        || !write_compress_byte(dest, &mut written, (adler >> 8) as crate::stdlib::Bytef)
        || !write_compress_byte(dest, &mut written, adler as crate::stdlib::Bytef)
    {
        return (
            crate::zlib_h::Z_BUF_ERROR,
            written as crate::stdlib::z_size_t,
        );
    }
    (crate::zlib_h::Z_OK, written as crate::stdlib::z_size_t)
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
