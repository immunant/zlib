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

fn compact_buffered_input(buffer: &mut [crate::stdlib::Bytef], source_start: usize, len: usize) {
    buffer.copy_within(source_start..source_start + len, 0);
}

fn is_gzip_header(input: &[u8]) -> bool {
    input.len() >= 4 && input[0] == 31 && input[1] == 139 && input[2] == 8 && input[3] < 32
}

fn copy_buffered_input(input: &[u8], output: &mut [u8]) {
    output[..input.len()].copy_from_slice(input);
}

fn copy_through_newline(input: &[u8], output: &mut [u8]) -> (usize, bool) {
    let copied = input
        .iter()
        .position(|&byte| byte == b'\n')
        .map_or(input.len(), |newline| newline + 1);
    output[..copied].copy_from_slice(&input[..copied]);
    (copied, copied != input.len())
}

fn pushback_empty(buffer: &mut [u8], byte: u8) -> Option<usize> {
    let next = buffer.len().checked_sub(1)?;
    buffer[next] = byte;
    Some(next)
}

fn pushback_buffer(buffer: &mut [u8], next: usize, have: usize, byte: u8) -> Option<usize> {
    if next > buffer.len() || have >= buffer.len() {
        return None;
    }
    let next = if next == 0 {
        let shifted = buffer.len().checked_sub(have)?;
        buffer.copy_within(0..have, shifted);
        shifted
    } else {
        next
    };
    let next = next.checked_sub(1)?;
    buffer[next] = byte;
    Some(next)
}

enum GzLoad {
    Loaded {
        have: ::core::ffi::c_uint,
        eof: bool,
        again: bool,
    },
    Error {
        have: ::core::ffi::c_uint,
        errno_value: ::core::ffi::c_int,
        again: bool,
    },
}

// Reading an owned gzip buffer does not require the ABI-shaped state.  Keep
// the I/O loop pointer-free and return every state transition for the caller
// to apply at its existing boundary.
fn gz_load(fd: &rustix::fd::OwnedFd, buf: &mut [u8]) -> GzLoad {
    let mut have: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    loop {
        // `have` is incremented only by bytes read into this same slice and
        // stops at its length, so it remains a valid suffix boundary.
        let output = &mut buf[have as usize..];
        let mut get = output.len() as ::core::ffi::c_uint;
        if get > max {
            get = max;
        }
        match rustix::io::read(fd, &mut output[..get as usize]) {
            Ok(0) => {
                return GzLoad::Loaded {
                    have,
                    eof: true,
                    again: false,
                };
            }
            Ok(read) => {
                have = have.wrapping_add(read as ::core::ffi::c_uint);
                if have as usize >= buf.len() {
                    return GzLoad::Loaded {
                        have,
                        eof: false,
                        again: false,
                    };
                }
            }
            Err(error) => {
                let errno_value = error.raw_os_error();
                let again = errno_value == crate::stdlib::EAGAIN
                    || errno_value == crate::stdlib::EWOULDBLOCK;
                if again && have != 0 {
                    return GzLoad::Loaded {
                        have,
                        eof: false,
                        again: true,
                    };
                }
                return GzLoad::Error {
                    have,
                    errno_value,
                    again,
                };
            }
        }
    }
}

unsafe fn apply_gz_load(
    state: &mut crate::gzguts_h::gz_state,
    result: GzLoad,
) -> Result<::core::ffi::c_uint, ::core::ffi::c_uint> {
    match result {
        GzLoad::Loaded { have, eof, again } => {
            state.again = again as ::core::ffi::c_int;
            if eof {
                state.eof = 1;
            }
            Ok(have)
        }
        GzLoad::Error {
            have,
            errno_value,
            again,
        } => {
            errno::set_errno(errno::Errno(errno_value));
            state.again = again as ::core::ffi::c_int;
            let message = errno::Errno(errno_value).to_string();
            crate::src::gzlib::gz_set_error(
                &mut state.msg,
                &mut state.err,
                &mut state.x.have,
                state.again,
                state.path.as_deref(),
                crate::zlib_h::Z_ERRNO,
                Some(message.as_bytes()),
            );
            Err(have)
        }
    }
}

unsafe fn gz_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut got: ::core::ffi::c_uint = 0;
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if state.eof == 0 as ::core::ffi::c_int {
        {
            let strm = &mut state.strm;
            if strm.avail_in != 0 {
                let buffer = state.in_0.as_deref_mut().unwrap();
                let p = buffer.as_mut_ptr();
                let q: *const ::core::ffi::c_uchar = strm.next_in;
                if q != p as *const ::core::ffi::c_uchar {
                    let n = strm.avail_in as usize;
                    let size = state.size as usize;
                    if p.is_null() || q.is_null() || n > size {
                        return -1 as ::core::ffi::c_int;
                    }
                    // `strm.next_in` is always a cursor in `in_0`: gz_load()
                    // installs the buffer, and inflate only advances that cursor.
                    let Some(source_start) = q.addr().checked_sub(p.addr()) else {
                        return -1 as ::core::ffi::c_int;
                    };
                    if source_start <= size && n <= size.wrapping_sub(source_start) {
                        compact_buffered_input(buffer, source_start, n);
                    } else {
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
        }
        let avail_in = state.strm.avail_in;
        let size = state.size as usize;
        let Some(mut buffer) = state.in_0.take() else {
            return -1 as ::core::ffi::c_int;
        };
        errno::set_errno(errno::Errno(0));
        let ret = if let Some(output) = buffer.get_mut(avail_in as usize..size) {
            match apply_gz_load(state, gz_load(state.fd.as_ref().unwrap(), output)) {
                Ok(have) => {
                    got = have;
                    0
                }
                Err(_) => -1,
            }
        } else {
            -1 as ::core::ffi::c_int
        };
        state.in_0 = Some(buffer);
        if ret == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        state.strm.avail_in = state.strm.avail_in.wrapping_add(got);
        state.strm.next_in = state.in_0.as_deref_mut().unwrap().as_mut_ptr();
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_look(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size == 0 as ::core::ffi::c_uint {
        state.in_0 = crate::src::gzlib::gz_buffer(state.want);
        state.out = crate::src::gzlib::gz_buffer(state.want << 1);
        if state.in_0.is_none() || state.out.is_none() {
            state.out = None;
            state.in_0 = None;
            crate::src::gzlib::gz_set_error(
                &mut state.msg,
                &mut state.err,
                &mut state.x.have,
                state.again,
                state.path.as_deref(),
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory"),
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
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            state.out = None;
            state.in_0 = None;
            state.size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_set_error(
                &mut state.msg,
                &mut state.err,
                &mut state.x.have,
                state.again,
                state.path.as_deref(),
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory"),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if state.direct == -1 as ::core::ffi::c_int || state.junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflateReset(&raw mut state.strm as *mut crate::zlib_h::z_stream_s);
        state.how = crate::gzguts_h::GZIP;
        state.junk = (state.junk != -1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        state.direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.strm.avail_in == 0 as crate::stdlib::uInt
        || state.again != 0 && state.strm.avail_in < 4 as crate::stdlib::uInt
    {
        return 0 as ::core::ffi::c_int;
    }
    let avail_in = state.strm.avail_in as usize;
    let next_in = state.strm.next_in;
    // `gz_avail()` installs `next_in` in this owned buffer, and inflate only
    // advances that cursor. Validate it before deriving the input view.
    let Some(input) = state.in_0.as_deref().and_then(|buffer| {
        let start = next_in.addr().checked_sub(buffer.as_ptr().addr())?;
        let end = start.checked_add(avail_in)?;
        buffer.get(start..end)
    }) else {
        return -1 as ::core::ffi::c_int;
    };
    if is_gzip_header(input) {
        crate::src::inflate::inflateReset(&raw mut state.strm as *mut crate::zlib_h::z_stream_s);
        state.how = crate::gzguts_h::GZIP;
        state.junk = 1 as ::core::ffi::c_int;
        state.direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    let Some(output) = state
        .out
        .as_deref_mut()
        .and_then(|buffer| buffer.get_mut(..avail_in))
    else {
        return -1 as ::core::ffi::c_int;
    };
    copy_buffered_input(input, output);
    state.x.next = output.as_mut_ptr();
    state.x.have = avail_in as ::core::ffi::c_uint;
    state.strm.avail_in = 0 as crate::stdlib::uInt;
    state.how = crate::gzguts_h::COPY;
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_decomp(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut had: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut state.strm;
    had = (*strm).avail_out as ::core::ffi::c_uint;
    loop {
        if (*strm).avail_in == 0 as crate::stdlib::uInt
            && gz_avail(state) == -1 as ::core::ffi::c_int
        {
            ret = state.err;
            break;
        } else if (*strm).avail_in == 0 as crate::stdlib::uInt {
            if state.again == 0 {
                crate::src::gzlib::gz_set_error(
                    &mut state.msg,
                    &mut state.err,
                    &mut state.x.have,
                    state.again,
                    state.path.as_deref(),
                    crate::zlib_h::Z_BUF_ERROR,
                    Some(b"unexpected end of file"),
                );
            }
            break;
        } else {
            ret = crate::src::inflate::inflate(
                strm as *mut crate::zlib_h::z_stream_s,
                crate::zlib_h::Z_NO_FLUSH,
            );
            if (*strm).avail_out < had {
                state.junk = 0 as ::core::ffi::c_int;
            }
            if ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT {
                crate::src::gzlib::gz_set_error(
                    &mut state.msg,
                    &mut state.err,
                    &mut state.x.have,
                    state.again,
                    state.path.as_deref(),
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(b"internal error: inflate stream corrupt"),
                );
                break;
            } else if ret == crate::zlib_h::Z_MEM_ERROR {
                crate::src::gzlib::gz_set_error(
                    &mut state.msg,
                    &mut state.err,
                    &mut state.x.have,
                    state.again,
                    state.path.as_deref(),
                    crate::zlib_h::Z_MEM_ERROR,
                    Some(b"out of memory"),
                );
                break;
            } else if ret == crate::zlib_h::Z_DATA_ERROR {
                if state.junk == 1 as ::core::ffi::c_int {
                    (*strm).avail_in = 0 as crate::stdlib::uInt;
                    state.eof = 1 as ::core::ffi::c_int;
                    state.how = crate::gzguts_h::LOOK;
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
    state.x.have =
        (had as crate::stdlib::uInt).wrapping_sub((*strm).avail_out) as ::core::ffi::c_uint;
    state.x.next =
        (*strm).next_out.wrapping_sub(state.x.have as usize) as *mut ::core::ffi::c_uchar;
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

unsafe fn gz_fetch(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut state.strm;
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
                let Some(mut output) = state.out.take() else {
                    return -1 as ::core::ffi::c_int;
                };
                errno::set_errno(errno::Errno(0));
                let (ret, have) = match apply_gz_load(
                    state,
                    gz_load(state.fd.as_ref().unwrap(), output.as_mut()),
                ) {
                    Ok(have) => (0, have),
                    Err(have) => (-1, have),
                };
                state.x.have = have;
                state.out = Some(output);
                if ret == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                state.x.next = state.out.as_deref_mut().unwrap().as_mut_ptr();
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                (*strm).avail_out = (state.size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                (*strm).next_out = state.out.as_deref_mut().unwrap().as_mut_ptr();
                if gz_decomp(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            _ => {
                crate::src::gzlib::gz_set_error(
                    &mut state.msg,
                    &mut state.err,
                    &mut state.x.have,
                    state.again,
                    state.path.as_deref(),
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(b"state corrupt"),
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if !(state.x.have == 0 as ::core::ffi::c_uint && (state.eof == 0 || (*strm).avail_in != 0))
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
            state.x.have = state.x.have.wrapping_sub(n);
            state.x.next = state.x.next.wrapping_add(n as usize);
            state.x.pos += n as crate::stdlib::off64_t;
            state.skip -= n as crate::stdlib::off64_t;
        } else {
            if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_fetch(state) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if state.skip == 0 {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_read(
    state: &mut crate::gzguts_h::gz_state,
    output: &mut [u8],
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut len = output.len() as crate::stdlib::z_size_t;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
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
            if state.x.have != 0 {
                if state.x.have < n {
                    n = state.x.have;
                }
                // `x.next` is a cursor in the owned output buffer whenever
                // `x.have` is nonzero here. Rebuild the buffered input with
                // a checked range so a corrupt cursor cannot extend a raw
                // slice beyond that allocation.
                let cursor = state.x.next;
                let Some(input) = state.out.as_deref().and_then(|buffer| {
                    let start = cursor.addr().checked_sub(buffer.as_ptr().addr())?;
                    let end = start.checked_add(n as usize)?;
                    buffer.get(start..end)
                }) else {
                    return got;
                };
                let Some(destination) = output.get_mut(got as usize..got as usize + n as usize)
                else {
                    return got;
                };
                copy_buffered_input(input, destination);
                state.x.next = state.x.next.wrapping_add(n as usize);
                state.x.have = state.x.have.wrapping_sub(n);
                if state.err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
            } else {
                if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                    break 's_140;
                }
                if state.how == crate::gzguts_h::LOOK || n < state.size << 1 as ::core::ffi::c_int {
                    if gz_fetch(state) == -1 as ::core::ffi::c_int
                        && state.x.have == 0 as ::core::ffi::c_uint
                    {
                        err = -1 as ::core::ffi::c_int;
                    }
                    break 's_28;
                } else if state.how == crate::gzguts_h::COPY {
                    let Some(destination) = output.get_mut(got as usize..got as usize + n as usize)
                    else {
                        return got;
                    };
                    errno::set_errno(errno::Errno(0));
                    match apply_gz_load(state, gz_load(state.fd.as_ref().unwrap(), destination)) {
                        Ok(have) => n = have,
                        Err(have) => {
                            n = have;
                            err = -1;
                        }
                    }
                } else {
                    let Some(destination) = output.get_mut(got as usize..got as usize + n as usize)
                    else {
                        return got;
                    };
                    state.strm.avail_out = n as crate::stdlib::uInt;
                    state.strm.next_out = destination.as_mut_ptr();
                    err = gz_decomp(state);
                    n = state.x.have;
                    state.x.have = 0 as ::core::ffi::c_uint;
                }
            }
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
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
unsafe fn gzread(
    state: &mut crate::gzguts_h::gz_state,
    output: &mut [u8],
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    if (output.len() as ::core::ffi::c_uint as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        crate::src::gzlib::gz_set_error(
            &mut state.msg,
            &mut state.err,
            &mut state.x.have,
            state.again,
            state.path.as_deref(),
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in an int"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let len = gz_read(state, output) as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if state.again != 0 {
            let errno_value = errno::errno().0;
            let message = errno::Errno(errno_value).to_string();
            crate::src::gzlib::gz_set_error(
                &mut state.msg,
                &mut state.err,
                &mut state.x.have,
                state.again,
                state.path.as_deref(),
                crate::zlib_h::Z_ERRNO,
                Some(message.as_bytes()),
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
    let output = if len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len as usize)
    };
    let Some(mut state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as ::core::ffi::c_int;
    };
    gzread(state.as_mut(), output)
}
unsafe fn gzfread(
    state: &mut crate::gzguts_h::gz_state,
    output: &mut [u8],
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut len: crate::stdlib::z_size_t = 0;
    if state.mode != crate::gzguts_h::GZ_READ {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
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
    }
    return if len != 0 {
        gz_read(state, output).wrapping_div(size)
    } else {
        0 as crate::stdlib::z_size_t
    };
}
#[export_name = "gzfread"]

pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let output = match size.checked_mul(nitems) {
        Some(0) | None => &mut [],
        Some(len) => ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len),
    };
    let Some(mut state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return 0 as crate::stdlib::z_size_t;
    };
    gzfread(state.as_mut(), output, size, nitems)
}
unsafe fn gzgetc(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    if state.x.have != 0 {
        // `x.next` is an ABI cursor into the owned output buffer here.  Do
        // not construct a raw slice from its advertised `have` length: a
        // corrupt cursor must not extend the view past that allocation.
        let cursor = state.x.next;
        let Some(byte) = state.out.as_deref().and_then(|buffer| {
            let offset = cursor.addr().checked_sub(buffer.as_ptr().addr())?;
            buffer.get(offset).copied()
        }) else {
            return -1 as ::core::ffi::c_int;
        };
        state.x.have = state.x.have.wrapping_sub(1);
        state.x.pos += 1;
        state.x.next = state.x.next.wrapping_add(1);
        return byte as ::core::ffi::c_int;
    }
    return if gz_read(state, &mut buf) < 1 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as usize] as ::core::ffi::c_int
    };
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzgetc(state)
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzgetc(state)
}
unsafe fn gzungetc(
    mut c: ::core::ffi::c_int,
    state: &mut crate::gzguts_h::gz_state,
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.how == crate::gzguts_h::LOOK && state.x.have == 0 as ::core::ffi::c_uint {
        gz_look(state);
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.x.have == 0 as ::core::ffi::c_uint {
        let size = state.size as usize;
        let Some(capacity) = size.checked_mul(2) else {
            return -1 as ::core::ffi::c_int;
        };
        let buffer = &mut state.out.as_deref_mut().unwrap()[..capacity];
        let Some(next) = pushback_empty(buffer, c as ::core::ffi::c_uchar) else {
            return -1 as ::core::ffi::c_int;
        };
        state.x.have = 1 as ::core::ffi::c_uint;
        state.x.next = buffer.as_mut_ptr().wrapping_add(next);
        state.x.pos -= 1;
        state.past = 0 as ::core::ffi::c_int;
        return c;
    }
    if state.x.have == state.size << 1 as ::core::ffi::c_int {
        crate::src::gzlib::gz_set_error(
            &mut state.msg,
            &mut state.err,
            &mut state.x.have,
            state.again,
            state.path.as_deref(),
            crate::zlib_h::Z_DATA_ERROR,
            Some(b"out of room to push characters"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let size = state.size as usize;
    let Some(capacity) = size.checked_mul(2) else {
        return -1 as ::core::ffi::c_int;
    };
    let buffer = &mut state.out.as_deref_mut().unwrap()[..capacity];
    let out = buffer.as_mut_ptr();
    let cursor = state.x.next;
    if cursor.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let Some(next) = cursor.addr().checked_sub(out.addr()) else {
        return -1 as ::core::ffi::c_int;
    };
    let Some(next) = pushback_buffer(
        buffer,
        next,
        state.x.have as usize,
        c as ::core::ffi::c_uchar,
    ) else {
        return -1 as ::core::ffi::c_int;
    };
    state.x.have = state.x.have.wrapping_add(1);
    state.x.next = out.wrapping_add(next);
    state.x.pos -= 1;
    state.past = 0 as ::core::ffi::c_int;
    return c;
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzungetc(c, state)
}
unsafe fn gzgets(
    mut state: Option<::core::ptr::NonNull<crate::gzguts_h::gz_state>>,
    output: &mut [u8],
) -> *mut ::core::ffi::c_char {
    if output.is_empty() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let Some(mut state) = state else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let state = state.as_mut();
    if state.mode != crate::gzguts_h::GZ_READ {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let str = output.as_mut_ptr().cast::<::core::ffi::c_char>();
    let mut left = output.len() - 1;
    let mut written = 0;
    if left != 0 {
        while !(state.x.have == 0 as ::core::ffi::c_uint
            && gz_fetch(state) == -1 as ::core::ffi::c_int)
        {
            if state.x.have == 0 as ::core::ffi::c_uint {
                state.past = 1 as ::core::ffi::c_int;
                break;
            } else {
                let mut n = if state.x.have as usize > left {
                    left
                } else {
                    state.x.have as usize
                };
                // `x.next` is a cursor in the owned output buffer whenever
                // `x.have` is nonzero.  Rebuild that view with a checked
                // range, so a corrupt cursor cannot extend a raw slice past
                // the allocation's remaining capacity.
                let cursor = state.x.next;
                let Some(input) = state.out.as_deref().and_then(|buffer| {
                    let start = cursor.addr().checked_sub(buffer.as_ptr().addr())?;
                    let end = start.checked_add(n)?;
                    buffer.get(start..end)
                }) else {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                };
                let (copied, found_newline) =
                    copy_through_newline(input, &mut output[written..written + n]);
                n = copied;
                state.x.have = state.x.have.wrapping_sub(n as ::core::ffi::c_uint);
                state.x.next = state.x.next.wrapping_add(n);
                state.x.pos += n as crate::stdlib::off64_t;
                left -= n;
                written += n;
                if !(left != 0 && !found_newline) {
                    break;
                }
            }
        }
    }
    if written == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    output[written] = 0;
    return str;
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if file.is_null() || buf.is_null() || len < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let output = ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len as usize);
    gzgets(
        ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep),
        output,
    )
}
unsafe fn gzdirect(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::LOOK
        && state.x.have == 0 as ::core::ffi::c_uint
    {
        gz_look(state);
    }
    return (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(mut state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return 0 as ::core::ffi::c_int;
    };
    gzdirect(state.as_mut())
}
pub unsafe extern "C" fn gzclose_r(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_ptr = file as crate::gzguts_h::gz_statep;
    if (*state_ptr).mode != crate::gzguts_h::GZ_READ {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *state_ptr;
    if state.size != 0 {
        crate::src::inflate::inflateEnd(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        state.out = None;
        state.in_0 = None;
    }
    err = if state.err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    state.path = None;
    state.msg = None;
    ret = match state.fd.take() {
        Some(fd) => unsafe {
            rustix::io::try_close(<rustix::fd::OwnedFd as rustix::fd::IntoRawFd>::into_raw_fd(
                fd,
            ))
        }
        .map(|()| 0 as ::core::ffi::c_int)
        .unwrap_or(-1 as ::core::ffi::c_int),
        None => -1 as ::core::ffi::c_int,
    };
    drop(Vec::from_raw_parts(state_ptr, 1, 1));
    return if ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    };
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_r(file)
}
