pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZIP;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::LOOK;
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

enum GzLoadBuffer<'a> {
    Slice(&'a mut [u8]),
    Output,
}

/// Destination selection for one fetch of readable gzip data.
///
/// Most fetches fill the state-owned output buffer.  Large reads can instead
/// decompress directly into the caller's already-bounded output slice, while
/// retaining the same state transition in `gz_fetch`.
enum GzFetchOutput<'a> {
    StateOutput,
    Direct(&'a mut [u8]),
}

/// The owned output-buffer cursor used by the read side of a gzip handle.
///
/// `gzFile_s::next` remains the ABI-visible cursor, but pushback only needs
/// an offset into the state-owned buffer.  Keeping that transition separate
/// makes the buffer manipulation independent of the raw ABI cursor.
#[derive(Clone, Copy)]
struct GzReadCursor {
    have: usize,
    next: usize,
    pos: crate::stdlib::off64_t,
}

enum GzUngetcError {
    InvalidCharacter,
    StateCorrupt,
    OutOfRoom,
}

/// The subset of a gzip reader needed to report pushback errors.
///
/// This deliberately keeps the public `gzFile_s` cursor out of the pushback
/// implementation.  The state adapter performs the ABI cursor conversion
/// before and after calling it.
struct GzUngetcErrorState<'a> {
    have: &'a mut crate::stdlib::uInt,
    again: ::core::ffi::c_int,
    err: &'a mut ::core::ffi::c_int,
    path: &'a ::core::ffi::CStr,
    msg: &'a mut Option<std::ffi::CString>,
}

impl GzUngetcErrorState<'_> {
    fn set(&mut self, err: ::core::ffi::c_int, msg: Option<&::core::ffi::CStr>) {
        *self.msg = None;
        if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && self.again == 0 {
            *self.have = 0;
        }
        *self.err = err;
        let Some(msg) = msg else {
            return;
        };
        if err == crate::zlib_h::Z_MEM_ERROR {
            return;
        }
        let path = self.path.to_bytes();
        let message = msg.to_bytes();
        let Some(size) = path
            .len()
            .checked_add(2)
            .and_then(|size| size.checked_add(message.len()))
            .and_then(|size| size.checked_add(1))
        else {
            *self.err = crate::zlib_h::Z_MEM_ERROR;
            return;
        };
        let mut text = Vec::new();
        if text.try_reserve_exact(size).is_err() {
            *self.err = crate::zlib_h::Z_MEM_ERROR;
            return;
        }
        text.extend_from_slice(path);
        text.extend_from_slice(b": ");
        text.extend_from_slice(message);
        text.push(0);
        *self.msg = Some(
            std::ffi::CString::from_vec_with_nul(text)
                .expect("a concatenation of C strings contains only its final NUL"),
        );
    }
}

/// Pointer-free state for the byte insertion half of `gzungetc`.
struct GzUngetcState<'a> {
    cursor: GzReadCursor,
    output: &'a mut [u8],
    past: &'a mut ::core::ffi::c_int,
    error: GzUngetcErrorState<'a>,
}

/// Insert one byte before the unread portion of an owned gzip output buffer.
///
/// The caller converts the ABI cursor to and from `GzReadCursor`; this helper
/// deliberately operates only on an owned byte slice and checked indices.
fn gzungetc_cursor(
    c: ::core::ffi::c_int,
    cursor: &mut GzReadCursor,
    output: &mut [u8],
) -> Result<(), GzUngetcError> {
    if c < 0 {
        return Err(GzUngetcError::InvalidCharacter);
    }
    if cursor.have == 0 {
        let last = output
            .len()
            .checked_sub(1)
            .ok_or(GzUngetcError::StateCorrupt)?;
        output[last] = c as ::core::ffi::c_uchar;
        cursor.have = 1;
        cursor.next = last;
        cursor.pos -= 1;
        return Ok(());
    }
    if cursor.have == output.len() {
        return Err(GzUngetcError::OutOfRoom);
    }
    if cursor.next > output.len() || cursor.have > output.len() - cursor.next {
        return Err(GzUngetcError::StateCorrupt);
    }

    if cursor.next == 0 {
        let moved = output.len() - cursor.have;
        output.copy_within(..cursor.have, moved);
        cursor.next = moved;
    }
    cursor.next = cursor
        .next
        .checked_sub(1)
        .ok_or(GzUngetcError::StateCorrupt)?;
    output[cursor.next] = c as ::core::ffi::c_uchar;
    cursor.have += 1;
    cursor.pos -= 1;
    Ok(())
}

/// Read from a descriptor that is owned by the gzip state for the duration of
/// the call. The borrowed descriptor and slice cover exactly one `read`.
fn gz_read_fd(fd: &std::os::fd::OwnedFd, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
    rustix::io::read(fd, buffer).map_err(std::io::Error::from)
}

fn gz_load_impl(
    state: &mut crate::gzguts_h::gz_state,
    buffer: GzLoadBuffer<'_>,
) -> Result<usize, ()> {
    let buf = match buffer {
        GzLoadBuffer::Slice(buf) => buf,
        GzLoadBuffer::Output => {
            let Some(len) = (state.size as usize).checked_mul(2) else {
                return Err(());
            };
            if len > state.out.len() {
                return Err(());
            }
            &mut state.out[..len]
        }
    };
    let mut have = 0usize;
    let max = ((-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2) + 1) as usize;
    state.again = 0 as ::core::ffi::c_int;
    errno::set_errno(errno::Errno(0));
    loop {
        let get = (buf.len() - have).min(max);
        let Some(fd) = state.fd.as_ref() else {
            return Err(());
        };
        match gz_read_fd(fd, &mut buf[have..have + get]) {
            Ok(0) => {
                state.eof = 1 as ::core::ffi::c_int;
                break;
            }
            Ok(read) => {
                have += read;
                if have >= buf.len() {
                    break;
                }
            }
            Err(error) => {
                let errno = error.raw_os_error().unwrap_or(0);
                if errno == crate::stdlib::EAGAIN || errno == crate::stdlib::EWOULDBLOCK {
                    state.again = 1 as ::core::ffi::c_int;
                    if have != 0 {
                        return Ok(have);
                    }
                }
                // `Errno` obtains the platform's strerror text through its
                // safe API, so the gzip implementation neither calls libc
                // directly nor borrows a C string with an unbounded lifetime.
                let message = std::ffi::CString::new(errno::Errno(errno).to_string()).ok();
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_ERRNO,
                    message.as_deref(),
                );
                return Err(());
            }
        }
    }
    Ok(have)
}

fn gz_avail_impl(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if state.eof == 0 as ::core::ffi::c_int {
        let available = state.strm.avail_in as usize;
        let capacity = state.size as usize;
        if available > capacity || state.in_0.len() != capacity {
            return -1 as ::core::ffi::c_int;
        }
        let input_start = state
            .strm
            .next_in
            .addr()
            .wrapping_sub(state.in_0.as_ptr().addr());
        if available != 0 && (input_start > capacity || available > capacity - input_start) {
            return -1 as ::core::ffi::c_int;
        }
        // Move the owner out while refilling so the read operation can update
        // state without aliasing the staging slice. Moving a `Vec` preserves
        // its allocation and therefore the stream cursor remains valid until
        // it is reinstalled below.
        let mut input = std::mem::take(&mut state.in_0);
        if available != 0 && input_start != 0 {
            input.copy_within(input_start..input_start + available, 0);
        }
        let got = gz_load_impl(state, GzLoadBuffer::Slice(&mut input[available..]));
        state.in_0 = input;
        let got = match got {
            Ok(got) => got,
            Err(()) => return -1 as ::core::ffi::c_int,
        };
        state.strm.avail_in = (available + got) as crate::stdlib::uInt;
        state.strm.next_in = state.in_0.as_mut_ptr() as *mut crate::stdlib::Bytef;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_look(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size == 0 as ::core::ffi::c_uint {
        let input_len = state.want as usize;
        let Some(output_len) = (state.want as usize).checked_mul(2) else {
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(c"out of memory"),
            );
            return -1 as ::core::ffi::c_int;
        };
        if state.in_0.try_reserve_exact(input_len).is_ok()
            && state.out.try_reserve_exact(output_len).is_ok()
        {
            state.in_0.resize(input_len, 0);
            state.out.resize(output_len, 0);
        }
        if state.in_0.len() != input_len || state.out.len() != output_len {
            state.in_0.clear();
            state.out.clear();
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(c"out of memory"),
            );
            return -1 as ::core::ffi::c_int;
        }
        state.size = state.want;
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        state.strm.avail_in = 0 as crate::stdlib::uInt;
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit2_(
            &mut state.strm,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            state.in_0.clear();
            state.out.clear();
            state.size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(c"out of memory"),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if state.direct == -1 as ::core::ffi::c_int || state.junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflate_reset_gzip(&mut state.strm);
        state.how = crate::gzguts_h::GZIP;
        state.junk = (state.junk != -1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        state.direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail_impl(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.strm.avail_in == 0 as crate::stdlib::uInt
        || state.again != 0 && state.strm.avail_in < 4 as crate::stdlib::uInt
    {
        return 0 as ::core::ffi::c_int;
    }
    let input_start = state
        .strm
        .next_in
        .addr()
        .wrapping_sub(state.in_0.as_ptr().addr());
    let input_len = state.strm.avail_in as usize;
    if input_start > state.in_0.len() || input_len > state.in_0.len() - input_start {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let input = &state.in_0[input_start..input_start + input_len];
    if input.len() > 3 && input[0] == 31 && input[1] == 139 && input[2] == 8 && input[3] < 32 {
        crate::src::inflate::inflate_reset_gzip(&mut state.strm);
        state.how = crate::gzguts_h::GZIP;
        state.junk = 1 as ::core::ffi::c_int;
        state.direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if input.len() > state.out.len() {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    }
    state.out[..input.len()].copy_from_slice(input);
    state.x.next = state.out.as_mut_ptr();
    state.x.have = state.strm.avail_in as ::core::ffi::c_uint;
    state.strm.avail_in = 0 as crate::stdlib::uInt;
    state.how = crate::gzguts_h::COPY;
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_decomp(
    state: &mut crate::gzguts_h::gz_state,
    output: &mut [u8],
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let had = state.strm.avail_out as ::core::ffi::c_uint;
    loop {
        if state.strm.avail_in == 0 as crate::stdlib::uInt
            && gz_avail_impl(state) == -1 as ::core::ffi::c_int
        {
            ret = state.err;
            break;
        } else if state.strm.avail_in == 0 as crate::stdlib::uInt {
            if state.again == 0 {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_BUF_ERROR,
                    Some(c"unexpected end of file"),
                );
            }
            break;
        } else {
            let input_len = state.strm.avail_in as usize;
            let Some(input_start) = state
                .strm
                .next_in
                .addr()
                .checked_sub(state.in_0.as_ptr().addr())
            else {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"state corrupt"),
                );
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break;
            };
            if input_start > state.in_0.len() || input_len > state.in_0.len() - input_start {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"state corrupt"),
                );
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break;
            }
            let output_len = state.strm.avail_out as usize;
            if output_len > output.len() {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"state corrupt"),
                );
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break;
            }
            let input = &state.in_0[input_start..input_start + input_len];
            let mut inflate_message = None;
            ret = crate::src::inflate::inflate(
                &mut state.strm,
                crate::zlib_h::Z_NO_FLUSH,
                input,
                &mut output[..output_len],
                &mut inflate_message,
            );
            if state.strm.avail_out < had {
                state.junk = 0 as ::core::ffi::c_int;
            }
            if ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"internal error: inflate stream corrupt"),
                );
                break;
            } else if ret == crate::zlib_h::Z_MEM_ERROR {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_MEM_ERROR,
                    Some(c"out of memory"),
                );
                break;
            } else if ret == crate::zlib_h::Z_DATA_ERROR {
                if state.junk == 1 as ::core::ffi::c_int {
                    state.strm.avail_in = 0 as crate::stdlib::uInt;
                    state.eof = 1 as ::core::ffi::c_int;
                    state.how = crate::gzguts_h::LOOK;
                    ret = crate::zlib_h::Z_OK;
                    break;
                } else {
                    let message = inflate_message.unwrap_or(c"compressed data error");
                    crate::src::gzlib::gz_error_state(
                        state,
                        crate::zlib_h::Z_DATA_ERROR,
                        Some(message),
                    );
                    break;
                }
            } else if !(state.strm.avail_out != 0 && ret != crate::zlib_h::Z_STREAM_END) {
                break;
            }
        }
    }
    state.x.have =
        (had as crate::stdlib::uInt).wrapping_sub(state.strm.avail_out) as ::core::ffi::c_uint;
    state.x.next = output.as_mut_ptr() as *mut ::core::ffi::c_uchar;
    if ret == crate::zlib_h::Z_STREAM_END {
        state.junk = 0 as ::core::ffi::c_int;
        state.how = crate::gzguts_h::LOOK;
        return 0 as ::core::ffi::c_int;
    }
    return if ret != crate::zlib_h::Z_OK {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}

unsafe fn gz_fetch(
    state: &mut crate::gzguts_h::gz_state,
    mut output: GzFetchOutput<'_>,
) -> ::core::ffi::c_int {
    loop {
        match state.how {
            crate::gzguts_h::LOOK => {
                if gz_look(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                if state.how == crate::gzguts_h::LOOK {
                    return 0 as ::core::ffi::c_int;
                }
            }
            crate::gzguts_h::COPY => {
                if !matches!(&output, GzFetchOutput::StateOutput) {
                    crate::src::gzlib::gz_error_state(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(c"state corrupt"),
                    );
                    return -1 as ::core::ffi::c_int;
                }
                state.x.have = match gz_load_impl(state, GzLoadBuffer::Output) {
                    Ok(got) => got as ::core::ffi::c_uint,
                    Err(()) => return -1 as ::core::ffi::c_int,
                };
                state.x.next = state.out.as_mut_ptr();
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                let direct = matches!(&output, GzFetchOutput::Direct(_));
                // Move the state-owned output away only while its slice is
                // passed independently of the gzip state.  A direct read
                // selects the caller's bounded output slice instead.
                let mut state_output = None;
                let output = match &mut output {
                    GzFetchOutput::StateOutput => {
                        state_output = Some(std::mem::take(&mut state.out));
                        state_output
                            .as_deref_mut()
                            .expect("state output was installed")
                    }
                    GzFetchOutput::Direct(output) => &mut **output,
                };
                state.strm.avail_out = output.len() as crate::stdlib::uInt;
                state.strm.next_out = output.as_mut_ptr();
                let result = gz_decomp(state, output);
                if let Some(output) = state_output {
                    state.out = output;
                }
                if result == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                if direct {
                    return 0 as ::core::ffi::c_int;
                }
            }
            _ => {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"state corrupt"),
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if !(state.x.have == 0 as ::core::ffi::c_uint
            && (state.eof == 0 || state.strm.avail_in != 0))
        {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_skip(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_uint = 0;
    loop {
        if state.x.have != 0 {
            n = if ::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<crate::stdlib::off64_t>()
                && state.x.have > crate::src::gzlib::gz_intmax()
                || state.x.have as crate::stdlib::off64_t > state.skip
            {
                state.skip as ::core::ffi::c_uint
            } else {
                state.x.have
            };
            let Some(start) = state
                .out
                .iter()
                .position(|byte| std::ptr::eq(byte, state.x.next))
            else {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"state corrupt"),
                );
                return -1 as ::core::ffi::c_int;
            };
            let Some(next) = start
                .checked_add(n as usize)
                .filter(|next| *next <= state.out.len())
            else {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"state corrupt"),
                );
                return -1 as ::core::ffi::c_int;
            };
            if start > state.out.len() || state.x.have as usize > state.out.len() - start {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"state corrupt"),
                );
                return -1 as ::core::ffi::c_int;
            }
            state.x.have = state.x.have.wrapping_sub(n);
            state.x.next = state.out[next..].as_mut_ptr();
            state.x.pos += n as crate::stdlib::off64_t;
            state.skip -= n as crate::stdlib::off64_t;
        } else {
            if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_fetch(state, GzFetchOutput::StateOutput) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if state.skip == 0 {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_read_impl(
    state: &mut crate::gzguts_h::gz_state,
    buf: &mut [u8],
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut len = buf.len();
    let mut out = 0usize;
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    's_140: loop {
        n = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
        if n as usize > len {
            n = len as ::core::ffi::c_uint;
        }
        's_28: {
            if state.x.have != 0 {
                if state.x.have < n {
                    n = state.x.have;
                }
                let count = n as usize;
                let Some(start) = state.x.next.addr().checked_sub(state.out.as_ptr().addr()) else {
                    crate::src::gzlib::gz_error_state(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(c"internal error: output buffer corrupt"),
                    );
                    n = 0;
                    err = -1;
                    break 's_28;
                };
                let Some(end) = start
                    .checked_add(count)
                    .filter(|end| *end <= state.out.len())
                else {
                    crate::src::gzlib::gz_error_state(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(c"internal error: output buffer corrupt"),
                    );
                    n = 0;
                    err = -1;
                    break 's_28;
                };
                buf[out..out + count].copy_from_slice(&state.out[start..end]);
                state.x.next = state.x.next.wrapping_add(count);
                state.x.have = state.x.have.wrapping_sub(n);
                if state.err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
            } else {
                if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                    break 's_140;
                }
                let direct = state.how != crate::gzguts_h::LOOK
                    && n >= state.size << 1 as ::core::ffi::c_int
                    && state.how != crate::gzguts_h::COPY;
                let fetch_output = if state.how == crate::gzguts_h::LOOK
                    || n < state.size << 1 as ::core::ffi::c_int
                {
                    Some(GzFetchOutput::StateOutput)
                } else if state.how == crate::gzguts_h::COPY {
                    match gz_load_impl(state, GzLoadBuffer::Slice(&mut buf[out..out + n as usize]))
                    {
                        Ok(got) => n = got as ::core::ffi::c_uint,
                        Err(()) => err = -1 as ::core::ffi::c_int,
                    }
                    None
                } else {
                    Some(GzFetchOutput::Direct(&mut buf[out..out + n as usize]))
                };
                if let Some(fetch_output) = fetch_output {
                    let fetched = gz_fetch(state, fetch_output);
                    if direct {
                        err = fetched;
                        n = state.x.have;
                        state.x.have = 0 as ::core::ffi::c_uint;
                    } else {
                        if fetched == -1 as ::core::ffi::c_int
                            && state.x.have == 0 as ::core::ffi::c_uint
                        {
                            err = -1 as ::core::ffi::c_int;
                        }
                        break 's_28;
                    }
                }
            }
            len -= n as usize;
            out += n as usize;
            got = got.wrapping_add(n as crate::stdlib::z_size_t);
            state.x.pos += n as crate::stdlib::off64_t;
        }
        if !(len != 0 && err == 0) {
            break;
        }
    }
    if len != 0 && state.eof != 0 {
        state.past = 1 as ::core::ffi::c_int;
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
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).err != crate::zlib_h::Z_OK
        && (*state).err != crate::zlib_h::Z_BUF_ERROR
        && (*state).again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *state;
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"request does not fit in an int"),
        );
        return -1 as ::core::ffi::c_int;
    }
    len = if len == 0 {
        0
    } else {
        gz_read_impl(
            state,
            ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len as crate::stdlib::z_size_t),
        ) as ::core::ffi::c_uint
    };
    if len == 0 as ::core::ffi::c_uint {
        if (*state).err != crate::zlib_h::Z_OK && (*state).err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if (*state).again != 0 {
            let message = crate::stdlib::strerror(*crate::stdlib::__errno_location());
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_ERRNO,
                (!message.is_null()).then(|| ::core::ffi::CStr::from_ptr(message)),
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
enum GzfreadBuffer<'a> {
    Empty,
    Bytes(&'a mut [u8]),
    Overflow,
}

fn gzfread_impl(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    buffer: GzfreadBuffer<'_>,
) -> crate::stdlib::z_size_t {
    if state.mode != crate::gzguts_h::GZ_READ {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    let buffer = match buffer {
        GzfreadBuffer::Empty => return 0,
        GzfreadBuffer::Bytes(buffer) => buffer,
        GzfreadBuffer::Overflow => {
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"request does not fit in a size_t"),
            );
            return 0;
        }
    };
    unsafe { gz_read_impl(state, buffer) }.wrapping_div(size)
}
#[export_name = "gzfread"]

pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0;
    };
    let buffer = match nitems.checked_mul(size) {
        Some(0) => GzfreadBuffer::Empty,
        Some(len) => GzfreadBuffer::Bytes(::core::slice::from_raw_parts_mut(buf.cast(), len)),
        None => GzfreadBuffer::Overflow,
    };
    gzfread_impl(state, size, buffer)
}
fn gzgetc_impl(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if state.x.have != 0 {
        let have = state.x.have as usize;
        let Some(start) = state.x.next.addr().checked_sub(state.out.as_ptr().addr()) else {
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"state corrupt"),
            );
            return -1 as ::core::ffi::c_int;
        };
        if start >= state.out.len() || have > state.out.len() - start {
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"state corrupt"),
            );
            return -1 as ::core::ffi::c_int;
        }
        let byte = state.out[start];
        state.x.have -= 1;
        state.x.pos += 1;
        state.x.next = state.out.as_mut_ptr().wrapping_add(start + 1);
        return byte as ::core::ffi::c_int;
    }
    let mut buf = [0u8; 1];
    if unsafe { gz_read_impl(state, &mut buf) } < 1 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        buf[0] as ::core::ffi::c_int
    }
}

#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzgetc_impl(&mut *(file as crate::gzguts_h::gz_statep))
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzgetc_impl(&mut *(file as crate::gzguts_h::gz_statep))
}
/// Insert a byte using only the checked cursor view of the gzip output.
fn gzungetc(c: ::core::ffi::c_int, state: &mut GzUngetcState<'_>) -> ::core::ffi::c_int {
    match gzungetc_cursor(c, &mut state.cursor, state.output) {
        Ok(()) => {
            *state.past = 0;
            c
        }
        Err(GzUngetcError::InvalidCharacter) => -1,
        Err(GzUngetcError::OutOfRoom) => {
            state.error.set(
                crate::zlib_h::Z_DATA_ERROR,
                Some(c"out of room to push characters"),
            );
            -1
        }
        Err(GzUngetcError::StateCorrupt) => {
            state
                .error
                .set(crate::zlib_h::Z_STREAM_ERROR, Some(c"state corrupt"));
            -1
        }
    }
}

/// Prepare the ABI-backed gzip reader, then dispatch pushback through the
/// pointer-free cursor facade above.
fn gzungetc_state(
    c: ::core::ffi::c_int,
    state: &mut crate::gzguts_h::gz_state,
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.how == crate::gzguts_h::LOOK && state.x.have == 0 as ::core::ffi::c_uint {
        unsafe { gz_look(state) };
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if state.skip != 0 && unsafe { gz_skip(state) } == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let Some(output_len) = (state.size as usize).checked_mul(2) else {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    };
    if state.out.len() != output_len {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let have = state.x.have as usize;
    let next = if have == 0 {
        output_len
    } else {
        let Some(next) = state.x.next.addr().checked_sub(state.out.as_ptr().addr()) else {
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"state corrupt"),
            );
            return -1 as ::core::ffi::c_int;
        };
        next
    };
    let cursor = GzReadCursor {
        have,
        next,
        pos: state.x.pos,
    };
    let (result, cursor) = {
        let mut ungetc = GzUngetcState {
            cursor,
            output: &mut state.out,
            past: &mut state.past,
            error: GzUngetcErrorState {
                have: &mut state.x.have,
                again: state.again,
                err: &mut state.err,
                path: state.path.as_c_str(),
                msg: &mut state.msg,
            },
        };
        let result = gzungetc(c, &mut ungetc);
        (result, ungetc.cursor)
    };
    if result >= 0 {
        state.x.have = cursor.have as ::core::ffi::c_uint;
        state.x.next = state.out.as_mut_ptr().wrapping_add(cursor.next);
        state.x.pos = cursor.pos;
    }
    result
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzungetc_state(c, state)
}
/// Read a line into `buf`, which includes room for the terminating NUL.
///
/// `x.next` is an ABI cursor into the state-owned output buffer.  Keep the
/// pointer-to-range conversion here so the line-copying path itself only
/// operates on checked slices.
fn gzgets_impl(state: &mut crate::gzguts_h::gz_state, buf: &mut [u8]) -> bool {
    if state.mode != crate::gzguts_h::GZ_READ {
        return false;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return false;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if state.skip != 0 && unsafe { gz_skip(state) } == -1 as ::core::ffi::c_int {
        return false;
    }

    let mut written = 0usize;
    while written + 1 < buf.len() {
        if state.x.have == 0 {
            if unsafe { gz_fetch(state, GzFetchOutput::StateOutput) } == -1 as ::core::ffi::c_int {
                break;
            }
            if state.x.have == 0 {
                state.past = 1 as ::core::ffi::c_int;
                break;
            }
        }

        let have = state.x.have as usize;
        let Some(start) = state.x.next.addr().checked_sub(state.out.as_ptr().addr()) else {
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"state corrupt"),
            );
            return false;
        };
        if start > state.out.len() || have > state.out.len() - start {
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"state corrupt"),
            );
            return false;
        }

        let available = (buf.len() - 1) - written;
        let copied = have.min(available);
        let source = &state.out[start..start + copied];
        let copied = source
            .iter()
            .position(|&byte| byte == b'\n')
            .map_or(copied, |newline| newline + 1);
        let hit_newline = source[copied - 1] == b'\n';
        buf[written..written + copied].copy_from_slice(&source[..copied]);
        state.x.have -= copied as ::core::ffi::c_uint;
        state.x.next = state.out.as_mut_ptr().wrapping_add(start + copied);
        state.x.pos += copied as crate::stdlib::off64_t;
        written += copied;

        if hit_newline {
            break;
        }
    }
    if written == 0 {
        return false;
    }
    buf[written] = 0;
    true
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    file: crate::zlib_h::gzFile,
    buf: *mut ::core::ffi::c_char,
    len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if file.is_null() || buf.is_null() || len < 1 {
        return ::core::ptr::null_mut();
    }
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return ::core::ptr::null_mut();
    };
    let output = ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len as usize);
    if gzgets_impl(state, output) {
        buf
    } else {
        ::core::ptr::null_mut()
    }
}
#[inline(never)]
fn gzdirect_impl(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::LOOK
        && state.x.have == 0 as ::core::ffi::c_uint
    {
        unsafe { gz_look(state) };
    }
    (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0;
    };
    gzdirect_impl(state)
}
pub unsafe fn gzclose_r(mut owned: Box<crate::gzguts_h::gz_state>) -> ::core::ffi::c_int {
    let ret = {
        let state: &mut crate::gzguts_h::gz_state = &mut owned;
        if state.mode != crate::gzguts_h::GZ_READ {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if state.size != 0 {
            crate::src::inflate::inflateEnd(&mut state.strm);
            state.in_0.clear();
        }
        let err = if state.err == crate::zlib_h::Z_BUF_ERROR {
            crate::zlib_h::Z_BUF_ERROR
        } else {
            crate::zlib_h::Z_OK
        };
        crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
        let close_failed = match state.fd.take() {
            // The state owns this descriptor, so dropping the owner closes it
            // exactly once without transferring it back into a raw handle.
            Some(fd) => {
                drop(fd);
                false
            }
            None => true,
        };
        if close_failed {
            crate::zlib_h::Z_ERRNO
        } else {
            err
        }
    };
    ret
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let owned = Box::from_raw(file.cast::<crate::gzguts_h::gz_state>());
    gzclose_r(owned)
}
