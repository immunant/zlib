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
        if !(*have < len) {
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
                    if !(n != 0) {
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

unsafe extern "C" fn gz_skip(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_uint = 0;
    loop {
        if (*state).x.have != 0 {
            n = if ::core::mem::size_of::<::core::ffi::c_int>() as usize
                == ::core::mem::size_of::<crate::stdlib::off64_t>() as usize
                && (*state).x.have > crate::src::gzlib::gz_intmax()
                || (*state).x.have as crate::stdlib::off64_t > (*state).skip
            {
                (*state).skip as ::core::ffi::c_uint
            } else {
                (*state).x.have
            };
            (*state).x.have = (*state).x.have.wrapping_sub(n);
            (*state).x.next = (*state).x.next.offset(n as isize);
            (*state).x.pos += n as crate::stdlib::off64_t;
            (*state).skip -= n as crate::stdlib::off64_t;
        } else {
            if (*state).eof != 0 && (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_fetch(state) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if !((*state).skip != 0) {
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
    let mut c2rust_current_block_30: u64;
    loop {
        n = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
        if n as crate::stdlib::z_size_t > len {
            n = len as ::core::ffi::c_uint;
        }
        if (*state).x.have != 0 {
            if (*state).x.have < n {
                n = (*state).x.have;
            }
            crate::stdlib::memcpy(
                buf as *mut ::core::ffi::c_void,
                (*state).x.next as *const ::core::ffi::c_void,
                n as crate::__stddef_size_t_h::size_t,
            );
            (*state).x.next = (*state).x.next.offset(n as isize);
            (*state).x.have = (*state).x.have.wrapping_sub(n);
            if (*state).err != crate::zlib_h::Z_OK {
                err = -1 as ::core::ffi::c_int;
            }
            c2rust_current_block_30 = 2719512138335094285;
        } else {
            if (*state).eof != 0 && (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if (*state).how == crate::gzguts_h::LOOK || n < (*state).size << 1 as ::core::ffi::c_int
            {
                if gz_fetch(state) == -1 as ::core::ffi::c_int
                    && (*state).x.have == 0 as ::core::ffi::c_uint
                {
                    err = -1 as ::core::ffi::c_int;
                }
                c2rust_current_block_30 = 15240798224410183470;
            } else {
                if (*state).how == crate::gzguts_h::COPY {
                    err = gz_load(state, buf as *mut ::core::ffi::c_uchar, n, &raw mut n);
                } else {
                    (*state).strm.avail_out = n as crate::stdlib::uInt;
                    (*state).strm.next_out =
                        buf as *mut ::core::ffi::c_uchar as *mut crate::stdlib::Bytef;
                    err = gz_decomp(state);
                    n = (*state).x.have;
                    (*state).x.have = 0 as ::core::ffi::c_uint;
                }
                c2rust_current_block_30 = 2719512138335094285;
            }
        }
        match c2rust_current_block_30 {
            2719512138335094285 => {
                len = len.wrapping_sub(n as crate::stdlib::z_size_t);
                buf = (buf as *mut ::core::ffi::c_char).offset(n as isize) as crate::stdlib::voidp;
                got = got.wrapping_add(n as crate::stdlib::z_size_t);
                (*state).x.pos += n as crate::stdlib::off64_t;
            }
            _ => {}
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

fn gzread_len_fits_int(len: ::core::ffi::c_uint) -> bool {
    (len as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int
}

fn gz_read_state_ready(state: &crate::gzguts_h::gz_state) -> bool {
    state.mode == crate::gzguts_h::GZ_READ
        && (state.err == crate::zlib_h::Z_OK
            || state.err == crate::zlib_h::Z_BUF_ERROR
            || state.again != 0)
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
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !gzread_len_fits_int(len) {
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
fn gzf_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    size.checked_mul(nitems)
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
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(len) = gzf_len(size, nitems) else {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as crate::stdlib::z_size_t;
    };
    return if len != 0 {
        gz_read(state, buf, len).wrapping_div(size)
    } else {
        0 as crate::stdlib::z_size_t
    };
}
#[export_name = "gzgetc"]
pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
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
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (*state).x.have != 0 {
        (*state).x.have = (*state).x.have.wrapping_sub(1);
        (*state).x.pos += 1;
        let c2rust_fresh2 = (*state).x.next;
        (*state).x.next = (*state).x.next.offset(1);
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
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc_ffi(file)
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
        gz_look(state);
    }
    if !gz_read_state_ready(&*state) {
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
        (*state).x.next = (*state)
            .out
            .offset(((*state).size << 1 as ::core::ffi::c_int) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        *(*state).x.next.offset(0 as ::core::ffi::c_int as isize) = c as ::core::ffi::c_uchar;
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
    if (*state).x.next == (*state).out {
        let mut src: *mut ::core::ffi::c_uchar = (*state).out.offset((*state).x.have as isize);
        let mut dest: *mut ::core::ffi::c_uchar = (*state)
            .out
            .offset(((*state).size << 1 as ::core::ffi::c_int) as isize);
        while src > (*state).out {
            src = src.offset(-1);
            dest = dest.offset(-1);
            *dest = *src;
        }
        (*state).x.next = dest;
    }
    (*state).x.have = (*state).x.have.wrapping_add(1);
    (*state).x.next = (*state).x.next.offset(-1);
    *(*state).x.next.offset(0 as ::core::ffi::c_int as isize) = c as ::core::ffi::c_uchar;
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
    let mut eol: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
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
                eol = crate::stdlib::memchr(
                    (*state).x.next as *const ::core::ffi::c_void,
                    '\n' as i32,
                    n as crate::__stddef_size_t_h::size_t,
                ) as *mut ::core::ffi::c_uchar;
                if !eol.is_null() {
                    n = (eol.offset_from((*state).x.next) as ::core::ffi::c_long
                        as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint);
                }
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    (*state).x.next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                (*state).x.have = (*state).x.have.wrapping_sub(n);
                (*state).x.next = (*state).x.next.offset(n as isize);
                (*state).x.pos += n as crate::stdlib::off64_t;
                left = left.wrapping_sub(n);
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
pub fn gzdirect(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    return (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let state = file as crate::gzguts_h::gz_statep;
    if (*state).mode == crate::gzguts_h::GZ_READ
        && (*state).how == crate::gzguts_h::LOOK
        && (*state).x.have == 0 as ::core::ffi::c_uint
    {
        gz_look(state);
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
        crate::src::inflate::inflateEnd(
            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
        crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
    }
    err = if (*state).err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
    ret = crate::stdlib::close((*state).fd);
    crate::stdlib::free(state as *mut ::core::ffi::c_void);
    return if ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    };
}
