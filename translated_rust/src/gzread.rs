pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZIP;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::LOOK;
pub use crate::src::gzlib::gz_consume_buffered_read_cursor;
pub use crate::src::gzlib::gz_errno_is_retryable;
pub use crate::src::gzlib::gz_error;
pub(crate) use crate::src::gzlib::gz_file_completed_items;
pub(crate) use crate::src::gzlib::gz_file_request_len;
pub use crate::src::gzlib::gz_io_chunk_len;
pub use crate::src::gzlib::gz_uInt_fits_int;
pub use crate::src::gzlib::gz_z_size_to_uInt_chunk;

pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflateEnd_ffi;
pub use crate::src::inflate::inflateInit2__ffi;
pub use crate::src::inflate::inflateReset_ffi;
pub use crate::src::inflate::inflate_ffi;

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

enum GzLoadReadResult {
    Ok,
    RetryAfterPartial,
    Error { retryable: bool },
    Eof,
}

fn gz_load_read_result(
    ret: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
    errno: ::core::ffi::c_int,
) -> GzLoadReadResult {
    if ret < 0 as ::core::ffi::c_int {
        let retryable = gz_errno_is_retryable(errno);
        if retryable && have != 0 as ::core::ffi::c_uint {
            GzLoadReadResult::RetryAfterPartial
        } else {
            GzLoadReadResult::Error { retryable }
        }
    } else if ret == 0 as ::core::ffi::c_int {
        GzLoadReadResult::Eof
    } else {
        GzLoadReadResult::Ok
    }
}

unsafe fn gz_load(
    state: &mut crate::gzguts_h::gz_state,
    mut buf: *mut ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
    have: &mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut get: ::core::ffi::c_uint = 0;
    state.again = 0 as ::core::ffi::c_int;
    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
    *have = 0 as ::core::ffi::c_uint;
    loop {
        get = gz_io_chunk_len(len.wrapping_sub(*have));
        ret = crate::stdlib::read(
            state.fd,
            buf.wrapping_add(*have as usize) as *mut ::core::ffi::c_void,
            get as crate::__stddef_size_t_h::size_t,
        ) as ::core::ffi::c_int;
        if ret <= 0 as ::core::ffi::c_int {
            break;
        }
        *have = (*have).wrapping_add(ret as ::core::ffi::c_uint);
        if !(*have < len) {
            break;
        }
    }
    let errno = if ret < 0 as ::core::ffi::c_int {
        *crate::stdlib::__errno_location()
    } else {
        0 as ::core::ffi::c_int
    };
    match gz_load_read_result(ret, *have, errno) {
        GzLoadReadResult::Ok => {}
        GzLoadReadResult::RetryAfterPartial => {
            state.again = 1 as ::core::ffi::c_int;
            return 0 as ::core::ffi::c_int;
        }
        GzLoadReadResult::Error { retryable } => {
            if retryable {
                state.again = 1 as ::core::ffi::c_int;
            }
            crate::src::gzlib::gz_error(
                &mut *state,
                crate::zlib_h::Z_ERRNO,
                crate::stdlib::strerror(errno),
            );
            return -1 as ::core::ffi::c_int;
        }
        GzLoadReadResult::Eof => {
            state.eof = 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_buffered_input_next_offset(
    next_addr: usize,
    base_addr: usize,
    avail_in: usize,
) -> Option<usize> {
    if avail_in == 0 {
        return None;
    }
    let next_offset = next_addr.wrapping_sub(base_addr);
    if next_offset == 0 {
        None
    } else {
        Some(next_offset)
    }
}

fn gz_compact_input_buffer(buf: &mut [crate::stdlib::Bytef], next_offset: usize, avail_in: usize) {
    if next_offset != 0 && avail_in != 0 {
        buf.copy_within(next_offset..next_offset + avail_in, 0);
    }
}

unsafe fn gz_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut got: ::core::ffi::c_uint = 0;
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if state.eof == 0 as ::core::ffi::c_int {
        if let Some(next_offset) = gz_buffered_input_next_offset(
            state.strm.next_in as usize,
            state.in_0 as usize,
            state.strm.avail_in as usize,
        ) {
            let input = ::core::slice::from_raw_parts_mut(
                state.in_0 as *mut crate::stdlib::Bytef,
                state.size as usize,
            );
            gz_compact_input_buffer(input, next_offset, state.strm.avail_in as usize);
        }
        let load_buf = state.in_0.wrapping_add(state.strm.avail_in as usize);
        let load_len = state
            .size
            .wrapping_sub(state.strm.avail_in as ::core::ffi::c_uint);
        if gz_load(state, load_buf, load_len, &mut got) == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        state.strm.avail_in = state.strm.avail_in.wrapping_add(got);
        state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_set_gzip_mode(state: &mut crate::gzguts_h::gz_state, junk: ::core::ffi::c_int) {
    state.how = crate::gzguts_h::GZIP;
    state.junk = junk;
    state.direct = 0 as ::core::ffi::c_int;
}

fn gz_is_gzip_header(header: [crate::stdlib::Bytef; 4]) -> bool {
    header[0] as ::core::ffi::c_int == 31 as ::core::ffi::c_int
        && header[1] as ::core::ffi::c_int == 139 as ::core::ffi::c_int
        && header[2] as ::core::ffi::c_int == 8 as ::core::ffi::c_int
        && (header[3] as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
}

fn gz_prepare_fetch_output(state: &mut crate::gzguts_h::gz_state) {
    state.strm.avail_out = (state.size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
    state.strm.next_out = state.out as *mut crate::stdlib::Bytef;
}

fn gz_look_needs_more_header_input(
    avail_in: crate::stdlib::uInt,
    again: ::core::ffi::c_int,
) -> bool {
    avail_in == 0 as crate::stdlib::uInt || again != 0 && avail_in < 4 as crate::stdlib::uInt
}

unsafe fn gz_look(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size == 0 as ::core::ffi::c_uint {
        state.in_0 = crate::stdlib::malloc(state.want as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar;
        state.out = crate::stdlib::malloc(
            (state.want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
        ) as *mut ::core::ffi::c_uchar;
        if state.in_0.is_null() || state.out.is_null() {
            crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                &mut *state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        state.size = state.want;
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        state.strm.avail_in = 0 as crate::stdlib::uInt;
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit2__ffi(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            state.size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_error(
                &mut *state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if state.direct == -1 as ::core::ffi::c_int || state.junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflateReset_ffi(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        let junk = (state.junk != -1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        gz_set_gzip_mode(state, junk);
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if gz_look_needs_more_header_input(state.strm.avail_in, state.again) {
        return 0 as ::core::ffi::c_int;
    }
    if state.strm.avail_in > 3 as crate::stdlib::uInt {
        let header = [
            *state.strm.next_in,
            *state.strm.next_in.wrapping_add(1),
            *state.strm.next_in.wrapping_add(2),
            *state.strm.next_in.wrapping_add(3),
        ];
        if gz_is_gzip_header(header) {
            crate::src::inflate::inflateReset_ffi(
                &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            );
            gz_set_gzip_mode(state, 1 as ::core::ffi::c_int);
            return 0 as ::core::ffi::c_int;
        }
    }
    state.x.next = state.out;
    crate::stdlib::memcpy(
        state.x.next as *mut ::core::ffi::c_void,
        state.strm.next_in as *const ::core::ffi::c_void,
        state.strm.avail_in as crate::__stddef_size_t_h::size_t,
    );
    state.x.have = state.strm.avail_in as ::core::ffi::c_uint;
    state.strm.avail_in = 0 as crate::stdlib::uInt;
    state.how = crate::gzguts_h::COPY;
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_decomp(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let had: ::core::ffi::c_uint = state.strm.avail_out as ::core::ffi::c_uint;
    loop {
        if state.strm.avail_in == 0 as crate::stdlib::uInt
            && gz_avail(state) == -1 as ::core::ffi::c_int
        {
            ret = state.err;
            break;
        } else if state.strm.avail_in == 0 as crate::stdlib::uInt {
            if state.again == 0 {
                crate::src::gzlib::gz_error(
                    &mut *state,
                    crate::zlib_h::Z_BUF_ERROR,
                    b"unexpected end of file\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            break;
        } else {
            ret = crate::src::inflate::inflate_ffi(
                &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                crate::zlib_h::Z_NO_FLUSH,
            );
            if state.strm.avail_out < had {
                state.junk = 0 as ::core::ffi::c_int;
            }
            if gz_inflate_stream_corrupt(ret) {
                crate::src::gzlib::gz_error(
                    &mut *state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                break;
            } else if ret == crate::zlib_h::Z_MEM_ERROR {
                crate::src::gzlib::gz_error(
                    &mut *state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break;
            } else if ret == crate::zlib_h::Z_DATA_ERROR {
                match gz_decomp_data_error_action(state.junk, state.strm.msg.is_null()) {
                    GzDecompDataErrorAction::TreatTrailingJunkAsEof => {
                        state.strm.avail_in = 0 as crate::stdlib::uInt;
                        state.eof = 1 as ::core::ffi::c_int;
                        state.how = crate::gzguts_h::LOOK;
                        ret = crate::zlib_h::Z_OK;
                        break;
                    }
                    GzDecompDataErrorAction::Report { use_default_msg } => {
                        let msg = if use_default_msg {
                            b"compressed data error\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            state.strm.msg as *const ::core::ffi::c_char
                        };
                        crate::src::gzlib::gz_error(&mut *state, crate::zlib_h::Z_DATA_ERROR, msg);
                        break;
                    }
                }
            } else if !gz_decomp_should_continue(state.strm.avail_out, ret) {
                break;
            }
        }
    }
    state.x.have = gz_decompressed_output_have(had, state.strm.avail_out);
    state.x.next = state.strm.next_out.wrapping_sub(state.x.have as usize);
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
                let mut loaded = 0 as ::core::ffi::c_uint;
                let out = state.out;
                let len = state.size << 1 as ::core::ffi::c_int;
                let ret = gz_load(state, out, len, &mut loaded);
                state.x.have = loaded;
                if ret == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                state.x.next = state.out;
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                gz_prepare_fetch_output(state);
                if gz_decomp(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            _ => {
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"state corrupt\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if !gz_fetch_needs_more_output(state.x.have, state.eof, state.strm.avail_in) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_skip(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    loop {
        if state.x.have != 0 {
            gz_consume_skip_buffer(state);
        } else {
            if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_fetch(state) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if !(state.skip != 0) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

enum GzReadStep {
    ProducedOutput,
    NeedMoreInput,
}

unsafe fn gz_read(
    state: &mut crate::gzguts_h::gz_state,
    mut buf: crate::stdlib::voidp,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    loop {
        let step: GzReadStep;
        n = gz_z_size_to_uInt_chunk(len);
        if state.x.have != 0 {
            n = gz_read_buffered_copy_len(len, state.x.have);
            crate::stdlib::memcpy(
                buf as *mut ::core::ffi::c_void,
                state.x.next as *const ::core::ffi::c_void,
                n as crate::__stddef_size_t_h::size_t,
            );
            gz_advance_buffered_read_cursor(state, n);
            if state.err != crate::zlib_h::Z_OK {
                err = -1 as ::core::ffi::c_int;
            }
            step = GzReadStep::ProducedOutput;
        } else {
            if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_read_should_fetch(state.how, n, state.size) {
                if gz_fetch(state) == -1 as ::core::ffi::c_int
                    && state.x.have == 0 as ::core::ffi::c_uint
                {
                    err = -1 as ::core::ffi::c_int;
                }
                step = GzReadStep::NeedMoreInput;
            } else {
                if state.how == crate::gzguts_h::COPY {
                    err = gz_load(state, buf as *mut ::core::ffi::c_uchar, n, &mut n);
                } else {
                    state.strm.avail_out = n as crate::stdlib::uInt;
                    state.strm.next_out =
                        buf as *mut ::core::ffi::c_uchar as *mut crate::stdlib::Bytef;
                    err = gz_decomp(state);
                    n = state.x.have;
                    state.x.have = 0 as ::core::ffi::c_uint;
                }
                step = GzReadStep::ProducedOutput;
            }
        }
        match step {
            GzReadStep::ProducedOutput => {
                len = len.wrapping_sub(n as crate::stdlib::z_size_t);
                buf = (buf as *mut ::core::ffi::c_char).wrapping_add(n as usize)
                    as crate::stdlib::voidp;
                got = got.wrapping_add(n as crate::stdlib::z_size_t);
                state.x.pos += n as crate::stdlib::off64_t;
            }
            GzReadStep::NeedMoreInput => {}
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

fn gzread_len_fits_int(len: ::core::ffi::c_uint) -> bool {
    gz_uInt_fits_int(len)
}

fn gz_read_should_fetch(
    how: ::core::ffi::c_int,
    n: crate::stdlib::uInt,
    size: crate::stdlib::uInt,
) -> bool {
    how == crate::gzguts_h::LOOK || n < size << 1 as ::core::ffi::c_int
}

fn gz_read_state_ready(state: &crate::gzguts_h::gz_state) -> bool {
    state.mode == crate::gzguts_h::GZ_READ
        && (state.err == crate::zlib_h::Z_OK
            || state.err == crate::zlib_h::Z_BUF_ERROR
            || state.again != 0)
}

fn gzread_zero_needs_errno(err: ::core::ffi::c_int, again: ::core::ffi::c_int) -> bool {
    again != 0 && (err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR)
}

enum GzReadZeroResult {
    Ok,
    Error,
    Errno,
}

fn gzread_zero_result(err: ::core::ffi::c_int, again: ::core::ffi::c_int) -> GzReadZeroResult {
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR {
        GzReadZeroResult::Error
    } else if gzread_zero_needs_errno(err, again) {
        GzReadZeroResult::Errno
    } else {
        GzReadZeroResult::Ok
    }
}

fn gz_decomp_should_continue(avail_out: crate::stdlib::uInt, ret: ::core::ffi::c_int) -> bool {
    avail_out != 0 && ret != crate::zlib_h::Z_STREAM_END
}

fn gz_inflate_stream_corrupt(ret: ::core::ffi::c_int) -> bool {
    ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT
}

enum GzDecompDataErrorAction {
    TreatTrailingJunkAsEof,
    Report { use_default_msg: bool },
}

fn gz_decomp_data_error_action(
    junk: ::core::ffi::c_int,
    msg_is_null: bool,
) -> GzDecompDataErrorAction {
    if junk == 1 as ::core::ffi::c_int {
        GzDecompDataErrorAction::TreatTrailingJunkAsEof
    } else {
        GzDecompDataErrorAction::Report {
            use_default_msg: msg_is_null,
        }
    }
}

fn gz_fetch_needs_more_output(
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
) -> bool {
    have == 0 as ::core::ffi::c_uint && (eof == 0 || avail_in != 0)
}

fn gz_decompressed_output_have(
    had: ::core::ffi::c_uint,
    avail_out: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    had.wrapping_sub(avail_out) as ::core::ffi::c_uint
}

fn gz_consume_skip_buffer(state: &mut crate::gzguts_h::gz_state) {
    let mut skip = state.skip;
    gz_consume_buffered_read_cursor(state, &mut skip);
    state.skip = skip;
}

fn gz_advance_buffered_read_cursor(
    state: &mut crate::gzguts_h::gz_state,
    count: ::core::ffi::c_uint,
) {
    state.x.have = state.x.have.wrapping_sub(count);
    state.x.next = state.x.next.wrapping_add(count as usize);
}

fn gz_read_buffered_copy_len(
    len: crate::stdlib::z_size_t,
    have: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    let n = gz_z_size_to_uInt_chunk(len);
    if have < n {
        have
    } else {
        n
    }
}

fn gz_note_buffered_read(state: &mut crate::gzguts_h::gz_state, count: ::core::ffi::c_uint) {
    gz_advance_buffered_read_cursor(state, count);
    state.x.pos += count as crate::stdlib::off64_t;
}

fn gz_shift_pushback_buffer(buf: &mut [crate::stdlib::Bytef], have: usize) -> usize {
    let next = buf.len() - have;
    buf.copy_within(0..have, next);
    next
}

enum GzUngetcPlan {
    Empty { write_index: usize },
    Full,
    Existing { shift_to_end: bool },
}

fn gzungetc_pushback_plan(
    have: crate::stdlib::uInt,
    size: crate::stdlib::uInt,
    next_at_out: bool,
) -> GzUngetcPlan {
    let capacity = size << 1 as ::core::ffi::c_int;
    if have == 0 as crate::stdlib::uInt {
        GzUngetcPlan::Empty {
            write_index: capacity.wrapping_sub(1 as crate::stdlib::uInt) as usize,
        }
    } else if have == capacity {
        GzUngetcPlan::Full
    } else {
        GzUngetcPlan::Existing {
            shift_to_end: next_at_out,
        }
    }
}

fn gzgets_copy_len(
    buffered: &[crate::stdlib::Bytef],
    left: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let limit = core::cmp::min(buffered.len(), left as usize);
    match buffered[..limit].iter().position(|&byte| byte == b'\n') {
        Some(pos) => pos.wrapping_add(1) as ::core::ffi::c_uint,
        None => limit as ::core::ffi::c_uint,
    }
}

fn gzgets_copy_buffered(
    output: &mut [crate::stdlib::Bytef],
    buffered: &[crate::stdlib::Bytef],
    left: ::core::ffi::c_uint,
) -> (::core::ffi::c_uint, bool) {
    let n = gzgets_copy_len(buffered, left);
    if n != 0 {
        output[..n as usize].copy_from_slice(&buffered[..n as usize]);
    }
    let found_eol = n != 0 && buffered[n.wrapping_sub(1) as usize] == b'\n';
    (n, found_eol)
}

fn gzclose_read_status(err: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    }
}

fn gzclose_r_final_status(
    close_ret: ::core::ffi::c_int,
    read_status: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if close_ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        read_status
    }
}

macro_rules! gzgetc_body {
    ($file:expr) => {{
        let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
        let mut state: crate::gzguts_h::gz_statep =
            ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
        if $file.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        state = $file as crate::gzguts_h::gz_statep;
        if !gz_read_state_ready(&*state) {
            return -1 as ::core::ffi::c_int;
        }
        crate::src::gzlib::gz_error(
            &mut *state,
            crate::zlib_h::Z_OK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if (*state).x.have != 0 {
            let c = *(*state).x.next;
            gz_note_buffered_read(&mut *state, 1 as ::core::ffi::c_uint);
            return c as ::core::ffi::c_int;
        }
        if gz_read(
            &mut *state,
            &raw mut buf as *mut ::core::ffi::c_uchar as crate::stdlib::voidp,
            1 as crate::stdlib::z_size_t,
        ) < 1 as crate::stdlib::z_size_t
        {
            -1 as ::core::ffi::c_int
        } else {
            buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        }
    }};
}

#[export_name = "gzread"]

pub unsafe extern "C" fn gzread_ffi(
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
    if !gz_read_state_ready(&*state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        &mut *state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !gzread_len_fits_int(len) {
        crate::src::gzlib::gz_error(
            &mut *state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in an int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let state_ref = &mut *state;
    len = gz_read(state_ref, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        match gzread_zero_result(state_ref.err, state_ref.again) {
            GzReadZeroResult::Ok => {}
            GzReadZeroResult::Error => {
                return -1 as ::core::ffi::c_int;
            }
            GzReadZeroResult::Errno => {
                crate::src::gzlib::gz_error(
                    &mut *state,
                    crate::zlib_h::Z_ERRNO,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                return -1 as ::core::ffi::c_int;
            }
        }
    }
    return len as ::core::ffi::c_int;
}
#[export_name = "gzfread"]

pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
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
    if !gz_read_state_ready(&*state) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error(
        &mut *state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(len) = gz_file_request_len(size, nitems) else {
        crate::src::gzlib::gz_error(
            &mut *state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as crate::stdlib::z_size_t;
    };
    let completed = if len != 0 {
        gz_read(&mut *state, buf, len)
    } else {
        0 as crate::stdlib::z_size_t
    };
    return gz_file_completed_items(len, size, completed);
}
#[export_name = "gzgetc"]
pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc_body!(file)
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc_body!(file)
}
#[export_name = "gzungetc"]
pub unsafe extern "C" fn gzungetc_ffi(
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
        gz_look(&mut *state);
    }
    if !gz_read_state_ready(&*state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        &mut *state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    {
        let state_ref = &mut *state;
        if state_ref.skip != 0 && gz_skip(state_ref) == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    match gzungetc_pushback_plan(
        (*state).x.have,
        (*state).size,
        (*state).x.next == (*state).out,
    ) {
        GzUngetcPlan::Empty { write_index } => {
            (*state).x.have = 1 as ::core::ffi::c_uint;
            (*state).x.next = (*state).out.wrapping_add(write_index);
            *(*state).x.next = c as ::core::ffi::c_uchar;
            (*state).x.pos -= 1;
            (*state).past = 0 as ::core::ffi::c_int;
            return c;
        }
        GzUngetcPlan::Full => {
            crate::src::gzlib::gz_error(
                &mut *state,
                crate::zlib_h::Z_DATA_ERROR,
                b"out of room to push characters\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        GzUngetcPlan::Existing { shift_to_end } => {
            if shift_to_end {
                let out = ::core::slice::from_raw_parts_mut(
                    (*state).out as *mut crate::stdlib::Bytef,
                    ((*state).size << 1 as ::core::ffi::c_int) as usize,
                );
                let next = gz_shift_pushback_buffer(out, (*state).x.have as usize);
                (*state).x.next = (*state).out.wrapping_add(next);
            }
        }
    }
    (*state).x.have = (*state).x.have.wrapping_add(1);
    (*state).x.next = (*state).x.next.wrapping_sub(1);
    *(*state).x.next = c as ::core::ffi::c_uchar;
    (*state).x.pos -= 1;
    (*state).past = 0 as ::core::ffi::c_int;
    return c;
}
#[export_name = "gzgets"]
pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut left: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() || buf.is_null() || len < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_read_state_ready(&*state) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::gzlib::gz_error(
        &mut *state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    {
        let state_ref = &mut *state;
        if state_ref.skip != 0 && gz_skip(state_ref) == -1 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    str = buf;
    left = (len as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
    if left != 0 {
        while !((*state).x.have == 0 as ::core::ffi::c_uint
            && gz_fetch(&mut *state) == -1 as ::core::ffi::c_int)
        {
            if (*state).x.have == 0 as ::core::ffi::c_uint {
                (*state).past = 1 as ::core::ffi::c_int;
                break;
            } else {
                let buffered = ::core::slice::from_raw_parts(
                    (*state).x.next as *const crate::stdlib::Bytef,
                    (*state).x.have as usize,
                );
                let output = ::core::slice::from_raw_parts_mut(
                    buf as *mut crate::stdlib::Bytef,
                    left as usize,
                );
                let found_eol;
                (n, found_eol) = gzgets_copy_buffered(output, buffered, left);
                gz_note_buffered_read(&mut *state, n);
                left = left.wrapping_sub(n);
                buf = buf.wrapping_add(n as usize);
                if !(left != 0 && !found_eol) {
                    break;
                }
            }
        }
    }
    if buf == str {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *buf = 0 as ::core::ffi::c_char;
    return str;
}
pub fn gzdirect(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    return (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}

fn gz_direct_needs_look(state: &crate::gzguts_h::gz_state) -> bool {
    state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::LOOK
        && state.x.have == 0 as ::core::ffi::c_uint
}

#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let state = file as crate::gzguts_h::gz_statep;
    if gz_direct_needs_look(&*state) {
        gz_look(&mut *state);
    }
    gzdirect(&*state)
}
#[export_name = "gzclose_r"]
pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
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
        crate::src::inflate::inflateEnd_ffi(
            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
        crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
    }
    err = gzclose_read_status((*state).err);
    crate::src::gzlib::gz_error(
        &mut *state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
    ret = crate::stdlib::close((*state).fd);
    crate::stdlib::free(state as *mut ::core::ffi::c_void);
    return gzclose_r_final_status(ret, err);
}
