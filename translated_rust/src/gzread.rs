pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZIP;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::LOOK;
pub use crate::src::gzlib::gz_error;
pub use crate::src::gzlib::gz_intmax;

pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate;
pub use crate::src::inflate::inflateEnd;
pub use crate::src::inflate::inflateInit2_;
pub use crate::src::inflate::inflateReset;

pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidp;
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
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_ERRNO;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NEED_DICT;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;

/// Decide whether a public read operation may proceed after its boundary
/// adapter has sampled the opaque gzip state.  A retryable descriptor error
/// remains admissible, matching zlib's nonblocking I/O behavior.
fn gzread_state_is_valid(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
) -> bool {
    mode == crate::gzguts_h::GZ_READ
        && (err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR || again != 0)
}

unsafe extern "C" fn gz_load(
    mut state: crate::gzguts_h::gz_statep,
    mut buf: *mut ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
    mut have: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut get: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    (*state).again = 0 as ::core::ffi::c_int;
    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
    *have = 0 as ::core::ffi::c_uint;
    loop {
        get = len.wrapping_sub(*have);
        if get > max {
            get = max;
        }
        ret = crate::stdlib::read(
            (*state).fd,
            buf.offset(*have as isize) as *mut ::core::ffi::c_void,
            get as crate::__stddef_size_t_h::size_t,
        ) as ::core::ffi::c_int;
        if ret <= 0 as ::core::ffi::c_int {
            break;
        }
        *have = (*have).wrapping_add(ret as ::core::ffi::c_uint);
        if *have >= len {
            break;
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        if *crate::stdlib::__errno_location() == crate::stdlib::EAGAIN
            || *crate::stdlib::__errno_location() == crate::stdlib::EWOULDBLOCK
        {
            (*state).again = 1 as ::core::ffi::c_int;
            if *have != 0 as ::core::ffi::c_uint {
                return 0 as ::core::ffi::c_int;
            }
        }
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_ERRNO,
            crate::stdlib::strerror(*crate::stdlib::__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    if ret == 0 as ::core::ffi::c_int {
        (*state).eof = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_avail(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut got: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).err != crate::zlib_h::Z_OK && (*state).err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).eof == 0 as ::core::ffi::c_int {
        if (*strm).avail_in != 0 {
            let mut p: *mut ::core::ffi::c_uchar = (*state).in_0;
            let mut q: *const ::core::ffi::c_uchar = (*strm).next_in;
            if q != p as *const ::core::ffi::c_uchar {
                let mut n: ::core::ffi::c_uint = (*strm).avail_in as ::core::ffi::c_uint;
                loop {
                    let c2rust_fresh0 = q;
                    q = q.offset(1);
                    let c2rust_fresh1 = p;
                    p = p.offset(1);
                    *c2rust_fresh1 = *c2rust_fresh0;
                    n = n.wrapping_sub(1);
                    if n == 0 {
                        break;
                    }
                }
            }
        }
        if gz_load(
            state,
            (*state).in_0.offset((*strm).avail_in as isize),
            (*state)
                .size
                .wrapping_sub((*strm).avail_in as ::core::ffi::c_uint),
            &raw mut got,
        ) == -1 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        (*strm).avail_in = (*strm).avail_in.wrapping_add(got);
        (*strm).next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_look(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint {
        (*state).in_0 = crate::stdlib::malloc((*state).want as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar;
        (*state).out = crate::stdlib::malloc(
            ((*state).want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
        ) as *mut ::core::ffi::c_uchar;
        if (*state).in_0.is_null() || (*state).out.is_null() {
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*state).size = (*state).want;
        (*state).strm.zalloc = None;
        (*state).strm.zfree = None;
        (*state).strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*state).strm.avail_in = 0 as crate::stdlib::uInt;
        (*state).strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit2_(
            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            (*state).size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if (*state).direct == -1 as ::core::ffi::c_int || (*state).junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
        (*state).how = crate::gzguts_h::GZIP;
        (*state).junk = ((*state).junk != -1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        (*state).direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*strm).avail_in == 0 as crate::stdlib::uInt
        || (*state).again != 0 && (*strm).avail_in < 4 as crate::stdlib::uInt
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*strm).avail_in > 3 as crate::stdlib::uInt
        && *(*strm).next_in.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 31 as ::core::ffi::c_int
        && *(*strm).next_in.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 139 as ::core::ffi::c_int
        && *(*strm).next_in.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 8 as ::core::ffi::c_int
        && (*(*strm).next_in.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
    {
        crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
        (*state).how = crate::gzguts_h::GZIP;
        (*state).junk = 1 as ::core::ffi::c_int;
        (*state).direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    (*state).x.next = (*state).out;
    crate::stdlib::memcpy(
        (*state).x.next as *mut ::core::ffi::c_void,
        (*strm).next_in as *const ::core::ffi::c_void,
        (*strm).avail_in as crate::__stddef_size_t_h::size_t,
    );
    (*state).x.have = (*strm).avail_in as ::core::ffi::c_uint;
    (*strm).avail_in = 0 as crate::stdlib::uInt;
    (*state).how = crate::gzguts_h::COPY;
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_decomp(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut had: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    had = (*strm).avail_out as ::core::ffi::c_uint;
    loop {
        if (*strm).avail_in == 0 as crate::stdlib::uInt
            && gz_avail(state) == -1 as ::core::ffi::c_int
        {
            ret = (*state).err;
            break;
        } else if (*strm).avail_in == 0 as crate::stdlib::uInt {
            if (*state).again == 0 {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_BUF_ERROR,
                    b"unexpected end of file\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            break;
        } else {
            ret = crate::src::inflate::inflate(
                strm as *mut crate::zlib_h::z_stream_s,
                crate::zlib_h::Z_NO_FLUSH,
            );
            if (*strm).avail_out < had {
                (*state).junk = 0 as ::core::ffi::c_int;
            }
            if ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                break;
            } else if ret == crate::zlib_h::Z_MEM_ERROR {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break;
            } else if ret == crate::zlib_h::Z_DATA_ERROR {
                if (*state).junk == 1 as ::core::ffi::c_int {
                    (*strm).avail_in = 0 as crate::stdlib::uInt;
                    (*state).eof = 1 as ::core::ffi::c_int;
                    (*state).how = crate::gzguts_h::LOOK;
                    ret = crate::zlib_h::Z_OK;
                    break;
                } else {
                    crate::src::gzlib::gz_error(
                        state as *mut crate::gzguts_h::gz_state,
                        crate::zlib_h::Z_DATA_ERROR,
                        if (*strm).msg.is_null() {
                            b"compressed data error\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            (*strm).msg as *const ::core::ffi::c_char
                        },
                    );
                    break;
                }
            } else if !((*strm).avail_out != 0 && ret != crate::zlib_h::Z_STREAM_END) {
                break;
            }
        }
    }
    (*state).x.have =
        (had as crate::stdlib::uInt).wrapping_sub((*strm).avail_out) as ::core::ffi::c_uint;
    (*state).x.next =
        (*strm).next_out.offset(-((*state).x.have as isize)) as *mut ::core::ffi::c_uchar;
    if ret == crate::zlib_h::Z_STREAM_END {
        (*state).junk = 0 as ::core::ffi::c_int;
        (*state).how = crate::gzguts_h::LOOK;
        return 0 as ::core::ffi::c_int;
    }
    return if ret != crate::zlib_h::Z_OK {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}

unsafe extern "C" fn gz_fetch(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    loop {
        match (*state).how {
            crate::gzguts_h::LOOK => {
                if gz_look(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                if (*state).how == crate::gzguts_h::LOOK {
                    return 0 as ::core::ffi::c_int;
                }
            }
            crate::gzguts_h::COPY => {
                if gz_load(
                    state,
                    (*state).out,
                    (*state).size << 1 as ::core::ffi::c_int,
                    &raw mut (*state).x.have,
                ) == -1 as ::core::ffi::c_int
                {
                    return -1 as ::core::ffi::c_int;
                }
                (*state).x.next = (*state).out;
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                (*strm).avail_out =
                    ((*state).size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                (*strm).next_out = (*state).out as *mut crate::stdlib::Bytef;
                if gz_decomp(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            _ => {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"state corrupt\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if !((*state).x.have == 0 as ::core::ffi::c_uint
            && ((*state).eof == 0 || (*strm).avail_in != 0))
        {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

/// Decide how much already-buffered gzip output a pending forward seek can
/// consume.  Cursor reconstruction remains at the raw boundary, but this
/// keeps the size conversion and scalar arithmetic out of that adapter.
fn gz_skip_buffer_plan(
    have: crate::stdlib::uInt,
    skip: crate::stdlib::off64_t,
) -> Option<::core::ffi::c_uint> {
    if skip < 0 {
        return None;
    }
    Some(
        if (::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && have > crate::src::gzlib::gz_intmax())
            || have as crate::stdlib::off64_t > skip
        {
            skip as ::core::ffi::c_uint
        } else {
            have
        },
    )
}

/// Commit a preflighted buffered-seek consumption after the boundary has
/// advanced the raw output cursor.
fn gz_skip_buffer_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    consume: ::core::ffi::c_uint,
) -> bool {
    if consume > state.x.have || state.skip < consume as crate::stdlib::off64_t {
        return false;
    }
    state.x.have = state.x.have.wrapping_sub(consume);
    state.x.pos = state.x.pos.wrapping_add(consume as crate::stdlib::off64_t);
    state.skip = state.skip.wrapping_sub(consume as crate::stdlib::off64_t);
    true
}

/// Limit a direct copy from already-buffered gzip output to the amount that
/// is both requested and available.  The raw cursor is advanced separately at
/// the boundary only after this preflight succeeds.
fn gz_read_buffer_copy_plan(
    requested: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    requested.min(have)
}

/// Commit a preflighted direct buffered read after its raw cursor has moved.
/// The shared read loop records the logical position for every source path.
fn gz_read_buffer_copy_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    copied: ::core::ffi::c_uint,
) -> bool {
    if copied > state.x.have {
        return false;
    }
    state.x.have = state.x.have.wrapping_sub(copied);
    true
}

/// Commit the common read-loop byte accounting after the boundary has copied
/// or produced `copied` bytes and advanced its raw caller cursor.
fn gz_read_progress_state(
    remaining: crate::stdlib::z_size_t,
    got: crate::stdlib::z_size_t,
    pos: crate::stdlib::off64_t,
    copied: ::core::ffi::c_uint,
) -> (
    crate::stdlib::z_size_t,
    crate::stdlib::z_size_t,
    crate::stdlib::off64_t,
) {
    let copied = copied as crate::stdlib::z_size_t;
    (
        remaining.wrapping_sub(copied),
        got.wrapping_add(copied),
        pos.wrapping_add(copied as crate::stdlib::off64_t),
    )
}

/// Limit one `gzgets` copy to the caller's remaining space and, when one was
/// found in that range, include the newline byte.  The boundary still locates
/// that byte in its raw buffered output.
fn gzgets_buffer_copy_plan(
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    newline_offset: Option<::core::ffi::c_uint>,
) -> ::core::ffi::c_uint {
    let copy = have.min(left);
    match newline_offset {
        Some(offset) if offset < copy => offset + 1,
        _ => copy,
    }
}

/// Commit a preflighted `gzgets` buffered copy after the boundary has copied
/// bytes and advanced its raw cursors.
fn gzgets_buffer_copy_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    left: ::core::ffi::c_uint,
    copied: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    if copied > state.x.have || copied > left {
        return None;
    }
    state.x.have = state.x.have.wrapping_sub(copied);
    state.x.pos = state.x.pos.wrapping_add(copied as crate::stdlib::off64_t);
    Some(left.wrapping_sub(copied))
}

/// Commit consumption of one buffered byte after the boundary has read it and
/// advanced the raw cursor.  This keeps `gzgetc`'s visible prefix accounting
/// checked and independent of the raw output pointer.
fn gzgetc_buffer_commit_state(state: &mut crate::gzguts_h::gz_state) -> bool {
    if state.x.have == 0 {
        return false;
    }
    state.x.have = state.x.have.wrapping_sub(1);
    state.x.pos = state.x.pos.wrapping_add(1);
    true
}

/// Calculate a `gzfread` byte request with the same wrapping multiplication
/// and overflow rejection as the C API.  This deliberately reports a zero
/// request separately from an invalid overflowing request.
fn gzfread_request_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    let len = nitems.wrapping_mul(size);
    if size == 0 || len.wrapping_div(size) == nitems {
        Some(len)
    } else {
        None
    }
}

/// Convert a completed `gz_read` byte count into complete `gzfread` items.
/// A zero byte request is a successful zero-item operation and never divides
/// by a zero item size.
fn gzfread_completed_items(
    request_len: crate::stdlib::z_size_t,
    size: crate::stdlib::z_size_t,
    read_len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if request_len == 0 {
        0
    } else {
        read_len.wrapping_div(size)
    }
}

unsafe extern "C" fn gz_skip(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_uint = 0;
    loop {
        if (*state).x.have != 0 {
            let Some(consume) = gz_skip_buffer_plan((*state).x.have, (*state).skip) else {
                return -1 as ::core::ffi::c_int;
            };
            n = consume;
            (*state).x.next = (*state).x.next.offset(n as isize);
            if !gz_skip_buffer_commit_state(&mut *state, n) {
                return -1 as ::core::ffi::c_int;
            }
        } else {
            if (*state).eof != 0 && (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_fetch(state) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if (*state).skip == 0 {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_read(
    mut state: crate::gzguts_h::gz_statep,
    mut buf: crate::stdlib::voidp,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if (*state).skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    's_140: loop {
        n = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
        if n as crate::stdlib::z_size_t > len {
            n = len as ::core::ffi::c_uint;
        }
        's_28: {
            if (*state).x.have != 0 {
                n = gz_read_buffer_copy_plan(n, (*state).x.have);
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    (*state).x.next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                (*state).x.next = (*state).x.next.offset(n as isize);
                if !gz_read_buffer_copy_commit_state(&mut *state, n) {
                    return got;
                }
                if (*state).err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
            } else {
                if (*state).eof != 0 && (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                    break 's_140;
                }
                if (*state).how == crate::gzguts_h::LOOK
                    || n < (*state).size << 1 as ::core::ffi::c_int
                {
                    if gz_fetch(state) == -1 as ::core::ffi::c_int
                        && (*state).x.have == 0 as ::core::ffi::c_uint
                    {
                        err = -1 as ::core::ffi::c_int;
                    }
                    break 's_28;
                } else if (*state).how == crate::gzguts_h::COPY {
                    err = gz_load(state, buf as *mut ::core::ffi::c_uchar, n, &raw mut n);
                } else {
                    (*state).strm.avail_out = n as crate::stdlib::uInt;
                    (*state).strm.next_out =
                        buf as *mut ::core::ffi::c_uchar as *mut crate::stdlib::Bytef;
                    err = gz_decomp(state);
                    n = (*state).x.have;
                    (*state).x.have = 0 as ::core::ffi::c_uint;
                }
            }
            buf = (buf as *mut ::core::ffi::c_char).offset(n as isize) as crate::stdlib::voidp;
            (len, got, (*state).x.pos) = gz_read_progress_state(len, got, (*state).x.pos, n);
        }
        if !(len != 0 && err == 0) {
            break;
        }
    }
    if len != 0 && (*state).eof != 0 {
        (*state).past = 1 as ::core::ffi::c_int;
    }
    return got;
}
pub unsafe extern "C" fn gzread(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzread_state_is_valid((*state).mode, (*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !crate::src::gzlib::gz_request_len_fits_int(len) {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in an int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    len = gz_read(state, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        if (*state).err != crate::zlib_h::Z_OK && (*state).err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if (*state).again != 0 {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_ERRNO,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    return len as ::core::ffi::c_int;
}
#[export_name = "gzread"]

pub unsafe extern "C" fn gzread_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzread(file, buf, len)
}
pub unsafe extern "C" fn gzfread(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let mut len: crate::stdlib::z_size_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzread_state_is_valid((*state).mode, (*state).err, (*state).again) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(request_len) = gzfread_request_len(size, nitems) else {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as crate::stdlib::z_size_t;
    };
    len = request_len;
    return gzfread_completed_items(len, size, gz_read(state, buf, len));
}
#[export_name = "gzfread"]

pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    gzfread(buf, size, nitems, file)
}
pub unsafe extern "C" fn gzgetc(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzread_state_is_valid((*state).mode, (*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (*state).x.have != 0 {
        let c2rust_fresh2 = (*state).x.next;
        (*state).x.next = (*state).x.next.offset(1);
        if !gzgetc_buffer_commit_state(&mut *state) {
            return -1 as ::core::ffi::c_int;
        }
        return *c2rust_fresh2 as ::core::ffi::c_int;
    }
    return if gz_read(
        state,
        &raw mut buf as *mut ::core::ffi::c_uchar as crate::stdlib::voidp,
        1 as crate::stdlib::z_size_t,
    ) < 1 as crate::stdlib::z_size_t
    {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    };
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc(file)
}
pub unsafe extern "C" fn gzgetc_(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    return gzgetc(file);
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc_(file)
}

/// Describe where an ungot byte belongs in the existing read buffer without
/// touching its ABI cursor.  The caller keeps the pointer adjustment and byte
/// store at the raw boundary.
#[derive(Clone, Copy)]
enum GzUngetcBufferPlan {
    InvalidCharacter,
    Full,
    Empty {
        capacity: crate::stdlib::uInt,
    },
    Buffered {
        shift_to_end: bool,
        capacity: crate::stdlib::uInt,
    },
}

fn gzungetc_buffer_plan(
    c: ::core::ffi::c_int,
    have: crate::stdlib::uInt,
    size: crate::stdlib::uInt,
    next_is_output_start: bool,
) -> GzUngetcBufferPlan {
    if c < 0 {
        return GzUngetcBufferPlan::InvalidCharacter;
    }
    let capacity = size.wrapping_shl(1);
    if have == 0 {
        return GzUngetcBufferPlan::Empty { capacity };
    }
    if have >= capacity {
        return GzUngetcBufferPlan::Full;
    }
    GzUngetcBufferPlan::Buffered {
        shift_to_end: next_is_output_start,
        capacity,
    }
}

/// Commit only the scalar consequences of a successful ungetc buffer write.
/// `x.next` is deliberately left to the boundary, where it remains a raw ABI
/// cursor.
fn gzungetc_buffer_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    plan: GzUngetcBufferPlan,
) -> bool {
    let capacity = match plan {
        GzUngetcBufferPlan::InvalidCharacter | GzUngetcBufferPlan::Full => return false,
        GzUngetcBufferPlan::Empty { capacity } => {
            if state.x.have != 0 {
                return false;
            }
            capacity
        }
        GzUngetcBufferPlan::Buffered { capacity, .. } => {
            if state.x.have == 0 || state.x.have >= capacity {
                return false;
            }
            capacity
        }
    };
    if state.size.wrapping_shl(1) != capacity {
        return false;
    }
    state.x.have = state.x.have.wrapping_add(1);
    state.x.pos = state.x.pos.wrapping_sub(1);
    state.past = 0;
    true
}

/// Move the currently buffered output to the end of the output buffer before
/// inserting an ungot byte.  `copy_within` deliberately preserves the
/// overlap-safe backward copy performed by the original pointer loop.
fn gzungetc_shift_to_end(buffer: &mut [u8], have: crate::stdlib::uInt) -> Option<usize> {
    let have = have as usize;
    if have > buffer.len() {
        return None;
    }
    let start = buffer.len().checked_sub(have)?;
    buffer.copy_within(0..have, start);
    Some(start)
}

pub unsafe extern "C" fn gzungetc(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).how == crate::gzguts_h::LOOK && (*state).x.have == 0 as ::core::ffi::c_uint {
        gz_look(state);
    }
    if !gzread_state_is_valid((*state).mode, (*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (*state).skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let plan = gzungetc_buffer_plan(
        c,
        (*state).x.have,
        (*state).size,
        (*state).x.next == (*state).out,
    );
    match plan {
        GzUngetcBufferPlan::InvalidCharacter => return -1 as ::core::ffi::c_int,
        GzUngetcBufferPlan::Full => {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_DATA_ERROR,
                b"out of room to push characters\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        GzUngetcBufferPlan::Empty { .. } | GzUngetcBufferPlan::Buffered { .. } => {}
    };
    if let GzUngetcBufferPlan::Empty { capacity } = plan {
        (*state).x.next = (*state)
            .out
            .offset(capacity as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        *(*state).x.next.offset(0 as ::core::ffi::c_int as isize) = c as ::core::ffi::c_uchar;
        if !gzungetc_buffer_commit_state(&mut *state, plan) {
            return -1 as ::core::ffi::c_int;
        }
        return c;
    }
    let GzUngetcBufferPlan::Buffered { shift_to_end, .. } = plan else {
        return -1 as ::core::ffi::c_int;
    };
    if shift_to_end {
        let capacity = (*state).size.wrapping_shl(1) as usize;
        let buffer = ::core::slice::from_raw_parts_mut((*state).out, capacity);
        let Some(start) = gzungetc_shift_to_end(buffer, (*state).x.have) else {
            return -1 as ::core::ffi::c_int;
        };
        (*state).x.next = (*state).out.add(start);
    }
    (*state).x.next = (*state).x.next.offset(-1);
    *(*state).x.next.offset(0 as ::core::ffi::c_int as isize) = c as ::core::ffi::c_uchar;
    if !gzungetc_buffer_commit_state(&mut *state, plan) {
        return -1 as ::core::ffi::c_int;
    }
    return c;
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    gzungetc(c, file)
}
pub unsafe extern "C" fn gzgets(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut left: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut eol: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() || buf.is_null() || len < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzread_state_is_valid((*state).mode, (*state).err, (*state).again) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (*state).skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    str = buf;
    left = (len as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
    if left != 0 {
        while !((*state).x.have == 0 as ::core::ffi::c_uint
            && gz_fetch(state) == -1 as ::core::ffi::c_int)
        {
            if (*state).x.have == 0 as ::core::ffi::c_uint {
                (*state).past = 1 as ::core::ffi::c_int;
                break;
            } else {
                n = gzgets_buffer_copy_plan((*state).x.have, left, None);
                eol = crate::stdlib::memchr(
                    (*state).x.next as *const ::core::ffi::c_void,
                    '\n' as ::core::ffi::c_int,
                    n as crate::__stddef_size_t_h::size_t,
                ) as *mut ::core::ffi::c_uchar;
                let newline_offset = if eol.is_null() {
                    None
                } else {
                    Some(eol.offset_from((*state).x.next) as ::core::ffi::c_uint)
                };
                n = gzgets_buffer_copy_plan((*state).x.have, left, newline_offset);
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    (*state).x.next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                (*state).x.next = (*state).x.next.offset(n as isize);
                let Some(remaining) = gzgets_buffer_copy_commit_state(&mut *state, left, n) else {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                };
                left = remaining;
                buf = buf.offset(n as isize);
                if !(left != 0 && eol.is_null()) {
                    break;
                }
            }
        }
    }
    if buf == str {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *buf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    return str;
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    gzgets(file, buf, len)
}
/// Convert the direct-stream flag into the public `gzdirect` result after
/// the boundary has performed any required lazy lookahead.
fn gzdirect_state(direct: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (direct == 1) as ::core::ffi::c_int
}

/// Map the read-side saved stream status and descriptor-close result to the
/// public close status.  Resource release remains at the raw boundary.
fn gzclose_read_result(
    state_err: ::core::ffi::c_int,
    close_result: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if close_result != 0 {
        crate::zlib_h::Z_ERRNO
    } else if state_err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    }
}

#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::LOOK
        && state.x.have == 0
    {
        gz_look(state);
    }
    gzdirect_state(state.direct)
}
pub unsafe extern "C" fn gzclose_r(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).size != 0 {
        crate::src::inflate::inflateEnd(
            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
        crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
    }
    let state_err = (*state).err;
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
    let close_result = crate::stdlib::close((*state).fd);
    crate::stdlib::free(state as *mut ::core::ffi::c_void);
    gzclose_read_result(state_err, close_result)
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_r(file)
}
