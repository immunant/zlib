pub use crate::stdlib::uLong;
pub use crate::stdlib::uLongf;
pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_ERROR;

/// The result of a bounded zlib decompression operation.  The byte counts are
/// the same values that `uncompress2_z()` exposes through its length pointers.
#[derive(Copy, Clone)]
pub struct UncompressResult {
    pub status: ::core::ffi::c_int,
    pub source_len: usize,
    pub dest_len: usize,
}

/// Decompress a zlib-wrapped stream into `dest` without exposing the caller's
/// buffers as pointers to the implementation.  This is the implementation
/// behind the C ABI functions below.
pub fn uncompress2_z(dest: &mut [Bytef], source: &[Bytef]) -> UncompressResult {
    use miniz_oxide::inflate::stream::{inflate, InflateState};
    use miniz_oxide::{DataFormat, MZError, MZFlush, MZStatus};

    let mut state = InflateState::new_boxed(DataFormat::Zlib);
    let mut source_len = 0usize;
    let mut dest_len = 0usize;

    loop {
        let result = inflate(
            &mut state,
            &source[source_len..],
            &mut dest[dest_len..],
            MZFlush::None,
        );
        source_len += result.bytes_consumed;
        dest_len += result.bytes_written;

        let status = match result.status {
            Ok(MZStatus::StreamEnd) => Z_OK,
            Ok(MZStatus::Ok) => {
                if dest_len == dest.len() || source_len == source.len() {
                    Z_BUF_ERROR
                } else {
                    // A successful inflater call must make progress unless a
                    // source or destination bound stopped it.
                    Z_STREAM_ERROR
                }
            }
            Ok(_) => Z_STREAM_ERROR,
            Err(MZError::Data) => Z_DATA_ERROR,
            Err(MZError::Buf) => Z_BUF_ERROR,
            Err(_) => Z_STREAM_ERROR,
        };

        if status != Z_STREAM_ERROR || result.bytes_consumed == 0 && result.bytes_written == 0 {
            return UncompressResult {
                // The safe streaming decoder can retain up to a dictionary's
                // worth of output, and consequently consume all available
                // input before it reports a full caller destination.  A full
                // destination is still zlib's buffer error, not truncation.
                status: if status == Z_BUF_ERROR
                    && source_len == source.len()
                    && dest_len < dest.len()
                {
                    Z_DATA_ERROR
                } else {
                    status
                },
                source_len,
                dest_len,
            };
        }
    }
}

/// Safe `uLong` compatibility form of [`uncompress2_z`].
pub fn uncompress2(dest: &mut [Bytef], source: &[Bytef]) -> UncompressResult {
    uncompress2_z(dest, source)
}

/// Safe `z_size_t` compatibility form of [`uncompress2_z`].
pub fn uncompress_z(dest: &mut [Bytef], source: &[Bytef]) -> UncompressResult {
    uncompress2_z(dest, source)
}

/// Safe `uLong` compatibility form of [`uncompress2_z`] that discards the
/// number of input bytes consumed, as the C `uncompress()` API does.
pub fn uncompress(dest: &mut [Bytef], source: &[Bytef]) -> UncompressResult {
    uncompress2_z(dest, source)
}

#[export_name = "uncompress2_z"]
pub unsafe extern "C" fn uncompress2_z_ffi(
    dest: *mut Bytef,
    dest_len: *mut z_size_t,
    source: *const Bytef,
    source_len: *mut z_size_t,
) -> ::core::ffi::c_int {
    if dest_len.is_null() || source_len.is_null() {
        return Z_STREAM_ERROR;
    }
    let dest_len_value = *dest_len;
    let source_len_value = *source_len;
    if dest.is_null() && dest_len_value != 0 {
        return Z_STREAM_ERROR;
    }
    if source.is_null() && source_len_value != 0 {
        return Z_STREAM_ERROR;
    }
    let dest = if dest.is_null() {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, dest_len_value)
    };
    let source = if source.is_null() {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, source_len_value)
    };
    let result = uncompress2_z(dest, source);
    *dest_len = result.dest_len;
    *source_len = result.source_len;
    result.status
}

#[export_name = "uncompress2"]
pub unsafe extern "C" fn uncompress2_ffi(
    dest: *mut Bytef,
    dest_len: *mut uLongf,
    source: *const Bytef,
    source_len: *mut uLong,
) -> ::core::ffi::c_int {
    if dest_len.is_null() || source_len.is_null() {
        return Z_STREAM_ERROR;
    }
    let dest_len_value = *dest_len as usize;
    let source_len_value = *source_len as usize;
    if dest.is_null() && dest_len_value != 0 {
        return Z_STREAM_ERROR;
    }
    if source.is_null() && source_len_value != 0 {
        return Z_STREAM_ERROR;
    }
    let dest = if dest.is_null() {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, dest_len_value)
    };
    let source = if source.is_null() {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, source_len_value)
    };
    let result = uncompress2(dest, source);
    *dest_len = result.dest_len as uLongf;
    *source_len = result.source_len as uLong;
    result.status
}

#[export_name = "uncompress_z"]
pub unsafe extern "C" fn uncompress_z_ffi(
    dest: *mut Bytef,
    dest_len: *mut z_size_t,
    source: *const Bytef,
    source_len: z_size_t,
) -> ::core::ffi::c_int {
    if dest_len.is_null() {
        return Z_STREAM_ERROR;
    }
    let dest_len_value = *dest_len;
    if dest.is_null() && dest_len_value != 0 {
        return Z_STREAM_ERROR;
    }
    if source.is_null() && source_len != 0 {
        return Z_STREAM_ERROR;
    }
    let dest = if dest.is_null() {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, dest_len_value)
    };
    let source = if source.is_null() {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, source_len)
    };
    let result = uncompress_z(dest, source);
    *dest_len = result.dest_len;
    result.status
}

#[export_name = "uncompress"]
pub unsafe extern "C" fn uncompress_ffi(
    dest: *mut Bytef,
    dest_len: *mut uLongf,
    source: *const Bytef,
    source_len: uLong,
) -> ::core::ffi::c_int {
    if dest_len.is_null() {
        return Z_STREAM_ERROR;
    }
    let dest_len_value = *dest_len as usize;
    if dest.is_null() && dest_len_value != 0 {
        return Z_STREAM_ERROR;
    }
    if source.is_null() && source_len != 0 {
        return Z_STREAM_ERROR;
    }
    let dest = if dest.is_null() {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, dest_len_value)
    };
    let source = if source.is_null() {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, source_len as usize)
    };
    let result = uncompress(dest, source);
    *dest_len = result.dest_len as uLongf;
    result.status
}
