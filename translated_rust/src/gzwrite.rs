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

fn gz_errno_error(state: &mut crate::gzguts_h::gz_state) {
    let error = std::io::Error::last_os_error();
    state.again = (error.kind() == std::io::ErrorKind::WouldBlock) as ::core::ffi::c_int;
    let message = std::ffi::CString::new(error.to_string()).ok();
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_ERRNO, message.as_deref());
}

unsafe fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if !crate::src::gzlib::gz_init_buffers(
        state,
        (state.want << 1 as ::core::ffi::c_int) as usize,
        if state.direct == 0 {
            state.want as usize
        } else {
            0
        },
    ) {
        crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_MEM_ERROR, Some(c"out of memory"));
        return -1 as ::core::ffi::c_int;
    }
    if state.direct == 0 {
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ret = crate::src::deflate::deflateInit2_(
            &mut state.strm,
            state.level,
            8 as ::core::ffi::c_int,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            state.strategy,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        );
        if ret != crate::zlib_h::Z_OK {
            state.in_buf.clear();
            state.out_buf.clear();
            state.in_end = 0;
            state.in_0 = ::core::ptr::null_mut();
            state.out = ::core::ptr::null_mut();
            crate::src::gzlib::gz_error_safe(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(c"out of memory"),
            );
            return -1 as ::core::ffi::c_int;
        }
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    state.size = state.want;
    state.out_start = 0;
    if state.direct == 0 {
        state.strm.avail_out = state.size as crate::stdlib::uInt;
        state.strm.next_out = state.out as *mut crate::stdlib::Bytef;
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
            *crate::stdlib::__errno_location() = 0;
            state.again = 0;
            put = if state.strm.avail_in > max {
                max
            } else {
                state.strm.avail_in as ::core::ffi::c_uint
            };
            writ = crate::stdlib::write(
                state.fd,
                state.strm.next_in as *const ::core::ffi::c_void,
                put as crate::__stddef_size_t_h::size_t,
            ) as ::core::ffi::c_int;
            if writ < 0 {
                gz_errno_error(state);
                return -1;
            }
            state.strm.avail_in = state.strm.avail_in.wrapping_sub(writ as ::core::ffi::c_uint);
            state.strm.next_in = state.strm.next_in.offset(writ as isize);
        }
        return 0 as ::core::ffi::c_int;
    }
    if state.reset != 0 {
        if state.strm.avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
            return 0 as ::core::ffi::c_int;
        }
        crate::src::deflate::deflateReset(&mut state.strm);
        state.reset = 0 as ::core::ffi::c_int;
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if state.strm.avail_out == 0 as crate::stdlib::uInt
            || flush != crate::zlib_h::Z_NO_FLUSH
                && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
        {
            let produced = state.size as usize - state.strm.avail_out as usize;
            while state.out_start < produced {
                put = (produced - state.out_start).min(max as usize) as ::core::ffi::c_uint;
                *crate::stdlib::__errno_location() = 0;
                state.again = 0;
                writ = crate::stdlib::write(
                    state.fd,
                    state.out_buf.as_ptr().wrapping_add(state.out_start).cast(),
                    put as crate::__stddef_size_t_h::size_t,
                ) as ::core::ffi::c_int;
                if writ < 0 {
                    gz_errno_error(state);
                    return -1;
                }
                state.out_start += writ as usize;
            }
            if state.strm.avail_out == 0 as crate::stdlib::uInt {
                state.strm.avail_out = state.size as crate::stdlib::uInt;
                state.strm.next_out = state.out as *mut crate::stdlib::Bytef;
                state.x.next = state.out;
                state.out_start = 0;
            }
        }
        have = state.strm.avail_out as ::core::ffi::c_uint;
        ret = crate::src::deflate::deflate(&mut state.strm, flush);
        if ret == crate::zlib_h::Z_STREAM_ERROR {
            crate::src::gzlib::gz_error_safe(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"internal error: deflate stream corrupt"),
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
    let mut n: ::core::ffi::c_uint = 0;
    if state.strm.avail_in != 0
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
            state.in_buf[..n as usize].fill(0);
            first = 0 as ::core::ffi::c_int;
        }
        state.strm.avail_in = n as crate::stdlib::uInt;
        state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
        state.in_end = n as usize;
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        n = n.wrapping_sub(state.strm.avail_in as ::core::ffi::c_uint);
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
    let mut len: crate::stdlib::z_size_t = input.len();
    let put: crate::stdlib::z_size_t = len;
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
                state.in_end = 0;
            }
            have = state.in_end as ::core::ffi::c_uint;
            copy = state.size.wrapping_sub(have);
            if copy as crate::stdlib::z_size_t > len {
                copy = len as ::core::ffi::c_uint;
            }
            let end = have.wrapping_add(copy) as usize;
            state.in_buf[have as usize..end].copy_from_slice(&input[..copy as usize]);
            state.in_end = end;
            state.strm.avail_in = state.strm.avail_in.wrapping_add(copy);
            state.x.pos += copy as crate::stdlib::off64_t;
            input = &input[copy as usize..];
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
        loop {
            let mut n: ::core::ffi::c_uint = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
            if n as crate::stdlib::z_size_t > len {
                n = len as ::core::ffi::c_uint;
            }
            state.strm.avail_in = n as crate::stdlib::uInt;
            state.strm.next_in = input.as_ptr() as *mut crate::stdlib::Bytef;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            n = n.wrapping_sub(state.strm.avail_in as ::core::ffi::c_uint);
            state.x.pos += n as crate::stdlib::off64_t;
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            input = &input[n as usize..];
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

fn gzwrite_usable(state: &crate::gzguts_h::gz_state) -> bool {
    state.mode == crate::gzguts_h::GZ_WRITE
        && (state.err == crate::zlib_h::Z_OK || state.again != 0)
}

enum GzWriteInput<'a> {
    Bytes(&'a [u8]),
    Null,
    TooLong,
}

unsafe fn gzwrite(
    state: &mut crate::gzguts_h::gz_state,
    input: GzWriteInput<'_>,
) -> ::core::ffi::c_int {
    if !gzwrite_usable(state) {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
    let input = match input {
        GzWriteInput::Bytes(input) => input,
        GzWriteInput::Null => return 0,
        GzWriteInput::TooLong => {
            crate::src::gzlib::gz_error_safe(
                state,
                crate::zlib_h::Z_DATA_ERROR,
                Some(c"requested length does not fit in int"),
            );
            return 0;
        }
    };
    gz_write(state, input) as ::core::ffi::c_int
}
#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    file: crate::zlib_h::gzFile,
    buf: crate::stdlib::voidpc,
    len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    let input = if len > ::core::ffi::c_int::MAX as ::core::ffi::c_uint {
        GzWriteInput::TooLong
    } else if buf.is_null() {
        GzWriteInput::Null
    } else {
        GzWriteInput::Bytes(::core::slice::from_raw_parts(
            buf as *const u8,
            len as usize,
        ))
    };
    gzwrite(state, input)
}
enum GzFwriteInput<'a> {
    Bytes(&'a [u8]),
    Null,
    Overflow,
}

unsafe fn gzfwrite(
    state: &mut crate::gzguts_h::gz_state,
    input: GzFwriteInput<'_>,
    size: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if !gzwrite_usable(state) {
        return 0;
    }
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
    let input = match input {
        GzFwriteInput::Bytes(input) => input,
        GzFwriteInput::Null => return 0,
        GzFwriteInput::Overflow => {
            crate::src::gzlib::gz_error_safe(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"request does not fit in a size_t"),
            );
            return 0;
        }
    };
    if input.is_empty() {
        return 0;
    }
    gz_write(state, input).wrapping_div(size)
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    buf: crate::stdlib::voidpc,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
    file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    if file.is_null() {
        return 0;
    }
    let input = match nitems.checked_mul(size) {
        None => GzFwriteInput::Overflow,
        Some(0) => GzFwriteInput::Bytes(&[]),
        Some(_) if buf.is_null() => GzFwriteInput::Null,
        Some(len) => GzFwriteInput::Bytes(::core::slice::from_raw_parts(buf.cast(), len)),
    };
    gzfwrite(&mut *(file as crate::gzguts_h::gz_statep), input, size)
}
unsafe fn gzputc(
    state: &mut crate::gzguts_h::gz_state,
    c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    if !gzwrite_usable(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.size != 0 {
        if state.strm.avail_in == 0 as crate::stdlib::uInt {
            state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
            state.in_end = 0;
        }
        let have = state.in_end as ::core::ffi::c_uint;
        if have < state.size {
            state.in_buf[have as usize] = c as ::core::ffi::c_uchar;
            state.in_end = have as usize + 1;
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
    file: crate::zlib_h::gzFile,
    c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1;
    }
    gzputc(&mut *(file as crate::gzguts_h::gz_statep), c)
}
unsafe fn gzputs(state: &mut crate::gzguts_h::gz_state, s: &std::ffi::CStr) -> ::core::ffi::c_int {
    if !gzwrite_usable(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
    let bytes = s.to_bytes();
    let len = bytes.len();
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || len as ::core::ffi::c_uint as crate::stdlib::z_size_t != len
    {
        crate::src::gzlib::gz_error_safe(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"string length does not fit in int"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let put = gz_write(state, bytes);
    return if len != 0 && put == 0 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        put as ::core::ffi::c_int
    };
}
#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    file: crate::zlib_h::gzFile,
    s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1;
    }
    gzputs(
        &mut *(file as crate::gzguts_h::gz_statep),
        std::ffi::CStr::from_ptr(s),
    )
}
unsafe fn gzflush(
    state: &mut crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !gzwrite_usable(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
    if flush < 0 as ::core::ffi::c_int || flush > crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    gz_comp(state, flush);
    state.err
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    file: crate::zlib_h::gzFile,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    gzflush(&mut *(file as crate::gzguts_h::gz_statep), flush)
}
pub unsafe extern "C" fn gzsetparams(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = ::core::ptr::null_mut::<crate::zlib_h::z_stream>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    strm = &raw mut (*state).strm as crate::zlib_h::z_streamp;
    if (*state).mode != crate::gzguts_h::GZ_WRITE
        || (*state).err != crate::zlib_h::Z_OK && (*state).again == 0
        || (*state).direct != 0
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if level == (*state).level && strategy == (*state).strategy {
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
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
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
        }
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if crate::stdlib::close((*state).fd) == -1 as ::core::ffi::c_int {
        ret = crate::zlib_h::Z_ERRNO;
    }
    drop(Box::from_raw(state));
    return ret;
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_w(file)
}
