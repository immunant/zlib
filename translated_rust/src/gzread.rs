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

struct GzLoad {
    bytes: usize,
    eof: bool,
}

fn gz_load(file: &mut std::fs::File, buf: &mut [u8]) -> Result<GzLoad, (usize, std::io::Error)> {
    use std::io::Read;

    let max = ((-1i32 as u32 >> 2) + 1) as usize;
    let mut have = 0;
    while have < buf.len() {
        let get = (buf.len() - have).min(max);
        match file.read(&mut buf[have..have + get]) {
            Ok(0) => return Ok(GzLoad { bytes: have, eof: true }),
            Ok(read) => have += read,
            Err(error) => return Err((have, error)),
        }
    }
    Ok(GzLoad { bytes: have, eof: false })
}

fn gz_load_result(
    state: &mut crate::gzguts_h::gz_state,
    result: Result<GzLoad, (usize, std::io::Error)>,
) -> Result<usize, ()> {
    state.again = 0;
    match result {
        Ok(load) => {
            state.eof = load.eof as ::core::ffi::c_int;
            Ok(load.bytes)
        }
        Err((bytes, error)) => {
            state.again = (error.kind() == std::io::ErrorKind::WouldBlock) as ::core::ffi::c_int;
            if state.again != 0 && bytes != 0 {
                return Ok(bytes);
            }
            let message = std::ffi::CString::new(error.to_string()).ok();
            crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_ERRNO, message.as_deref());
            Err(())
        }
    }
}

fn read_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1;
    }
    if state.eof != 0 {
        return 0;
    }

    let available = state.strm.avail_in as usize;
    if available > state.in_end || state.in_end > state.in_buf.len() || available > state.size as usize {
        crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_STREAM_ERROR, Some(c"input buffer state corrupt"));
        return -1;
    }
    let start = state.in_end - available;
    if start != 0 {
        state.in_buf.copy_within(start..state.in_end, 0);
    }
    state.in_end = available;
    let capacity = state.size as usize;
    let result = {
        let (read_file, in_buf) = (&mut state.read_file, &mut state.in_buf);
        let file = read_file
            .as_mut()
            .expect("gzip read descriptor must be adopted at the FFI boundary");
        gz_load(file, &mut in_buf[available..capacity])
    };
    let got = match gz_load_result(state, result) {
        Ok(got) => got,
        Err(()) => return -1,
    };
    state.in_end += got;
    state.strm.avail_in = (available + got) as crate::stdlib::uInt;
    0
}

fn reset_read_inflater(state: &mut crate::gzguts_h::gz_state) {
    state.gzip_inflater = Some(flate2::Decompress::new_gzip(15));
}

fn read_look(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size == 0 {
        if !crate::src::gzlib::gz_init_buffers(state, state.want as usize, (state.want as usize) * 2) {
            crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_MEM_ERROR, Some(c"out of memory"));
            return -1;
        }
        state.size = state.want;
        reset_read_inflater(state);
    }
    if state.direct == -1 || state.junk == 0 {
        reset_read_inflater(state);
        state.how = crate::gzguts_h::GZIP;
        state.junk = (state.junk != -1) as ::core::ffi::c_int;
        state.direct = 0;
        return 0;
    }
    if read_avail(state) == -1 {
        return -1;
    }
    let available = state.strm.avail_in as usize;
    if available == 0 || (state.again != 0 && available < 4) {
        return 0;
    }
    let start = state.in_end - available;
    let input = &state.in_buf[start..state.in_end];
    if input.len() > 3 && input[0] == 31 && input[1] == 139 && input[2] == 8 && input[3] < 32 {
        reset_read_inflater(state);
        state.how = crate::gzguts_h::GZIP;
        state.junk = 1;
        state.direct = 0;
        return 0;
    }
    state.out_buf[..available].copy_from_slice(input);
    state.x.next = 0;
    state.x.have = available as ::core::ffi::c_uint;
    state.strm.avail_in = 0;
    state.how = crate::gzguts_h::COPY;
    0
}

fn read_decomp(state: &mut crate::gzguts_h::gz_state, output: &mut [u8]) -> ::core::ffi::c_int {
    use flate2::{FlushDecompress, Status};

    let mut written = 0;
    let mut result = crate::zlib_h::Z_OK;
    loop {
        if state.strm.avail_in == 0 && read_avail(state) == -1 {
            result = state.err;
            break;
        }
        if state.strm.avail_in == 0 {
            if state.again == 0 {
                crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_BUF_ERROR, Some(c"unexpected end of file"));
            }
            break;
        }
        let available = state.strm.avail_in as usize;
        let start = state.in_end - available;
        let inflater = state.gzip_inflater.as_mut().expect("gzip inflater must be initialized before decompression");
        let before_in = inflater.total_in();
        let before_out = inflater.total_out();
        let status = inflater.decompress(&state.in_buf[start..state.in_end], &mut output[written..], FlushDecompress::None);
        let consumed = (inflater.total_in() - before_in) as usize;
        let produced = (inflater.total_out() - before_out) as usize;
        state.strm.avail_in = state.strm.avail_in.saturating_sub(consumed as crate::stdlib::uInt);
        state.strm.total_in = state.strm.total_in.wrapping_add(consumed as crate::stdlib::uLong);
        state.strm.total_out = state.strm.total_out.wrapping_add(produced as crate::stdlib::uLong);
        written += produced;
        match status {
            Ok(Status::StreamEnd) => {
                state.junk = 0;
                state.how = crate::gzguts_h::LOOK;
                break;
            }
            Ok(Status::Ok) if written == output.len() => break,
            Ok(Status::Ok) if consumed == 0 && produced == 0 => break,
            Ok(Status::BufError) => break,
            Ok(_) => {}
            Err(_) => {
                crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_DATA_ERROR, Some(c"compressed data error"));
                result = crate::zlib_h::Z_DATA_ERROR;
                break;
            }
        }
    }
    state.x.have = written as ::core::ffi::c_uint;
    state.x.next = 0;
    if result == crate::zlib_h::Z_OK { 0 } else { -1 }
}

fn read_fetch(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    loop {
        match state.how {
            crate::gzguts_h::LOOK => {
                if read_look(state) == -1 || state.how == crate::gzguts_h::LOOK {
                    return if state.err == crate::zlib_h::Z_OK || state.err == crate::zlib_h::Z_BUF_ERROR { 0 } else { -1 };
                }
            }
            crate::gzguts_h::COPY => {
                let output_len = state.size as usize * 2;
                let result = {
                    let (read_file, out_buf) = (&mut state.read_file, &mut state.out_buf);
                    let file = read_file.as_mut().expect("gzip read descriptor must be adopted at the FFI boundary");
                    gz_load(file, &mut out_buf[..output_len])
                };
                state.x.have = match gz_load_result(state, result) {
                    Ok(got) => got as ::core::ffi::c_uint,
                    Err(()) => return -1,
                };
                state.x.next = 0;
                return 0;
            }
            crate::gzguts_h::GZIP => {
                let mut out_buf = std::mem::take(&mut state.out_buf);
                let result = read_decomp(state, &mut out_buf);
                state.out_buf = out_buf;
                if result == -1 {
                    return -1;
                }
            }
            _ => {
                crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_STREAM_ERROR, Some(c"state corrupt"));
                return -1;
            }
        }
        if state.x.have != 0 || (state.eof != 0 && state.strm.avail_in == 0) {
            return 0;
        }
    }
}

fn read_skip(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    while state.skip != 0 {
        if state.x.have != 0 {
            let n = (state.x.have as crate::stdlib::off64_t).min(state.skip) as ::core::ffi::c_uint;
            let end = match state.x.next.checked_add(n as usize) {
                Some(end) if end <= state.out_buf.len() => end,
                _ => {
                    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_STREAM_ERROR, Some(c"output buffer state corrupt"));
                    return -1;
                }
            };
            state.x.have -= n;
            state.x.next = end;
            state.x.pos += n as crate::stdlib::off64_t;
            state.skip -= n as crate::stdlib::off64_t;
        } else {
            if state.eof != 0 && state.strm.avail_in == 0 {
                break;
            }
            if read_fetch(state) == -1 {
                return -1;
            }
        }
    }
    0
}

fn read_gz_read(state: &mut crate::gzguts_h::gz_state, buf: &mut [u8]) -> usize {
    if buf.is_empty() {
        return 0;
    }
    if state.skip != 0 && read_skip(state) == -1 {
        return 0;
    }
    let mut got = 0;
    let mut error = 0;
    while got < buf.len() && error == 0 {
        let mut n = (buf.len() - got).min(::core::ffi::c_uint::MAX as usize);
        if state.x.have != 0 {
            n = n.min(state.x.have as usize);
            let end = match state.x.next.checked_add(n) {
                Some(end) if end <= state.out_buf.len() => end,
                _ => {
                    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_STREAM_ERROR, Some(c"output buffer state corrupt"));
                    break;
                }
            };
            buf[got..got + n].copy_from_slice(&state.out_buf[state.x.next..end]);
            state.x.next = end;
            state.x.have -= n as ::core::ffi::c_uint;
            if state.err != crate::zlib_h::Z_OK {
                error = -1;
            }
        } else {
            if state.eof != 0 && state.strm.avail_in == 0 {
                break;
            }
            if state.how == crate::gzguts_h::LOOK || n < state.size as usize * 2 {
                if read_fetch(state) == -1 && state.x.have == 0 {
                    error = -1;
                }
                continue;
            }
            if state.how == crate::gzguts_h::COPY {
                if read_fetch(state) == -1 {
                    error = -1;
                }
                continue;
            }
            error = read_decomp(state, &mut buf[got..got + n]);
            n = state.x.have as usize;
            state.x.have = 0;
        }
        got += n;
        state.x.pos += n as crate::stdlib::off64_t;
    }
    if got < buf.len() && state.eof != 0 {
        state.past = 1;
    }
    got
}

unsafe extern "C" fn gz_avail(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let state = &mut *state;
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if state.eof == 0 {
        let available = state.strm.avail_in as usize;
        if available > state.in_end || state.in_end > state.in_buf.len() || available > state.size as usize {
            crate::src::gzlib::gz_error_safe(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"input buffer state corrupt"),
            );
            return -1;
        }
        let start = state.in_end - available;
        if start != 0 {
            state.in_buf.copy_within(start..state.in_end, 0);
        }
        state.in_end = available;
        let capacity = state.size as usize;
        let result = {
            let (read_file, in_buf) = (&mut state.read_file, &mut state.in_buf);
            let file = read_file
                .as_mut()
                .expect("gzip read descriptor must be adopted at the FFI boundary");
            gz_load(file, &mut in_buf[available..capacity])
        };
        let got = match gz_load_result(state, result) {
            Ok(got) => got,
            Err(()) => return -1,
        };
        state.in_end += got;
        state.strm.avail_in = (available + got) as crate::stdlib::uInt;
        state.strm.next_in = crate::input_cursor!(state.in_buf.as_mut_ptr());
    }
    0
}

unsafe extern "C" fn gz_look(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint {
        if !crate::src::gzlib::gz_init_buffers(
            &mut *state,
            (*state).want as usize,
            ((*state).want << 1 as ::core::ffi::c_int) as usize,
        ) {
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
        (*state).strm.opaque = crate::zlib_h::Opaque::default();
        (*state).strm.avail_in = 0 as crate::stdlib::uInt;
        (*state).strm.next_in = crate::zlib_h::InputBuffer::default();
        if crate::src::inflate::inflateInit2_(
            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            (*state).in_buf.clear();
            (*state).out_buf.clear();
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
        && *crate::input_pointer!((*strm).next_in).offset(0 as isize) as ::core::ffi::c_int == 31 as ::core::ffi::c_int
        && *crate::input_pointer!((*strm).next_in).offset(1 as isize) as ::core::ffi::c_int == 139 as ::core::ffi::c_int
        && *crate::input_pointer!((*strm).next_in).offset(2 as isize) as ::core::ffi::c_int == 8 as ::core::ffi::c_int
        && (*crate::input_pointer!((*strm).next_in).offset(3 as isize) as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
    {
        crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
        (*state).how = crate::gzguts_h::GZIP;
        (*state).junk = 1 as ::core::ffi::c_int;
        (*state).direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    (*state).x.next = 0;
    crate::stdlib::memcpy(
        (*state).out_buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        crate::input_pointer!((*strm).next_in) as *const ::core::ffi::c_void,
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
                    crate::src::gzlib::gz_error_safe(
                        &mut *state,
                        crate::zlib_h::Z_DATA_ERROR,
                        (*strm)
                            .msg
                            .as_ref()
                            .map(|message| message.as_c_str())
                            .or(Some(c"compressed data error")),
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
    // Buffered inflate output always begins at the start of `out_buf`.  When
    // inflating directly into the caller's buffer, `x.have` is cleared before
    // this cursor is observed.
    (*state).x.next = 0;
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
                let state = &mut *state;
                let output_len = state.size as usize * 2;
                let result = {
                    let (read_file, out_buf) = (&mut state.read_file, &mut state.out_buf);
                    let file = read_file
                        .as_mut()
                        .expect("gzip read descriptor must be adopted at the FFI boundary");
                    gz_load(file, &mut out_buf[..output_len])
                };
                state.x.have = match gz_load_result(state, result) {
                    Ok(got) => got as ::core::ffi::c_uint,
                    Err(()) => return -1,
                };
                state.x.next = 0;
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                (*strm).avail_out =
                    ((*state).size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                (*strm).next_out = crate::output_cursor!((*state).out_buf.as_mut_ptr());
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

unsafe extern "C" fn gz_skip(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_uint = 0;
    loop {
        if (*state).x.have != 0 {
            let state = &mut *state;
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
            state.x.next += n as usize;
            state.x.pos += n as crate::stdlib::off64_t;
            state.skip -= n as crate::stdlib::off64_t;
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
                if (*state).x.have < n {
                    n = (*state).x.have;
                }
                let start = (*state).x.next;
                let end = start + n as usize;
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    (&(*state).out_buf)[start..end].as_ptr() as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                (*state).x.next = end;
                (*state).x.have = (*state).x.have.wrapping_sub(n);
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
                    if gz_fetch(state) == -1 as ::core::ffi::c_int {
                        err = -1;
                    }
                    break 's_28;
                } else {
                    (*state).strm.avail_out = n as crate::stdlib::uInt;
                    (*state).strm.next_out = crate::output_cursor!(buf);
                    err = gz_decomp(state);
                    n = (*state).x.have;
                    (*state).x.have = 0 as ::core::ffi::c_uint;
                }
            }
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            buf = (buf as *mut ::core::ffi::c_char).offset(n as isize) as crate::stdlib::voidp;
            got = got.wrapping_add(n as crate::stdlib::z_size_t);
            (*state).x.pos += n as crate::stdlib::off64_t;
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
pub fn gzread(state: &mut crate::gzguts_h::gz_state, buf: &mut [u8]) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
    if buf.len() > crate::limits_h::INT_MAX as usize {
        crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_STREAM_ERROR, Some(c"request does not fit in an int"));
        return -1 as ::core::ffi::c_int;
    }
    let len = read_gz_read(state, buf);
    if len == 0 {
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if state.again != 0 {
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
    if file.is_null() || (buf.is_null() && len != 0) {
        return -1;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    let output = if len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(buf as *mut u8, len as usize)
    };
    gzread(state, output)
}
pub fn gzfread(
    state: &mut crate::gzguts_h::gz_state,
    buf: &mut [u8],
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
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
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
    let Some(len) = nitems.checked_mul(size) else {
        crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_STREAM_ERROR, Some(c"request does not fit in a size_t"));
        return 0 as crate::stdlib::z_size_t;
    };
    if len == 0 {
        return 0;
    }
    if len > buf.len() {
        crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_STREAM_ERROR, Some(c"output buffer too small"));
        return 0;
    }
    read_gz_read(state, &mut buf[..len]) / size
}
#[export_name = "gzfread"]

pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    if file.is_null() {
        return 0;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    let Some(len) = size.checked_mul(nitems) else {
        crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_STREAM_ERROR, Some(c"request does not fit in a size_t"));
        return 0;
    };
    if len != 0 && buf.is_null() {
        return 0;
    }
    let output = if len == 0 { &mut [] } else { ::core::slice::from_raw_parts_mut(buf as *mut u8, len) };
    gzfread(state, output, size, nitems)
}
pub fn gzgetc(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
    if state.x.have != 0 {
        if state.x.next >= state.out_buf.len() {
            crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_STREAM_ERROR, Some(c"output buffer state corrupt"));
            return -1;
        }
        state.x.have = state.x.have.wrapping_sub(1);
        state.x.pos += 1;
        let next = state.x.next;
        state.x.next += 1;
        return state.out_buf[next] as ::core::ffi::c_int;
    }
    let mut buf = [0u8; 1];
    if read_gz_read(state, &mut buf) < 1 { -1 } else { buf[0] as ::core::ffi::c_int }
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() { -1 } else { gzgetc(&mut *(file as crate::gzguts_h::gz_statep)) }
}
pub fn gzgetc_(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    gzgetc(state)
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() { -1 } else { gzgetc_(&mut *(file as crate::gzguts_h::gz_statep)) }
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
    if (*state).err != crate::zlib_h::Z_OK
        && (*state).err != crate::zlib_h::Z_BUF_ERROR
        && (*state).again == 0
    {
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
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).x.have == 0 as ::core::ffi::c_uint {
        (*state).x.have = 1 as ::core::ffi::c_uint;
        (*state).x.next = (*state).out_buf.len() - 1;
        (&mut (*state).out_buf)[(*state).x.next] = c as ::core::ffi::c_uchar;
        (*state).x.pos -= 1;
        (*state).past = 0 as ::core::ffi::c_int;
        return c;
    }
    if (*state).x.have == (*state).size << 1 as ::core::ffi::c_int {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_DATA_ERROR,
            b"out of room to push characters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*state).x.next == 0 {
        let have = (*state).x.have as usize;
        let next = (*state).out_buf.len() - have;
        (*state).out_buf.copy_within(0..have, next);
        (*state).x.next = next;
    }
    (*state).x.have = (*state).x.have.wrapping_add(1);
    (*state).x.next -= 1;
    (&mut (*state).out_buf)[(*state).x.next] = c as ::core::ffi::c_uchar;
    (*state).x.pos -= 1;
    (*state).past = 0 as ::core::ffi::c_int;
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
    let mut found_eol = false;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() || buf.is_null() || len < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*state).err != crate::zlib_h::Z_OK
        && (*state).err != crate::zlib_h::Z_BUF_ERROR
        && (*state).again == 0
    {
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
                n = if (*state).x.have > left {
                    left
                } else {
                    (*state).x.have
                };
                let start = (*state).x.next;
                let end = start + n as usize;
                let buffered = &(&(*state).out_buf)[start..end];
                found_eol = if let Some(offset) = buffered.iter().position(|&byte| byte == b'\n') {
                    n = offset as ::core::ffi::c_uint + 1;
                    true
                } else {
                    false
                };
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    buffered.as_ptr() as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                (*state).x.have = (*state).x.have.wrapping_sub(n);
                (*state).x.next += n as usize;
                (*state).x.pos += n as crate::stdlib::off64_t;
                left = left.wrapping_sub(n);
                buf = buf.offset(n as isize);
                if !(left != 0 && !found_eol) {
                    break;
                }
            }
        }
    }
    if buf == str {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *buf.offset(0 as isize) = 0 as ::core::ffi::c_char;
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
pub unsafe extern "C" fn gzdirect(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode == crate::gzguts_h::GZ_READ
        && (*state).how == crate::gzguts_h::LOOK
        && (*state).x.have == 0 as ::core::ffi::c_uint
    {
        gz_look(state);
    }
    return ((*state).direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzdirect(file)
}
pub struct GzCloseRead {
    pub result: ::core::ffi::c_int,
    pub valid: bool,
    pub end_inflater: bool,
}

pub fn gzclose_r(state: &mut crate::gzguts_h::gz_state) -> GzCloseRead {
    if state.mode != crate::gzguts_h::GZ_READ {
        return GzCloseRead {
            result: crate::zlib_h::Z_STREAM_ERROR,
            valid: false,
            end_inflater: false,
        };
    }
    let result = if state.err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    GzCloseRead {
        result,
        valid: true,
        end_inflater: state.size != 0,
    }
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    let close = gzclose_r(state);
    if !close.valid {
        return close.result;
    }
    if close.end_inflater {
        crate::src::inflate::inflateEnd(&mut state.strm);
    }
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
    let read_file = state.read_file.take();
    drop(Box::from_raw(state));
    drop(read_file);
    close.result
}
