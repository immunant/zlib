pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::deflateEnd;
pub use crate::src::deflate::deflateInit2_;
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

// The one-shot APIs feed z_stream in uInt-sized pieces.  Keep this cursor
// pointer-free so a later slice-backed stream facade can retain the exact
// chunking and progress accounting without carrying ABI pointers into its
// core.
pub(crate) struct OneShotCursor {
    remaining: crate::stdlib::z_size_t,
}

impl OneShotCursor {
    pub(crate) fn new(remaining: crate::stdlib::z_size_t) -> Self {
        Self { remaining }
    }

    pub(crate) fn remaining(&self) -> crate::stdlib::z_size_t {
        self.remaining
    }

    pub(crate) fn next_chunk(&mut self, max: crate::stdlib::uInt) -> crate::stdlib::uInt {
        let chunk = if self.remaining > max as crate::stdlib::z_size_t {
            max
        } else {
            self.remaining as crate::stdlib::uInt
        };
        self.remaining = self
            .remaining
            .wrapping_sub(chunk as crate::stdlib::z_size_t);
        chunk
    }
}

fn compress2_z(
    dest: &mut [crate::stdlib::Bytef],
    dest_len: &mut crate::stdlib::z_size_t,
    source: &[crate::stdlib::Bytef],
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *dest_len = 0 as crate::stdlib::z_size_t;
    let mut owner = crate::src::deflate::DeflateOneShotOwner::new(source, dest);
    let progress = crate::src::deflate::deflate_one_shot(&mut owner, level);
    *dest_len = progress.produced;
    progress.status
}
#[export_name = "compress2_z"]

pub unsafe extern "C" fn compress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sourceLen != 0 && source.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let capacity = *destLen;
    if capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen)
    };
    let dest = if capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, capacity)
    };
    compress2_z(dest, &mut *destLen, source, level)
}
fn compress2(
    dest: &mut [crate::stdlib::Bytef],
    dest_len: &mut crate::stdlib::uLongf,
    source: &[crate::stdlib::Bytef],
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut got: crate::stdlib::z_size_t = *dest_len as crate::stdlib::z_size_t;
    ret = compress2_z(dest, &mut got, source, level);
    *dest_len = got as crate::stdlib::uLong as crate::stdlib::uLongf;
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
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let capacity = *destLen as crate::stdlib::z_size_t;
    if sourceLen != 0 && source.is_null() || capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t)
    };
    let dest = if capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, capacity)
    };
    compress2(dest, &mut *destLen, source, level)
}
fn compress_z(
    dest: &mut [crate::stdlib::Bytef],
    dest_len: &mut crate::stdlib::z_size_t,
    source: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    return compress2_z(dest, dest_len, source, crate::zlib_h::Z_DEFAULT_COMPRESSION);
}
#[export_name = "compress_z"]

pub unsafe extern "C" fn compress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if sourceLen != 0 && source.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let capacity = *destLen;
    if capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen)
    };
    let dest = if capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, capacity)
    };
    compress_z(dest, &mut *destLen, source)
}
fn compress(
    dest: &mut [crate::stdlib::Bytef],
    dest_len: &mut crate::stdlib::uLongf,
    source: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    return compress2(dest, dest_len, source, crate::zlib_h::Z_DEFAULT_COMPRESSION);
}
#[export_name = "compress"]

pub unsafe extern "C" fn compress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let capacity = *destLen as crate::stdlib::z_size_t;
    if sourceLen != 0 && source.is_null() || capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t)
    };
    let dest = if capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, capacity)
    };
    compress(dest, &mut *destLen, source)
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
