pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_intmax;
pub use crate::src::gzlib::gz_error;

pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::deflate;
pub use crate::src::deflate::deflateEnd;
pub use crate::src::deflate::deflateInit2_;
pub use crate::src::deflate::deflateParams;
pub use crate::src::deflate::deflateReset;
pub use crate::src::deflate::internal_state;

pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpc;
pub use crate::stdlib::voidpf;

pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gzFile;
pub use crate::zlib_h::gzFile_s;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BLOCK;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_ERRNO;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;

unsafe extern "C" fn gz_init(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    (*state).in_0 = crate::stdlib::malloc(
        ((*state).want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
    ) as *mut ::core::ffi::c_uchar;
    if (*state).in_0.is_null() {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_MEM_ERROR,
            b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*state).direct == 0 {
        (*state).out = crate::stdlib::malloc((*state).want as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar;
        if (*state).out.is_null() {
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*strm).zalloc = None;
        (*strm).zfree = None;
        (*strm).opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ret = crate::src::deflate::deflateInit2_(
            strm as *mut crate::zlib_h::z_stream_s,
            (*state).level,
            8 as ::core::ffi::c_int,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (*state).strategy,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        );
        if ret != crate::zlib_h::Z_OK {
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*strm).next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    (*state).size = (*state).want;
    if (*state).direct == 0 {
        (*strm).avail_out = (*state).size as crate::stdlib::uInt;
        (*strm).next_out = (*state).out as *mut crate::stdlib::Bytef;
        (*state).x.next = (*strm).next_out as *mut ::core::ffi::c_uchar;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_comp(
    mut state: crate::gzguts_h::gz_statep,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).direct != 0 {
        while (*strm).avail_in != 0 {
            *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
            (*state).again = 0 as ::core::ffi::c_int;
            put = if (*strm).avail_in > max {
                max
            } else {
                (*strm).avail_in as ::core::ffi::c_uint
            };
            writ = crate::stdlib::write(
                (*state).fd,
                (*strm).next_in as *const ::core::ffi::c_void,
                put as crate::__stddef_size_t_h::size_t,
            ) as ::core::ffi::c_int;
            if writ < 0 as ::core::ffi::c_int {
                if *crate::stdlib::__errno_location() == crate::stdlib::EAGAIN
                    || *crate::stdlib::__errno_location() == crate::stdlib::EWOULDBLOCK
                {
                    (*state).again = 1 as ::core::ffi::c_int;
                }
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_ERRNO,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                return -1 as ::core::ffi::c_int;
            }
            (*strm).avail_in = (*strm).avail_in.wrapping_sub(writ as ::core::ffi::c_uint);
            (*strm).next_in = (*strm).next_in.offset(writ as isize);
        }
        return 0 as ::core::ffi::c_int;
    }
    if (*state).reset != 0 {
        if (*strm).avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
            return 0 as ::core::ffi::c_int;
        }
        crate::src::deflate::deflateReset(strm as *mut crate::zlib_h::z_stream_s);
        (*state).reset = 0 as ::core::ffi::c_int;
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if (*strm).avail_out == 0 as crate::stdlib::uInt
            || flush != crate::zlib_h::Z_NO_FLUSH
                && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
        {
            while (*strm).next_out > (*state).x.next {
                *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                (*state).again = 0 as ::core::ffi::c_int;
                put = if (*strm).next_out.offset_from((*state).x.next)
                    > max as ::core::ffi::c_int as isize
                {
                    max
                } else {
                    (*strm).next_out.offset_from((*state).x.next) as ::core::ffi::c_uint
                };
                writ = crate::stdlib::write(
                    (*state).fd,
                    (*state).x.next as *const ::core::ffi::c_void,
                    put as crate::__stddef_size_t_h::size_t,
                ) as ::core::ffi::c_int;
                if writ < 0 as ::core::ffi::c_int {
                    if *crate::stdlib::__errno_location() == crate::stdlib::EAGAIN
                        || *crate::stdlib::__errno_location() == crate::stdlib::EWOULDBLOCK
                    {
                        (*state).again = 1 as ::core::ffi::c_int;
                    }
                    crate::src::gzlib::gz_error(
                        state as *mut crate::gzguts_h::gz_state,
                        crate::zlib_h::Z_ERRNO,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    return -1 as ::core::ffi::c_int;
                }
                (*state).x.next = (*state).x.next.offset(writ as isize);
            }
            if (*strm).avail_out == 0 as crate::stdlib::uInt {
                (*strm).avail_out = (*state).size as crate::stdlib::uInt;
                (*strm).next_out = (*state).out as *mut crate::stdlib::Bytef;
                (*state).x.next = (*state).out;
            }
        }
        have = (*strm).avail_out as ::core::ffi::c_uint;
        ret = crate::src::deflate::deflate(strm as *mut crate::zlib_h::z_stream_s, flush);
        if ret == crate::zlib_h::Z_STREAM_ERROR {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        have = have.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
        if have == 0 {
            break;
        }
    }
    if flush == crate::zlib_h::Z_FINISH {
        (*state).reset = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

/// Choose one zero-fill chunk without converting the gzip handle or its
/// buffers.  The narrowing cast deliberately matches zlib's `unsigned`
/// chunk size when a malformed negative skip reaches this private adapter.
fn gz_zero_chunk_plan(
    size: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
) -> ::core::ffi::c_uint {
    if (::core::mem::size_of::<::core::ffi::c_int>()
        == ::core::mem::size_of::<crate::stdlib::off64_t>()
        && size > crate::src::gzlib::gz_intmax())
        || size as crate::stdlib::off64_t > skip
    {
        skip as ::core::ffi::c_uint
    } else {
        size
    }
}

/// Record the number of zero bytes consumed by a completed compression step.
/// The raw adapter owns the input cursor and compressor call; this only keeps
/// the C-style logical-position and pending-seek arithmetic together.
fn gz_zero_commit_state(state: &mut crate::gzguts_h::gz_state, consumed: ::core::ffi::c_uint) {
    state.x.pos = state.x.pos.wrapping_add(consumed as crate::stdlib::off64_t);
    state.skip = state.skip.wrapping_sub(consumed as crate::stdlib::off64_t);
}

/// Determine how much input fits after the existing buffered compressor
/// input.  The wrapping subtraction deliberately preserves zlib's behavior
/// for a malformed internal cursor while keeping the size conversion local.
fn gz_write_buffered_copy_plan(
    size: ::core::ffi::c_uint,
    buffered_end: ::core::ffi::c_uint,
    remaining: crate::stdlib::z_size_t,
) -> ::core::ffi::c_uint {
    let copy = size.wrapping_sub(buffered_end);
    if copy as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        copy
    }
}

/// Commit bytes copied into the pending compressor input after the raw
/// boundary has completed the bounded buffer copy.
fn gz_write_buffered_copy_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    copied: ::core::ffi::c_uint,
) {
    state.strm.avail_in = state.strm.avail_in.wrapping_add(copied);
    state.x.pos = state.x.pos.wrapping_add(copied as crate::stdlib::off64_t);
}

/// Limit one direct-write compression input chunk to the `uInt` range used
/// by zlib, without touching the caller-owned input cursor.
fn gz_write_direct_chunk_plan(remaining: crate::stdlib::z_size_t) -> ::core::ffi::c_uint {
    let max = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    if max as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        max
    }
}

/// Commit progress reported by a direct-write compression step.  The caller
/// keeps the raw stream cursor at the boundary; this preserves zlib's
/// wrapping logical-position arithmetic and returns the remaining input.
fn gz_write_direct_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    remaining: crate::stdlib::z_size_t,
    consumed: ::core::ffi::c_uint,
) -> crate::stdlib::z_size_t {
    state.x.pos = state.x.pos.wrapping_add(consumed as crate::stdlib::off64_t);
    remaining.wrapping_sub(consumed as crate::stdlib::z_size_t)
}

/// Convert a failed compression step into zlib's public write progress.  A
/// retryable descriptor error reports the completed prefix; every other
/// error reports no completed write.
fn gz_write_failure_result(
    again: ::core::ffi::c_int,
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if again != 0 {
        requested.wrapping_sub(remaining)
    } else {
        0
    }
}

/// Validate the common public gzip-writer admission state without borrowing
/// the opaque handle.  A retryable descriptor error remains writable, as it
/// does for every write-family entry point.
fn gzwrite_state_is_valid(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
) -> bool {
    mode == crate::gzguts_h::GZ_WRITE && (err == crate::zlib_h::Z_OK || again != 0)
}

/// Compute the byte length requested by `gzfwrite`.  `None` preserves the
/// API's overflow failure, while `Some(0)` remains an ordinary empty request.
fn gzfwrite_request_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    let len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        None
    } else {
        Some(len)
    }
}

/// Convert completed byte count back to completed items after a non-empty
/// `gzfwrite` request.  The caller keeps the raw write and handle access at
/// the boundary.
fn gzfwrite_completed_items(
    written: crate::stdlib::z_size_t,
    size: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    written.wrapping_div(size)
}

/// Accept precisely the flush values supported by the public gzip writer.
/// The opaque handle and compressor remain at the FFI boundary.
fn gzflush_is_valid(flush: ::core::ffi::c_int) -> bool {
    (crate::zlib_h::Z_NO_FLUSH..=crate::zlib_h::Z_FINISH).contains(&flush)
}

/// Validate the scalar gzip-writer state needed by `gzsetparams`, without
/// borrowing the opaque handle or touching the compressor.  A pending retry
/// is permitted to match zlib's existing write-side admission rule.
fn gzsetparams_state_is_valid(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
) -> bool {
    mode == crate::gzguts_h::GZ_WRITE && (err == crate::zlib_h::Z_OK || again != 0) && direct == 0
}

/// Decide whether a parameter update has work to do.  Keeping this separate
/// from the handle adapter avoids observing or changing state before the
/// existing no-op return.
fn gzsetparams_needs_update(
    current_level: ::core::ffi::c_int,
    current_strategy: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> bool {
    level != current_level || strategy != current_strategy
}

/// Apply the descriptor-close outcome to the write-side close result.  The
/// compressor, descriptor, and owned buffers remain at the raw boundary.
fn gzclose_write_result(
    close_result: ::core::ffi::c_int,
    result_before_close: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if close_result == -1 {
        crate::zlib_h::Z_ERRNO
    } else {
        result_before_close
    }
}

unsafe extern "C" fn gz_zero(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*strm).avail_in != 0
        && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    first = 1 as ::core::ffi::c_int;
    loop {
        n = gz_zero_chunk_plan((*state).size, (*state).skip);
        if first != 0 {
            crate::stdlib::memset(
                (*state).in_0 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                n as crate::__stddef_size_t_h::size_t,
            );
            first = 0 as ::core::ffi::c_int;
        }
        (*strm).avail_in = n as crate::stdlib::uInt;
        (*strm).next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        n = n.wrapping_sub((*strm).avail_in as ::core::ffi::c_uint);
        gz_zero_commit_state(&mut *state, n);
        if ret == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if (*state).skip == 0 {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_write(
    mut state: crate::gzguts_h::gz_statep,
    mut buf: crate::stdlib::voidpc,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut put: crate::stdlib::z_size_t = len;
    let mut ret: ::core::ffi::c_int = 0;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if (*state).size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if len < (*state).size as crate::stdlib::z_size_t {
        loop {
            let mut have: ::core::ffi::c_uint = 0;
            let mut copy: ::core::ffi::c_uint = 0;
            if (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                (*state).strm.next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
            }
            have = (*state)
                .strm
                .next_in
                .offset((*state).strm.avail_in as isize)
                .offset_from((*state).in_0) as ::core::ffi::c_uint;
            copy = gz_write_buffered_copy_plan((*state).size, have, len);
            crate::stdlib::memcpy(
                (*state).in_0.offset(have as isize) as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                copy as crate::__stddef_size_t_h::size_t,
            );
            gz_write_buffered_copy_commit_state(&mut *state, copy);
            buf =
                (buf as *const ::core::ffi::c_char).offset(copy as isize) as crate::stdlib::voidpc;
            len = len.wrapping_sub(copy as crate::stdlib::z_size_t);
            if len == 0 as crate::stdlib::z_size_t {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return gz_write_failure_result((*state).again, put, len);
            }
        }
    } else {
        if (*state).strm.avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
        {
            return 0 as crate::stdlib::z_size_t;
        }
        (*state).strm.next_in = buf as *mut crate::stdlib::Bytef;
        loop {
            let mut n = gz_write_direct_chunk_plan(len);
            (*state).strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            n = n.wrapping_sub((*state).strm.avail_in as ::core::ffi::c_uint);
            len = gz_write_direct_commit_state(&mut *state, len, n);
            if ret == -1 as ::core::ffi::c_int {
                return gz_write_failure_result((*state).again, put, len);
            }
            if len == 0 {
                break;
            }
        }
    }
    return put;
}
pub unsafe extern "C" fn gzwrite(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzwrite_state_is_valid((*state).mode, (*state).err, (*state).again) {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !crate::src::gzlib::gz_request_len_fits_int(len) {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_DATA_ERROR,
            b"requested length does not fit in int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return gz_write(state, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_int;
}
#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzwrite(file, buf, len)
}
pub unsafe extern "C" fn gzfwrite(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzwrite_state_is_valid((*state).mode, (*state).err, (*state).again) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    match gzfwrite_request_len(size, nitems) {
        None => {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
            );
            0
        }
        Some(0) => 0,
        Some(len) => gzfwrite_completed_items(gz_write(state, buf, len), size),
    }
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    gzfwrite(buf, size, nitems, file)
}
pub unsafe extern "C" fn gzputc(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut have: ::core::ffi::c_uint = 0;
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut strm: crate::zlib_h::z_streamp = ::core::ptr::null_mut::<crate::zlib_h::z_stream>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    strm = &raw mut (*state).strm as crate::zlib_h::z_streamp;
    if !gzwrite_state_is_valid((*state).mode, (*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).size != 0 {
        if (*strm).avail_in == 0 as crate::stdlib::uInt {
            (*strm).next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
        }
        have = (*strm)
            .next_in
            .offset((*strm).avail_in as isize)
            .offset_from((*state).in_0) as ::core::ffi::c_uint;
        if have < (*state).size {
            *(*state).in_0.offset(have as isize) = c as ::core::ffi::c_uchar;
            gz_write_buffered_copy_commit_state(&mut *state, 1);
            return c & 0xff as ::core::ffi::c_int;
        }
    }
    buf[0 as ::core::ffi::c_int as usize] = c as ::core::ffi::c_uchar;
    if gz_write(
        state,
        &raw mut buf as *mut ::core::ffi::c_uchar as crate::stdlib::voidpc,
        1 as crate::stdlib::z_size_t,
    ) != 1 as crate::stdlib::z_size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    return c & 0xff as ::core::ffi::c_int;
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzputc(file, c)
}
pub unsafe extern "C" fn gzputs(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: crate::stdlib::z_size_t = 0;
    let mut put: crate::stdlib::z_size_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzwrite_state_is_valid((*state).mode, (*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    len = crate::stdlib::strlen(s) as crate::stdlib::z_size_t;
    if !crate::src::gzlib::gz_len_fits_int(len) {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"string length does not fit in int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    put = gz_write(state, s as crate::stdlib::voidpc, len);
    return if len != 0 && put == 0 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        put as ::core::ffi::c_int
    };
}
#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    gzputs(file, s)
}
pub unsafe extern "C" fn gzflush(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzwrite_state_is_valid(state.mode, state.err, state.again) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error(
        state as *mut _,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !gzflush_is_valid(flush) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 && gz_zero(state as *mut _) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    gz_comp(state as *mut _, flush);
    state.err
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzflush(file, flush)
}
pub unsafe extern "C" fn gzsetparams(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut strm: crate::zlib_h::z_streamp = ::core::ptr::null_mut::<crate::zlib_h::z_stream>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    strm = &raw mut (*state).strm as crate::zlib_h::z_streamp;
    if !gzsetparams_state_is_valid((*state).mode, (*state).err, (*state).again, (*state).direct) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !gzsetparams_needs_update((*state).level, (*state).strategy, level, strategy) {
        return crate::zlib_h::Z_OK;
    }
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return (*state).err;
    }
    if (*state).size != 0 {
        if (*strm).avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
        {
            return (*state).err;
        }
        crate::src::deflate::deflateParams(strm as *mut crate::zlib_h::z_stream_s, level, strategy);
    }
    (*state).level = level;
    (*state).strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzsetparams(file, level, strategy)
}
pub unsafe extern "C" fn gzclose_w(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_WRITE {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        ret = (*state).err;
    }
    if gz_comp(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int {
        ret = (*state).err;
    }
    if (*state).size != 0 {
        if (*state).direct == 0 {
            crate::src::deflate::deflateEnd(
                &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            );
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
        }
        crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
    let close_result = crate::stdlib::close((*state).fd);
    crate::stdlib::free(state as *mut ::core::ffi::c_void);
    gzclose_write_result(close_result, ret)
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_w(file)
}
