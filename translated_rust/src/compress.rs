pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::deflate;
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

/// Compute the number of bytes committed to the one-shot output stream from
/// its scalar capacity accounting.  `unassigned` has not yet been lent to
/// deflate, while `avail_out` is the unused part of the final lent chunk.
/// This replaces same-allocation raw cursor-distance arithmetic at the ABI
/// boundary.
fn compress_output_len(
    capacity: crate::stdlib::z_size_t,
    unassigned: crate::stdlib::z_size_t,
    avail_out: crate::stdlib::uInt,
) -> crate::stdlib::z_size_t {
    capacity
        .wrapping_sub(unassigned)
        .wrapping_sub(avail_out as crate::stdlib::z_size_t)
}

// This macro expands only in exported functions. It retains the one-shot
// stream's raw cursors and legacy deflate calls at the ABI boundary until C4
// has a safe stream-call adapter, avoiding private unsafe forwarding
// implementations for each ABI-width spelling of compress().
macro_rules! compress2_z_at_boundary {
    ($dest:expr, $dest_len:expr, $source:expr, $source_len:expr, $level:expr $(,)?) => {{
        let mut dest = $dest;
        let destLen = $dest_len;
        let source = $source;
        let mut sourceLen = $source_len;
        let level = $level;
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
        let mut left: crate::stdlib::z_size_t = 0;
        let mut capacity: crate::stdlib::z_size_t = 0;
        if sourceLen > 0 as crate::stdlib::z_size_t && source.is_null()
            || destLen.is_null()
            || *destLen > 0 as crate::stdlib::z_size_t && dest.is_null()
        {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        left = *destLen;
        capacity = left;
        *destLen = 0 as crate::stdlib::z_size_t;
        stream.zalloc = None;
        stream.zfree = None;
        stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        err = crate::src::deflate::deflate_init2_at_boundary!(
            &raw mut stream,
            level,
            crate::zlib_h::Z_DEFLATED,
            crate::stdlib::MAX_WBITS,
            crate::zutil_h::DEF_MEM_LEVEL,
            crate::zlib_h::Z_DEFAULT_STRATEGY,
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
            let input_len = stream.avail_in as usize;
            let mut input = if input_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(stream.next_in, input_len)
            };
            let output_len = stream.avail_out as usize;
            let output = if output_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(stream.next_out, output_len)
            };
            let mut output = crate::src::deflate::DeflateOutput::new(output);
            let state = stream.state as *mut crate::src::deflate::deflate_state;
            if state.is_null() {
                err = crate::zlib_h::Z_STREAM_ERROR;
                break;
            }
            let pending_len = (*state).pending_buf_size as usize;
            if pending_len != 0 && (*state).pending_buf.is_none() {
                err = crate::zlib_h::Z_STREAM_ERROR;
                break;
            }
            let pending_buf = if pending_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(
                    (*state)
                        .pending_buf
                        .expect("validated deflate pending buffer")
                        .as_ptr(),
                    pending_len,
                )
            };
            let Some(mut window_hash) =
                crate::src::deflate::deflate_window_hash_buffers_at_boundary!(&mut *state)
            else {
                err = crate::zlib_h::Z_STREAM_ERROR;
                break;
            };
            err = crate::src::deflate::deflate(
                &mut stream,
                &mut *state,
                &mut input,
                pending_buf,
                &mut window_hash,
                &mut output,
                crate::src::deflate::DeflateGzipPayloads::empty(),
                if sourceLen != 0 {
                    crate::zlib_h::Z_NO_FLUSH
                } else {
                    crate::zlib_h::Z_FINISH
                },
            );
            if err != crate::zlib_h::Z_OK {
                break;
            }
        }
        *destLen = compress_output_len(capacity, left, stream.avail_out);
        crate::src::deflate::deflateEnd(&mut stream);
        if err == crate::zlib_h::Z_STREAM_END {
            crate::zlib_h::Z_OK
        } else {
            err
        }
    }};
}
#[export_name = "compress2_z"]

pub unsafe extern "C" fn compress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    compress2_z_at_boundary!(dest, destLen, source, sourceLen, level)
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
    ret = compress2_z_at_boundary!(
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
    compress2_z_at_boundary!(
        dest,
        destLen,
        source,
        sourceLen,
        crate::zlib_h::Z_DEFAULT_COMPRESSION,
    )
}
#[export_name = "compress"]
pub unsafe extern "C" fn compress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let mut got = *destLen as crate::stdlib::z_size_t;
    let ret = compress2_z_at_boundary!(
        dest,
        &raw mut got,
        source,
        sourceLen as crate::stdlib::z_size_t,
        crate::zlib_h::Z_DEFAULT_COMPRESSION,
    );
    *destLen = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    ret
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
    sourceLen: crate::stdlib::z_size_t,
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
    sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    compressBound(sourceLen)
}
pub use crate::src::deflate::deflateEnd;
