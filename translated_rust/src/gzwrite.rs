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

fn gz_direct_write(fd: &rustix::fd::OwnedFd, input: &[u8]) -> Result<usize, GzWriteFailure> {
    errno::set_errno(errno::Errno(0));
    rustix::io::write(fd, input).map_err(|error| gz_write_failure(error.raw_os_error()))
}

fn clear_buffered_input(buffer: &mut [u8]) {
    buffer.fill(0);
}

// A forward seek on a write handle is materialized as zero-filled input fed
// through the normal compression path.  Keep the byte-range proof and the
// scalar accounting outside the ABI-shaped gzip state so the eventual owned
// gzip facade can drive the same transaction without retaining stream
// pointers.  `input_len` deliberately follows zlib's c_int/off64_t chunk
// selection, including its truncating cast when the remaining skip is less
// than one buffer.
struct GzZeroStep {
    input_len: crate::stdlib::uInt,
}

struct GzZeroProgress {
    position: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
}

impl GzZeroStep {
    fn new(size: crate::stdlib::uInt, skip: crate::stdlib::off64_t) -> Self {
        let input_len = if ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && size > crate::src::gzlib::gz_intmax()
            || size as crate::stdlib::off64_t > skip
        {
            skip as crate::stdlib::uInt
        } else {
            size
        };
        Self { input_len }
    }

    fn zero_input<'a>(&self, input: &'a mut [u8]) -> Option<&'a mut [u8]> {
        let input = input.get_mut(..self.input_len as usize)?;
        clear_buffered_input(input);
        Some(input)
    }

    fn finish(
        &self,
        remaining: crate::stdlib::uInt,
        position: crate::stdlib::off64_t,
        skip: crate::stdlib::off64_t,
    ) -> GzZeroProgress {
        let written = self.input_len.wrapping_sub(remaining);
        GzZeroProgress {
            position: position + written as crate::stdlib::off64_t,
            skip: skip - written as crate::stdlib::off64_t,
        }
    }
}

// The write side's owned buffers are allocated as one pointer-free
// transaction.  In particular, a failed output allocation drops the input
// allocation before any ABI-shaped gzip state is changed.  Keeping this owner
// separate lets a later gzip-core facade take over resource ownership without
// reintroducing cursor or stream pointers into its setup path.
struct GzWriteBuffers {
    input: Box<[u8]>,
    output: Option<Box<[u8]>>,
    size: ::core::ffi::c_uint,
}

impl GzWriteBuffers {
    fn allocate(want: ::core::ffi::c_uint, direct: ::core::ffi::c_int) -> Option<Self> {
        let input = crate::src::gzlib::gz_buffer(want << 1)?;
        let output = if direct == 0 {
            Some(crate::src::gzlib::gz_buffer(want)?)
        } else {
            None
        };
        Some(Self {
            input,
            output,
            size: want,
        })
    }
}

unsafe fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let Some(buffers) = GzWriteBuffers::allocate(state.want, state.direct) else {
        crate::src::gzlib::GzErrorState {
            message: &mut state.msg,
            error: &mut state.err,
            buffered: &mut state.x.have,
            again: state.again,
            path: state.path.as_deref(),
        }
        .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
        return -1 as ::core::ffi::c_int;
    };
    state.in_0 = Some(buffers.input);
    state.out = buffers.output;
    if state.direct == 0 {
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ret = crate::src::deflate::deflateInit2_(
            &raw mut state.strm as *mut crate::zlib_h::z_stream_s,
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
            crate::src::gzlib::GzErrorState {
                message: &mut state.msg,
                error: &mut state.err,
                buffered: &mut state.x.have,
                again: state.again,
                path: state.path.as_deref(),
            }
            .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
            return -1 as ::core::ffi::c_int;
        }
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    state.size = buffers.size;
    if state.direct == 0 {
        state.strm.avail_out = state.size as crate::stdlib::uInt;
        state.strm.next_out = state.out.as_deref_mut().unwrap().as_mut_ptr();
        state.x.next = state.strm.next_out as *mut ::core::ffi::c_uchar;
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
    if state.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.direct != 0 {
        while state.strm.avail_in != 0 {
            state.again = 0;
            put = if state.strm.avail_in > max {
                max
            } else {
                state.strm.avail_in as ::core::ffi::c_uint
            };
            let write = {
                let Some(buffer) = state.in_0.as_deref() else {
                    return -1;
                };
                let Some(buffered) = crate::src::gzlib::GzBufferedCursor::from_owned_buffer(
                    buffer,
                    state.strm.next_in.addr(),
                    state.strm.avail_in,
                ) else {
                    return -1;
                };
                let Some((input, _)) = buffered.consume(put as usize) else {
                    return -1;
                };
                gz_direct_write(state.fd.as_ref().unwrap(), input)
            };
            match write {
                Ok(written) => writ = written as ::core::ffi::c_int,
                Err(failure) => {
                    if failure.would_block {
                        state.again = 1;
                    }
                    let message = errno::Errno(failure.errno_value).to_string();
                    crate::src::gzlib::GzErrorState {
                        message: &mut state.msg,
                        error: &mut state.err,
                        buffered: &mut state.x.have,
                        again: state.again,
                        path: state.path.as_deref(),
                    }
                    .set(crate::zlib_h::Z_ERRNO, Some(message.as_bytes()));
                    return -1;
                }
            }
            state.strm.avail_in = state
                .strm
                .avail_in
                .wrapping_sub(writ as crate::stdlib::uInt);
            state.strm.next_in = state.strm.next_in.wrapping_add(writ as usize);
        }
        return 0;
    }
    if state.reset != 0 {
        if state.strm.avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
            return 0 as ::core::ffi::c_int;
        }
        crate::src::deflate::deflateReset(&raw mut state.strm as *mut crate::zlib_h::z_stream_s);
        state.reset = 0 as ::core::ffi::c_int;
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if state.strm.avail_out == 0 as crate::stdlib::uInt
            || flush != crate::zlib_h::Z_NO_FLUSH
                && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
        {
            while state.strm.next_out > state.x.next {
                state.again = 0 as ::core::ffi::c_int;
                let write = {
                    let Some(buffer) = state.out.as_deref() else {
                        return -1;
                    };
                    let Some(buffered_len) =
                        state.strm.next_out.addr().checked_sub(state.x.next.addr())
                    else {
                        return -1;
                    };
                    let Some(buffered_len) = ::core::ffi::c_uint::try_from(buffered_len).ok()
                    else {
                        return -1;
                    };
                    let Some(buffered) = crate::src::gzlib::GzBufferedCursor::from_owned_buffer(
                        buffer,
                        state.x.next.addr(),
                        buffered_len,
                    ) else {
                        return -1;
                    };
                    put = buffered_len.min(max);
                    let Some((input, _)) = buffered.consume(put as usize) else {
                        return -1;
                    };
                    let result = gz_direct_write(state.fd.as_ref().unwrap(), input);
                    result
                };
                match write {
                    Ok(written) => {
                        writ = written as ::core::ffi::c_int;
                        state.x.next = state.x.next.wrapping_add(written);
                    }
                    Err(failure) => {
                        if failure.would_block {
                            state.again = 1 as ::core::ffi::c_int;
                        }
                        let message = errno::Errno(failure.errno_value).to_string();
                        crate::src::gzlib::GzErrorState {
                            message: &mut state.msg,
                            error: &mut state.err,
                            buffered: &mut state.x.have,
                            again: state.again,
                            path: state.path.as_deref(),
                        }
                        .set(crate::zlib_h::Z_ERRNO, Some(message.as_bytes()));
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
            if state.strm.avail_out == 0 as crate::stdlib::uInt {
                state.strm.avail_out = state.size as crate::stdlib::uInt;
                state.strm.next_out = state.out.as_deref_mut().unwrap().as_mut_ptr();
                state.x.next = state.strm.next_out;
            }
        }
        have = state.strm.avail_out as ::core::ffi::c_uint;
        ret = crate::src::deflate::deflate(
            &raw mut state.strm as *mut crate::zlib_h::z_stream_s,
            flush,
        );
        if ret == crate::zlib_h::Z_STREAM_ERROR {
            crate::src::gzlib::GzErrorState {
                message: &mut state.msg,
                error: &mut state.err,
                buffered: &mut state.x.have,
                again: state.again,
                path: state.path.as_deref(),
            }
            .set(
                crate::zlib_h::Z_STREAM_ERROR,
                Some(b"internal error: deflate stream corrupt"),
            );
            return -1 as ::core::ffi::c_int;
        }
        have = have.wrapping_sub(state.strm.avail_out as ::core::ffi::c_uint);
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
    if state.strm.avail_in != 0
        && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    first = 1 as ::core::ffi::c_int;
    loop {
        let step = GzZeroStep::new(state.size, state.skip);
        if first != 0 {
            if state
                .in_0
                .as_deref_mut()
                .and_then(|input| step.zero_input(input))
                .is_none()
            {
                return -1;
            }
            first = 0 as ::core::ffi::c_int;
        }
        state.strm.avail_in = step.input_len;
        state.strm.next_in = state.in_0.as_deref_mut().unwrap().as_mut_ptr();
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        let progress = step.finish(state.strm.avail_in, state.x.pos, state.skip);
        state.x.pos = progress.position;
        state.skip = progress.skip;
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
        if state.direct != 0 {
            while len != 0 {
                let max = ((-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2).wrapping_add(1))
                    as usize;
                let count = input.len().min(max);
                state.again = 0;
                match gz_direct_write(state.fd.as_ref().unwrap(), &input[..count]) {
                    Ok(written) => {
                        state.x.pos += written as crate::stdlib::off64_t;
                        input = &input[written..];
                        len = len.wrapping_sub(written as crate::stdlib::z_size_t);
                    }
                    Err(failure) => {
                        if failure.would_block {
                            state.again = 1;
                        }
                        let message = errno::Errno(failure.errno_value).to_string();
                        crate::src::gzlib::GzErrorState {
                            message: &mut state.msg,
                            error: &mut state.err,
                            buffered: &mut state.x.have,
                            again: state.again,
                            path: state.path.as_deref(),
                        }
                        .set(crate::zlib_h::Z_ERRNO, Some(message.as_bytes()));
                        return if state.again != 0 {
                            put.wrapping_sub(len)
                        } else {
                            0
                        };
                    }
                }
            }
            return put;
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
    crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    }
    .clear();
    if !gzwrite_length_fits_int(input.len()) {
        crate::src::gzlib::GzErrorState {
            message: &mut state.msg,
            error: &mut state.err,
            buffered: &mut state.x.have,
            again: state.again,
            path: state.path.as_deref(),
        }
        .set(
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
    crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    }
    .clear();
    let Some(len) = gzfwrite_length(size, nitems) else {
        crate::src::gzlib::GzErrorState {
            message: &mut state.msg,
            error: &mut state.err,
            buffered: &mut state.x.have,
            again: state.again,
            path: state.path.as_deref(),
        }
        .set(
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
    crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    }
    .clear();
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.size != 0 {
        let size = state.size as usize;
        let (copy, available) = {
            let strm = &mut state.strm;
            let Some(buffer) = state.in_0.as_deref_mut() else {
                return -1 as ::core::ffi::c_int;
            };
            let Some(buffer) = buffer.get_mut(..size) else {
                return -1 as ::core::ffi::c_int;
            };
            if strm.avail_in == 0 as crate::stdlib::uInt {
                strm.next_in = buffer.as_mut_ptr();
            }
            let Some(mut buffered) = crate::src::gzlib::GzBufferedInput::from_owned_buffer(
                buffer,
                strm.next_in.addr(),
                strm.avail_in,
            ) else {
                return -1 as ::core::ffi::c_int;
            };
            let copy = buffered.append(&[c as ::core::ffi::c_uchar]);
            let Some(available) = buffered.have() else {
                return -1 as ::core::ffi::c_int;
            };
            (copy, available)
        };
        if copy != 0 {
            state.strm.avail_in = available;
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
    crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    }
    .clear();
    let len = text.len() as crate::stdlib::z_size_t;
    if !gzputs_length_fits_int(len) {
        crate::src::gzlib::GzErrorState {
            message: &mut state.msg,
            error: &mut state.err,
            buffered: &mut state.x.have,
            again: state.again,
            path: state.path.as_deref(),
        }
        .set(
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
    crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    }
    .clear();
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
    crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    }
    .clear();
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
