pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate;
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

/// Take one zlib-sized chunk from a possibly larger byte count.  The caller
/// keeps its raw stream cursors at the boundary; this helper owns only the
/// wrapping scalar accounting used by the one-shot decoder.
fn uncompress_chunk(
    remaining: crate::stdlib::z_size_t,
    max: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::stdlib::z_size_t) {
    let chunk = if remaining > max as crate::stdlib::z_size_t {
        max
    } else {
        remaining as crate::stdlib::uInt
    };
    (
        chunk,
        remaining.wrapping_sub(chunk as crate::stdlib::z_size_t),
    )
}

/// Translate the final inflate status to the documented one-shot result
/// after the boundary has committed its input and output counters.
fn uncompress_result(
    status: ::core::ffi::c_int,
    remaining_input: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if status == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else if status == crate::zlib_h::Z_NEED_DICT {
        crate::zlib_h::Z_DATA_ERROR
    } else if status == crate::zlib_h::Z_BUF_ERROR && remaining_input == 0 {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        status
    }
}

/// Finalize one-shot input/output accounting after the ABI boundary has
/// committed the stream cursors.  Keeping the wrapping subtraction here
/// makes the exported wrappers responsible only for reading and publishing
/// their width-specific length arguments.
struct UncompressProgress {
    source_used: crate::stdlib::z_size_t,
    dest_used: crate::stdlib::z_size_t,
}

fn uncompress_progress(
    source_total: crate::stdlib::z_size_t,
    dest_total: crate::stdlib::z_size_t,
    source_remaining: crate::stdlib::z_size_t,
    dest_remaining: crate::stdlib::z_size_t,
) -> UncompressProgress {
    UncompressProgress {
        source_used: source_total.wrapping_sub(source_remaining),
        dest_used: dest_total.wrapping_sub(dest_remaining),
    }
}

// This macro intentionally expands only in exported functions.  It owns the
// one-shot stream's raw cursors and legacy inflate calls until C3 has a safe
// stream-call adapter.  Keeping it here avoids private unsafe forwarding
// adapters for each ABI-width spelling of uncompress().
macro_rules! uncompress2_z_at_boundary {
    ($dest:expr, $dest_len:expr, $source:expr, $source_len:expr) => {{
        let mut dest = $dest;
        let dest_len = $dest_len;
        let source = $source;
        let source_len = $source_len;
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
        let mut len: crate::stdlib::z_size_t = 0;
        let mut left: crate::stdlib::z_size_t = 0;
        'uncompress2_z_result: {
            if source_len.is_null()
                || *source_len > 0 as crate::stdlib::z_size_t && source.is_null()
                || dest_len.is_null()
                || *dest_len > 0 as crate::stdlib::z_size_t && dest.is_null()
            {
                break 'uncompress2_z_result crate::zlib_h::Z_STREAM_ERROR;
            }
            let source_total = *source_len;
            let dest_total = *dest_len;
            len = source_total;
            left = dest_total;
            if left == 0 as crate::stdlib::z_size_t && dest.is_null() {
                dest = &raw mut stream.reserved as *mut crate::stdlib::Bytef;
            }
            stream.next_in = source as *mut crate::stdlib::Bytef;
            stream.avail_in = 0 as crate::stdlib::uInt;
            stream.zalloc = None;
            stream.zfree = None;
            stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
            err = crate::src::inflate::inflate_init2_at_boundary!(
                &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
                crate::zutil_h::DEF_WBITS,
                crate::zlib_h::ZLIB_VERSION.as_ptr(),
                ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
            );
            if err != crate::zlib_h::Z_OK {
                break 'uncompress2_z_result err;
            }
            stream.next_out = dest;
            stream.avail_out = 0 as crate::stdlib::uInt;
            loop {
                if stream.avail_out == 0 as crate::stdlib::uInt {
                    let (chunk, remaining) = uncompress_chunk(left, max);
                    stream.avail_out = chunk;
                    left = remaining;
                }
                if stream.avail_in == 0 as crate::stdlib::uInt {
                    let (chunk, remaining) = uncompress_chunk(len, max);
                    stream.avail_in = chunk;
                    len = remaining;
                }
                err = crate::src::inflate::inflate(&mut stream, crate::zlib_h::Z_NO_FLUSH);
                if err != crate::zlib_h::Z_OK {
                    break;
                }
            }
            len = len.wrapping_add(stream.avail_in as crate::stdlib::z_size_t);
            left = left.wrapping_add(stream.avail_out as crate::stdlib::z_size_t);
            let progress = uncompress_progress(source_total, dest_total, len, left);
            *source_len = progress.source_used;
            *dest_len = progress.dest_used;
            crate::src::inflate::inflate_end_at_boundary!(
                &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
            );
            uncompress_result(err, len)
        }
    }};
}
#[export_name = "uncompress2_z"]

pub unsafe extern "C" fn uncompress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    uncompress2_z_at_boundary!(dest, destLen, source, sourceLen)
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
    let mut got = *destLen as crate::stdlib::z_size_t;
    let mut used = *sourceLen as crate::stdlib::z_size_t;
    let ret = uncompress2_z_at_boundary!(dest, &raw mut got, source, &raw mut used);
    *sourceLen = used as crate::stdlib::uLong;
    *destLen = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    ret
}
#[export_name = "uncompress_z"]

pub unsafe extern "C" fn uncompress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    let mut used = sourceLen;
    uncompress2_z_at_boundary!(dest, destLen, source, &raw mut used)
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
    let mut got = *destLen as crate::stdlib::z_size_t;
    let mut used = sourceLen as crate::stdlib::z_size_t;
    let ret = uncompress2_z_at_boundary!(dest, &raw mut got, source, &raw mut used);
    *destLen = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    ret
}
