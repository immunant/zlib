pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_error;
pub use crate::src::gzlib::gz_intmax;
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

// Keep the errno-dependent write failure policy separate from the ABI-shaped
// gzip state.  A later FD/write facade can return this pointer-free result
// directly instead of making the state machine inspect errno itself.
struct GzWriteFailure {
    errno_value: ::core::ffi::c_int,
    would_block: bool,
}

// This is the pointer-free portion of the gzip write state that determines
// whether an operation may proceed.  Keep the policy independent from the
// ABI-shaped owner: the eventual gzip-state facade can construct this directly
// and leave all handle conversion at the boundary.
struct GzWritePolicy {
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
}

impl GzWritePolicy {
    fn accepts_write(&self) -> bool {
        self.mode == crate::gzguts_h::GZ_WRITE
            && (self.err == crate::zlib_h::Z_OK || self.again != 0)
    }

    fn accepts_params(&self) -> bool {
        self.accepts_write() && self.direct == 0
    }
}

fn gzwrite_length_fits_int(len: usize) -> bool {
    (len as ::core::ffi::c_uint as ::core::ffi::c_int) >= 0
}

fn gzputs_length_fits_int(len: crate::stdlib::z_size_t) -> bool {
    (len as ::core::ffi::c_int) >= 0 && len as ::core::ffi::c_uint as crate::stdlib::z_size_t == len
}

fn gzfwrite_length(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    let len = nitems.wrapping_mul(size);
    (size == 0 || len.wrapping_div(size) == nitems).then_some(len)
}

fn gz_write_failure(errno_value: ::core::ffi::c_int) -> GzWriteFailure {
    GzWriteFailure {
        errno_value,
        would_block: errno_value == crate::stdlib::EAGAIN
            || errno_value == crate::stdlib::EWOULDBLOCK,
    }
}

fn write_buffered_byte(buffer: &mut [u8], index: usize, byte: u8) -> bool {
    let Some(slot) = buffer.get_mut(index) else {
        return false;
    };
    *slot = byte;
    true
}

fn clear_buffered_input(buffer: &mut [u8]) {
    buffer.fill(0);
}

unsafe fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut state.strm;
    state.in_0 = crate::src::gzlib::gz_buffer(state.want << 1);
    if state.in_0.is_none() {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_MEM_ERROR,
            b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if state.direct == 0 {
        state.out = crate::src::gzlib::gz_buffer(state.want);
        if state.out.is_none() {
            state.in_0 = None;
            crate::src::gzlib::gz_error(
                state,
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
            state.level,
            8 as ::core::ffi::c_int,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            state.strategy,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        );
        if ret != crate::zlib_h::Z_OK {
            state.out = None;
            state.in_0 = None;
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*strm).next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    state.size = state.want;
    if state.direct == 0 {
        (*strm).avail_out = state.size as crate::stdlib::uInt;
        (*strm).next_out = state.out.as_deref_mut().unwrap().as_mut_ptr();
        state.x.next = (*strm).next_out as *mut ::core::ffi::c_uchar;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_comp(
    state: &mut crate::gzguts_h::gz_state,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    let mut strm: crate::zlib_h::z_streamp = &raw mut state.strm;
    if state.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.direct != 0 {
        while (*strm).avail_in != 0 {
            *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
            state.again = 0 as ::core::ffi::c_int;
            put = if (*strm).avail_in > max {
                max
            } else {
                (*strm).avail_in as ::core::ffi::c_uint
            };
            writ = crate::stdlib::write(
                <rustix::fd::OwnedFd as rustix::fd::AsRawFd>::as_raw_fd(state.fd.as_ref().unwrap()),
                (*strm).next_in as *const ::core::ffi::c_void,
                put as crate::__stddef_size_t_h::size_t,
            ) as ::core::ffi::c_int;
            if writ < 0 as ::core::ffi::c_int {
                let failure = gz_write_failure(*crate::stdlib::__errno_location());
                if failure.would_block {
                    state.again = 1 as ::core::ffi::c_int;
                }
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_ERRNO,
                    crate::stdlib::strerror(failure.errno_value),
                );
                return -1 as ::core::ffi::c_int;
            }
            (*strm).avail_in = (*strm).avail_in.wrapping_sub(writ as ::core::ffi::c_uint);
            (*strm).next_in = (*strm).next_in.wrapping_add(writ as usize);
        }
        return 0 as ::core::ffi::c_int;
    }
    if state.reset != 0 {
        if (*strm).avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
            return 0 as ::core::ffi::c_int;
        }
        crate::src::deflate::deflateReset(strm as *mut crate::zlib_h::z_stream_s);
        state.reset = 0 as ::core::ffi::c_int;
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if (*strm).avail_out == 0 as crate::stdlib::uInt
            || flush != crate::zlib_h::Z_NO_FLUSH
                && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
        {
            while (*strm).next_out > state.x.next {
                *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                state.again = 0 as ::core::ffi::c_int;
                let buffered = (*strm).next_out.addr().wrapping_sub(state.x.next.addr());
                put = if buffered > max as usize {
                    max
                } else {
                    buffered as ::core::ffi::c_uint
                };
                writ = crate::stdlib::write(
                    <rustix::fd::OwnedFd as rustix::fd::AsRawFd>::as_raw_fd(
                        state.fd.as_ref().unwrap(),
                    ),
                    state.x.next as *const ::core::ffi::c_void,
                    put as crate::__stddef_size_t_h::size_t,
                ) as ::core::ffi::c_int;
                if writ < 0 as ::core::ffi::c_int {
                    let failure = gz_write_failure(*crate::stdlib::__errno_location());
                    if failure.would_block {
                        state.again = 1 as ::core::ffi::c_int;
                    }
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_ERRNO,
                        crate::stdlib::strerror(failure.errno_value),
                    );
                    return -1 as ::core::ffi::c_int;
                }
                state.x.next = state.x.next.wrapping_add(writ as usize);
            }
            if (*strm).avail_out == 0 as crate::stdlib::uInt {
                (*strm).avail_out = state.size as crate::stdlib::uInt;
                (*strm).next_out = state.out.as_deref_mut().unwrap().as_mut_ptr();
                state.x.next = (*strm).next_out;
            }
        }
        have = (*strm).avail_out as ::core::ffi::c_uint;
        ret = crate::src::deflate::deflate(strm as *mut crate::zlib_h::z_stream_s, flush);
        if ret == crate::zlib_h::Z_STREAM_ERROR {
            crate::src::gzlib::gz_error(
                state,
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
        state.reset = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_zero(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut state.strm;
    if (*strm).avail_in != 0
        && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    first = 1 as ::core::ffi::c_int;
    loop {
        n = if ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && state.size > crate::src::gzlib::gz_intmax()
            || state.size as crate::stdlib::off64_t > state.skip
        {
            state.skip as ::core::ffi::c_uint
        } else {
            state.size
        };
        if first != 0 {
            let input = &mut state.in_0.as_deref_mut().unwrap()[..n as usize];
            clear_buffered_input(input);
            first = 0 as ::core::ffi::c_int;
        }
        (*strm).avail_in = n as crate::stdlib::uInt;
        (*strm).next_in = state.in_0.as_deref_mut().unwrap().as_mut_ptr();
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        n = n.wrapping_sub((*strm).avail_in as ::core::ffi::c_uint);
        state.x.pos += n as crate::stdlib::off64_t;
        state.skip -= n as crate::stdlib::off64_t;
        if ret == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if state.skip == 0 {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_write(
    state: &mut crate::gzguts_h::gz_state,
    mut input: &[u8],
) -> crate::stdlib::z_size_t {
    let mut len = input.len() as crate::stdlib::z_size_t;
    let mut put: crate::stdlib::z_size_t = len;
    let mut ret: ::core::ffi::c_int = 0;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if len < state.size as crate::stdlib::z_size_t {
        loop {
            let (copy, have) = {
                let strm = &mut state.strm;
                let buffer = &mut state.in_0.as_deref_mut().unwrap()[..state.size as usize];
                if strm.avail_in == 0 as crate::stdlib::uInt {
                    strm.next_in = buffer.as_mut_ptr();
                }
                let Some(mut buffered) = crate::src::gzlib::GzBufferedInput::from_owned_buffer(
                    buffer,
                    strm.next_in.addr(),
                    strm.avail_in,
                ) else {
                    return 0 as crate::stdlib::z_size_t;
                };
                let copy = buffered.append(input);
                let Some(have) = buffered.have() else {
                    return 0 as crate::stdlib::z_size_t;
                };
                (copy, have)
            };
            state.strm.avail_in = have;
            state.x.pos += copy as crate::stdlib::off64_t;
            input = &input[copy..];
            len = len.wrapping_sub(copy as crate::stdlib::z_size_t);
            if len == 0 as crate::stdlib::z_size_t {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return if state.again != 0 {
                    put.wrapping_sub(len)
                } else {
                    0 as crate::stdlib::z_size_t
                };
            }
        }
    } else {
        if state.strm.avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
        {
            return 0 as crate::stdlib::z_size_t;
        }
        state.strm.next_in = input.as_ptr() as *mut crate::stdlib::Bytef;
        loop {
            let mut n: ::core::ffi::c_uint = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
            if n as crate::stdlib::z_size_t > len {
                n = len as ::core::ffi::c_uint;
            }
            state.strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            n = n.wrapping_sub(state.strm.avail_in as ::core::ffi::c_uint);
            state.x.pos += n as crate::stdlib::off64_t;
            input = &input[n as usize..];
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            if ret == -1 as ::core::ffi::c_int {
                return if state.again != 0 {
                    put.wrapping_sub(len)
                } else {
                    0 as crate::stdlib::z_size_t
                };
            }
            if len == 0 {
                break;
            }
        }
    }
    return put;
}
unsafe fn gzwrite(state: &mut crate::gzguts_h::gz_state, input: &[u8]) -> ::core::ffi::c_int {
    let policy = GzWritePolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
        direct: state.direct,
    };
    if !policy.accepts_write() {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    if !gzwrite_length_fits_int(input.len()) {
        crate::src::gzlib::gz_set_error(
            &mut state.msg,
            &mut state.err,
            &mut state.x.have,
            state.again,
            state.path.as_deref(),
            crate::zlib_h::Z_DATA_ERROR,
            Some(b"requested length does not fit in int"),
        );
        return 0 as ::core::ffi::c_int;
    }
    return gz_write(state, input) as ::core::ffi::c_int;
}
#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let input = if len == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(buf.cast::<u8>(), len as usize)
    };
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0 as ::core::ffi::c_int;
    };
    gzwrite(state, input)
}
unsafe fn gzfwrite(
    state: &mut crate::gzguts_h::gz_state,
    input: &[u8],
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let policy = GzWritePolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
        direct: state.direct,
    };
    if !policy.accepts_write() {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    let Some(len) = gzfwrite_length(size, nitems) else {
        crate::src::gzlib::gz_set_error(
            &mut state.msg,
            &mut state.err,
            &mut state.x.have,
            state.again,
            state.path.as_deref(),
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in a size_t"),
        );
        return 0 as crate::stdlib::z_size_t;
    };
    return if len != 0 {
        gz_write(state, input).wrapping_div(size)
    } else {
        0 as crate::stdlib::z_size_t
    };
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let input = match size.checked_mul(nitems) {
        Some(0) | None => &[],
        Some(len) => ::core::slice::from_raw_parts(buf.cast::<u8>(), len),
    };
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0 as crate::stdlib::z_size_t;
    };
    gzfwrite(state, input, size, nitems)
}
unsafe fn gzputc(
    state: &mut crate::gzguts_h::gz_state,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut have: ::core::ffi::c_uint = 0;
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let policy = GzWritePolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
        direct: state.direct,
    };
    if !policy.accepts_write() {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.size != 0 {
        let size = state.size;
        let Some(in_0) = state.in_0.as_deref().map(|buffer| buffer.as_ptr()) else {
            return -1 as ::core::ffi::c_int;
        };
        let (next_in, avail_in) = {
            let strm = &mut state.strm;
            if strm.avail_in == 0 as crate::stdlib::uInt {
                strm.next_in = in_0 as *mut crate::stdlib::Bytef;
            }
            (strm.next_in, strm.avail_in)
        };
        let Some(end) = next_in.addr().checked_add(avail_in as usize) else {
            return -1 as ::core::ffi::c_int;
        };
        let Some(have_at) = end.checked_sub(in_0.addr()) else {
            return -1 as ::core::ffi::c_int;
        };
        let Ok(have_value) = ::core::ffi::c_uint::try_from(have_at) else {
            return -1 as ::core::ffi::c_int;
        };
        have = have_value;
        if have < size {
            let Some(buffer) = state.in_0.as_deref_mut() else {
                return -1 as ::core::ffi::c_int;
            };
            if !write_buffered_byte(buffer, have as usize, c as ::core::ffi::c_uchar) {
                return -1 as ::core::ffi::c_int;
            }
            state.strm.avail_in = state.strm.avail_in.wrapping_add(1);
            state.x.pos += 1;
            return c & 0xff as ::core::ffi::c_int;
        }
    }
    buf[0 as usize] = c as ::core::ffi::c_uchar;
    if gz_write(state, &buf) != 1 as crate::stdlib::z_size_t {
        return -1 as ::core::ffi::c_int;
    }
    return c & 0xff as ::core::ffi::c_int;
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzputc(state, c)
}
unsafe fn gzputs(state: &mut crate::gzguts_h::gz_state, text: &[u8]) -> ::core::ffi::c_int {
    let policy = GzWritePolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
        direct: state.direct,
    };
    if !policy.accepts_write() {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    let len = text.len() as crate::stdlib::z_size_t;
    if !gzputs_length_fits_int(len) {
        crate::src::gzlib::gz_set_error(
            &mut state.msg,
            &mut state.err,
            &mut state.x.have,
            state.again,
            state.path.as_deref(),
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"string length does not fit in int"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let put = gz_write(state, text);
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
    if file.is_null() || s.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let text = ::core::ffi::CStr::from_ptr(s).to_bytes();
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzputs(state, text)
}
unsafe fn gzflush(
    state: &mut crate::gzguts_h::gz_state,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let policy = GzWritePolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
        direct: state.direct,
    };
    if !policy.accepts_write() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    if flush < 0 as ::core::ffi::c_int || flush > crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    gz_comp(state, flush);
    return state.err;
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzflush(state, flush)
}
unsafe fn gzsetparams(
    state: &mut crate::gzguts_h::gz_state,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let policy = GzWritePolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
        direct: state.direct,
    };
    if !policy.accepts_params() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    if level == state.level && strategy == state.strategy {
        return crate::zlib_h::Z_OK;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    if state.size != 0 {
        if state.strm.avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
        {
            return state.err;
        }
        crate::src::deflate::deflateParams(
            &mut state.strm as *mut crate::zlib_h::z_stream_s,
            level,
            strategy,
        );
    }
    state.level = level;
    state.strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzsetparams(state, level, strategy)
}
pub unsafe fn gzclose_w(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    if state.mode != crate::gzguts_h::GZ_WRITE {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        ret = state.err;
    }
    if gz_comp(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int {
        ret = state.err;
    }
    if state.size != 0 {
        if state.direct == 0 {
            crate::src::deflate::deflateEnd(
                &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            );
            state.out = None;
        }
        state.in_0 = None;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    state.path = None;
    state.msg = None;
    if state
        .fd
        .take()
        .map(|fd| unsafe {
            rustix::io::try_close(<rustix::fd::OwnedFd as rustix::fd::IntoRawFd>::into_raw_fd(
                fd,
            ))
        })
        .transpose()
        .is_err()
    {
        ret = crate::zlib_h::Z_ERRNO;
    }
    return ret;
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    crate::src::gzclose::gzclose(
        ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep),
        crate::src::gzclose::GzCloseTarget::Write,
    )
}
