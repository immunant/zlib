pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_clamped_uint;
pub use crate::src::gzlib::gz_errno_is_retryable;
pub use crate::src::gzlib::gz_error;
pub(crate) use crate::src::gzlib::gz_error_with_os_error;
pub(crate) use crate::src::gzlib::gz_file_completed_items;
pub(crate) use crate::src::gzlib::gz_file_request_len;
pub use crate::src::gzlib::gz_io_chunk_len;
pub use crate::src::gzlib::gz_io_chunk_limit;
pub use crate::src::gzlib::gz_uInt_fits_int;
pub use crate::src::gzlib::gz_z_size_to_uInt_chunk;

pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::deflateEnd_ffi;
pub use crate::src::deflate::deflateInit2__ffi;
pub use crate::src::deflate::deflateReset_ffi;
pub use crate::src::deflate::deflate_ffi;
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

fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    state.in_0 = unsafe {
        crate::stdlib::malloc(
            (state.want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
        ) as *mut ::core::ffi::c_uchar
    };
    if state.in_0.is_null() {
        crate::src::gzlib::gz_error_static(state, crate::zlib_h::Z_MEM_ERROR, b"out of memory\0");
        return -1 as ::core::ffi::c_int;
    }
    if state.direct == 0 {
        state.out = unsafe {
            crate::stdlib::malloc(state.want as crate::__stddef_size_t_h::size_t)
                as *mut ::core::ffi::c_uchar
        };
        if state.out.is_null() {
            unsafe {
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            }
            crate::src::gzlib::gz_error_static(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ret = unsafe {
            crate::src::deflate::deflateInit2__ffi(
                &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                state.level,
                8 as ::core::ffi::c_int,
                15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int,
                state.strategy,
                crate::zlib_h::ZLIB_VERSION.as_ptr(),
                ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
            )
        };
        if ret != crate::zlib_h::Z_OK {
            unsafe {
                crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            }
            crate::src::gzlib::gz_error_static(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    state.size = state.want;
    if state.direct == 0 {
        gz_reset_write_output(state);
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_comp(
    state: &mut crate::gzguts_h::gz_state,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = gz_io_chunk_limit();
    if state.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.direct != 0 {
        while state.strm.avail_in != 0 {
            state.again = 0 as ::core::ffi::c_int;
            put = gz_io_chunk_len(state.strm.avail_in);
            writ = unsafe {
                crate::stdlib::write(
                    state.fd,
                    state.strm.next_in as *const ::core::ffi::c_void,
                    put as crate::__stddef_size_t_h::size_t,
                ) as ::core::ffi::c_int
            };
            let write_result = if writ < 0 as ::core::ffi::c_int {
                gz_write_syscall_result(writ, gz_last_os_errno())
            } else {
                gz_write_syscall_result(writ, 0 as ::core::ffi::c_int)
            };
            match write_result {
                GzWriteSyscallResult::Wrote(written) => {
                    state.strm.avail_in = state
                        .strm
                        .avail_in
                        .wrapping_sub(written as ::core::ffi::c_uint);
                    state.strm.next_in = state.strm.next_in.wrapping_add(written as usize);
                }
                GzWriteSyscallResult::Error { errno, again } => {
                    state.again = again;
                    gz_error_with_os_error(state, crate::zlib_h::Z_ERRNO, errno);
                    return -1 as ::core::ffi::c_int;
                }
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    match gz_comp_reset_action(state.reset, state.strm.avail_in, flush) {
        GzCompResetAction::None => {}
        GzCompResetAction::ReturnOk => {
            return 0 as ::core::ffi::c_int;
        }
        GzCompResetAction::ResetStream => {
            unsafe {
                crate::src::deflate::deflateReset_ffi(
                    &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                );
            }
            state.reset = 0 as ::core::ffi::c_int;
        }
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if gz_comp_should_write_pending(state.strm.avail_out, flush, ret) {
            while let Some(chunk) = gz_pending_output_chunk(state, max) {
                state.again = 0 as ::core::ffi::c_int;
                put = chunk;
                writ = unsafe {
                    crate::stdlib::write(
                        state.fd,
                        state.x.next as *const ::core::ffi::c_void,
                        put as crate::__stddef_size_t_h::size_t,
                    ) as ::core::ffi::c_int
                };
                let write_result = if writ < 0 as ::core::ffi::c_int {
                    gz_write_syscall_result(writ, gz_last_os_errno())
                } else {
                    gz_write_syscall_result(writ, 0 as ::core::ffi::c_int)
                };
                match write_result {
                    GzWriteSyscallResult::Wrote(written) => {
                        state.x.next = state.x.next.wrapping_add(written as usize);
                    }
                    GzWriteSyscallResult::Error { errno, again } => {
                        state.again = again;
                        gz_error_with_os_error(state, crate::zlib_h::Z_ERRNO, errno);
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
            if state.strm.avail_out == 0 as crate::stdlib::uInt {
                gz_reset_write_output(state);
            }
        }
        have = state.strm.avail_out as ::core::ffi::c_uint;
        ret = unsafe {
            crate::src::deflate::deflate_ffi(
                &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                flush,
            )
        };
        if ret == crate::zlib_h::Z_STREAM_ERROR {
            crate::src::gzlib::gz_error_static(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        have = have.wrapping_sub(state.strm.avail_out as ::core::ffi::c_uint);
        if !(have != 0) {
            break;
        }
    }
    if flush == crate::zlib_h::Z_FINISH {
        state.reset = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_zero(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    if state.strm.avail_in != 0
        && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    first = 1 as ::core::ffi::c_int;
    loop {
        n = gz_clamped_uint(state.size, state.skip);
        if first != 0 {
            let zero_buf = unsafe { ::core::slice::from_raw_parts_mut(state.in_0, n as usize) };
            gz_fill_zero(zero_buf);
            first = 0 as ::core::ffi::c_int;
        }
        state.strm.avail_in = n as crate::stdlib::uInt;
        state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        n = gz_note_input_consumed(state, n);
        state.skip -= n as crate::stdlib::off64_t;
        if ret == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if !(state.skip != 0) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_write(
    state: &mut crate::gzguts_h::gz_state,
    input: &[crate::stdlib::Bytef],
) -> crate::stdlib::z_size_t {
    let mut len: crate::stdlib::z_size_t = input.len() as crate::stdlib::z_size_t;
    let put: crate::stdlib::z_size_t = len;
    let mut input_offset: usize = 0;
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
            let mut have: ::core::ffi::c_uint = 0;
            let mut copy: ::core::ffi::c_uint = 0;
            if state.strm.avail_in == 0 as crate::stdlib::uInt {
                state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
            }
            have = gz_buffered_input_used(state);
            copy = gz_buffered_write_copy_len(state.size, have, len);
            unsafe {
                crate::stdlib::memcpy(
                    state.in_0.wrapping_add(have as usize) as *mut ::core::ffi::c_void,
                    input.as_ptr().wrapping_add(input_offset) as *const ::core::ffi::c_void,
                    copy as crate::__stddef_size_t_h::size_t,
                );
            }
            gz_note_buffered_input(state, copy);
            input_offset = input_offset.wrapping_add(copy as usize);
            len = len.wrapping_sub(copy as crate::stdlib::z_size_t);
            if len == 0 as crate::stdlib::z_size_t {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return gz_write_error_return(state.again, put, len);
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
            let mut n: ::core::ffi::c_uint = gz_z_size_to_uInt_chunk(len);
            state.strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            n = gz_note_input_consumed(state, n);
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            if ret == -1 as ::core::ffi::c_int {
                return gz_write_error_return(state.again, put, len);
            }
            if !(len != 0) {
                break;
            }
        }
    }
    return put;
}
fn gz_write_state_ready(state: &crate::gzguts_h::gz_state) -> bool {
    state.mode == crate::gzguts_h::GZ_WRITE
        && (state.err == crate::zlib_h::Z_OK || state.again != 0)
}

fn gz_write_params_ready(state: &crate::gzguts_h::gz_state) -> bool {
    gz_write_state_ready(state) && state.direct == 0
}

fn gz_write_error_return(
    again: ::core::ffi::c_int,
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if again != 0 {
        requested.wrapping_sub(remaining)
    } else {
        0 as crate::stdlib::z_size_t
    }
}

fn gz_write_errno_again(errno: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if gz_errno_is_retryable(errno) {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }
}

fn gz_last_os_errno() -> ::core::ffi::c_int {
    ::std::io::Error::last_os_error()
        .raw_os_error()
        .unwrap_or(0 as ::core::ffi::c_int)
}

enum GzWriteSyscallResult {
    Wrote(::core::ffi::c_int),
    Error {
        errno: ::core::ffi::c_int,
        again: ::core::ffi::c_int,
    },
}

fn gz_write_syscall_result(
    writ: ::core::ffi::c_int,
    errno: ::core::ffi::c_int,
) -> GzWriteSyscallResult {
    if writ < 0 as ::core::ffi::c_int {
        GzWriteSyscallResult::Error {
            errno,
            again: gz_write_errno_again(errno),
        }
    } else {
        GzWriteSyscallResult::Wrote(writ)
    }
}

fn gz_comp_should_write_pending(
    avail_out: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    ret: ::core::ffi::c_int,
) -> bool {
    avail_out == 0 as crate::stdlib::uInt
        || flush != crate::zlib_h::Z_NO_FLUSH
            && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
}

enum GzCompResetAction {
    None,
    ReturnOk,
    ResetStream,
}

fn gz_comp_reset_action(
    reset: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> GzCompResetAction {
    if reset == 0 {
        GzCompResetAction::None
    } else if avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
        GzCompResetAction::ReturnOk
    } else {
        GzCompResetAction::ResetStream
    }
}

fn gz_note_buffered_input(state: &mut crate::gzguts_h::gz_state, count: ::core::ffi::c_uint) {
    state.strm.avail_in = state.strm.avail_in.wrapping_add(count);
    state.x.pos += count as crate::stdlib::off64_t;
}

fn gz_note_input_consumed(
    state: &mut crate::gzguts_h::gz_state,
    requested: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let consumed = requested.wrapping_sub(state.strm.avail_in as ::core::ffi::c_uint);
    state.x.pos += consumed as crate::stdlib::off64_t;
    consumed
}

fn gz_reset_write_output(state: &mut crate::gzguts_h::gz_state) {
    state.strm.avail_out = state.size as crate::stdlib::uInt;
    state.strm.next_out = state.out as *mut crate::stdlib::Bytef;
    state.x.next = state.out;
}

fn gz_buffered_input_used(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_uint {
    gz_buffered_input_used_addrs(
        state.strm.next_in as usize,
        state.strm.avail_in,
        state.in_0 as usize,
    )
}

fn gz_buffered_input_used_addrs(
    next_in_addr: usize,
    avail_in: crate::stdlib::uInt,
    base_addr: usize,
) -> ::core::ffi::c_uint {
    next_in_addr
        .wrapping_add(avail_in as usize)
        .wrapping_sub(base_addr) as ::core::ffi::c_uint
}

fn gz_buffered_write_copy_len(
    size: crate::stdlib::uInt,
    used: crate::stdlib::uInt,
    len: crate::stdlib::z_size_t,
) -> crate::stdlib::uInt {
    let space = size.wrapping_sub(used);
    if space as crate::stdlib::z_size_t > len {
        len as crate::stdlib::uInt
    } else {
        space
    }
}

fn gz_pending_output_chunk(
    state: &crate::gzguts_h::gz_state,
    max: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    gz_pending_output_chunk_addrs(state.strm.next_out as usize, state.x.next as usize, max)
}

fn gz_pending_output_chunk_addrs(
    next_out_addr: usize,
    next_addr: usize,
    max: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    if next_out_addr <= next_addr {
        return None;
    }
    let pending = next_out_addr - next_addr;
    Some(if pending > max as usize {
        max
    } else {
        pending as ::core::ffi::c_uint
    })
}

fn gz_fill_zero(buf: &mut [crate::stdlib::Bytef]) {
    buf.fill(0);
}

fn gz_store_buffered_byte(
    buf: &mut [crate::stdlib::Bytef],
    offset: ::core::ffi::c_uint,
    c: ::core::ffi::c_int,
) {
    buf[offset as usize] = c as ::core::ffi::c_uchar;
}

fn gzputc_buffered(
    state: &mut crate::gzguts_h::gz_state,
    buf: &mut [crate::stdlib::Bytef],
    c: ::core::ffi::c_int,
) -> Option<::core::ffi::c_int> {
    if state.size == 0 {
        return None;
    }
    if state.strm.avail_in == 0 as crate::stdlib::uInt {
        state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
    }
    let have = gz_buffered_input_used(state);
    if have < state.size {
        gz_store_buffered_byte(buf, have, c);
        gz_note_buffered_input(state, 1 as ::core::ffi::c_uint);
        Some(c & 0xff as ::core::ffi::c_int)
    } else {
        None
    }
}

fn gzwrite_len_fits_int(len: ::core::ffi::c_uint) -> bool {
    gz_uInt_fits_int(len)
}

#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    if !gzwrite_len_fits_int(len) {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            b"requested length does not fit in int\0",
        );
        return 0 as ::core::ffi::c_int;
    }
    let input = if len == 0 {
        &[] as &[crate::stdlib::Bytef]
    } else {
        ::core::slice::from_raw_parts(buf as *const crate::stdlib::Bytef, len as usize)
    };
    return gz_write(state, input) as ::core::ffi::c_int;
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    let Some(len) = gz_file_request_len(size, nitems) else {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0",
        );
        return 0 as crate::stdlib::z_size_t;
    };
    let completed = if len != 0 {
        let input = ::core::slice::from_raw_parts(buf as *const crate::stdlib::Bytef, len);
        gz_write(state, input)
    } else {
        0 as crate::stdlib::z_size_t
    };
    return gz_file_completed_items(len, size, completed);
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.size != 0 {
        let buffered = ::core::slice::from_raw_parts_mut(state.in_0, state.size as usize);
        if let Some(ret) = gzputc_buffered(state, buffered, c) {
            return ret;
        }
    }
    buf[0 as ::core::ffi::c_int as usize] = c as ::core::ffi::c_uchar;
    if gz_write(state, &buf) != 1 as crate::stdlib::z_size_t {
        return -1 as ::core::ffi::c_int;
    }
    return c & 0xff as ::core::ffi::c_int;
}
fn gzputs_len_fits_int(len: crate::stdlib::z_size_t) -> bool {
    (len as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int
        && len as ::core::ffi::c_uint as crate::stdlib::z_size_t == len
}

fn gzputs_return_value(
    len: crate::stdlib::z_size_t,
    put: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if len != 0 && put == 0 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        put as ::core::ffi::c_int
    }
}

#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: crate::stdlib::z_size_t = 0;
    let mut put: crate::stdlib::z_size_t = 0;
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    let input = ::core::ffi::CStr::from_ptr(s).to_bytes();
    len = input.len() as crate::stdlib::z_size_t;
    if !gzputs_len_fits_int(len) {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"string length does not fit in int\0",
        );
        return -1 as ::core::ffi::c_int;
    }
    put = gz_write(state, input);
    return gzputs_return_value(len, put);
}
fn gzflush_valid_flush(flush: ::core::ffi::c_int) -> bool {
    flush >= 0 as ::core::ffi::c_int && flush <= crate::zlib_h::Z_FINISH
}

#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    if !gzflush_valid_flush(flush) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    gz_comp(state, flush);
    return state.err;
}
fn gzsetparams_unchanged(
    state: &crate::gzguts_h::gz_state,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> bool {
    level == state.level && strategy == state.strategy
}

fn gzclose_w_after_step(
    ret: ::core::ffi::c_int,
    failed: bool,
    state_err: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if failed {
        state_err
    } else {
        ret
    }
}

fn gzclose_w_final_status(ret: ::core::ffi::c_int, close_failed: bool) -> ::core::ffi::c_int {
    if close_failed {
        crate::zlib_h::Z_ERRNO
    } else {
        ret
    }
}

#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_params_ready(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    if gzsetparams_unchanged(state, level, strategy) {
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
        crate::src::deflate::deflateParams_ffi(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            level,
            strategy,
        );
    }
    state.level = level;
    state.strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "gzclose_w"]
pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_ptr = file as crate::gzguts_h::gz_statep;
    let state = &mut *state_ptr;
    if state.mode != crate::gzguts_h::GZ_WRITE {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let zero_failed = state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int;
    ret = gzclose_w_after_step(ret, zero_failed, state.err);
    let comp_failed = gz_comp(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int;
    ret = gzclose_w_after_step(ret, comp_failed, state.err);
    if state.size != 0 {
        if state.direct == 0 {
            crate::src::deflate::deflateEnd_ffi(
                &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            );
            crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
        }
        crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    crate::stdlib::free(state.path as *mut ::core::ffi::c_void);
    ret = gzclose_w_final_status(
        ret,
        crate::stdlib::close(state.fd) == -1 as ::core::ffi::c_int,
    );
    crate::stdlib::free(state_ptr as *mut ::core::ffi::c_void);
    return ret;
}
