pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__O_LARGEFILE;

pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZBUFSIZE;
pub use crate::gzguts_h::GZ_APPEND;
pub use crate::gzguts_h::GZ_NONE;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::gzguts_h::LOOK;
pub use crate::internal::__INT_MAX__;
pub use crate::limits_h::INT_MAX;
pub use crate::stdlib::fcntl;
pub use crate::stdlib::open;
pub use crate::stdlib::__O_CLOEXEC;
pub use crate::stdlib::F_GETFD;
pub use crate::stdlib::F_GETFL;
pub use crate::stdlib::F_SETFD;
pub use crate::stdlib::F_SETFL;
pub use crate::stdlib::O_APPEND;
pub use crate::stdlib::O_CLOEXEC;
pub use crate::stdlib::O_CREAT;
pub use crate::stdlib::O_EXCL;
pub use crate::stdlib::O_LARGEFILE;
pub use crate::stdlib::O_NONBLOCK;
pub use crate::stdlib::O_RDONLY;
pub use crate::stdlib::O_TRUNC;
pub use crate::stdlib::O_WRONLY;
pub use crate::stdlib::SEEK_CUR;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off64_t;
pub use crate::stdlib::off_t;

pub use crate::src::deflate::internal_state;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
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
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DEFAULT_COMPRESSION;
pub use crate::zlib_h::Z_DEFAULT_STRATEGY;
pub use crate::zlib_h::Z_FILTERED;
pub use crate::zlib_h::Z_FIXED;
pub use crate::zlib_h::Z_HUFFMAN_ONLY;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_RLE;

fn gz_reset_state(state: &mut crate::gzguts_h::gz_state) {
    state.x.have = 0 as ::core::ffi::c_uint;
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0 as ::core::ffi::c_int;
        state.past = 0 as ::core::ffi::c_int;
        state.how = crate::gzguts_h::LOOK;
        state.junk = -1 as ::core::ffi::c_int;
    } else {
        state.reset = 0 as ::core::ffi::c_int;
    }
    state.again = 0 as ::core::ffi::c_int;
    state.skip = 0 as crate::stdlib::off64_t;
    gz_error_state(state, crate::zlib_h::Z_OK, None, None);
    state.x.pos = 0 as crate::stdlib::off64_t;
    state.strm.avail_in = 0 as crate::stdlib::uInt;
}

unsafe fn gz_open(
    mut path: *const ::core::ffi::c_char,
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut len: crate::stdlib::z_size_t = 0;
    let mut oflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut exclusive: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    state = crate::stdlib::malloc(::core::mem::size_of::<crate::gzguts_h::gz_state>())
        as crate::gzguts_h::gz_statep;
    if state.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    state.write(crate::gzguts_h::gz_state {
        x: crate::zlib_h::gzFile_s {
            have: 0,
            next: ::core::ptr::null_mut(),
            pos: 0,
        },
        mode: crate::gzguts_h::GZ_NONE,
        fd: -1,
        path: ::core::ptr::null_mut(),
        size: 0,
        want: crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint,
        in_0: ::core::ptr::null_mut(),
        out: ::core::ptr::null_mut(),
        direct: 0,
        junk: 0,
        how: 0,
        again: 0,
        start: 0,
        eof: 0,
        past: 0,
        level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
        strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
        reset: 0,
        skip: 0,
        err: crate::zlib_h::Z_OK,
        msg: ::core::mem::ManuallyDrop::new(None),
        strm: crate::zlib_h::z_stream_s {
            next_in: ::core::ptr::null_mut(),
            avail_in: 0,
            total_in: 0,
            next_out: ::core::ptr::null_mut(),
            avail_out: 0,
            total_out: 0,
            msg: ::core::ptr::null_mut(),
            state: ::core::ptr::null_mut(),
            zalloc: None,
            zfree: None,
            opaque: ::core::ptr::null_mut(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        },
    });
    while *mode != 0 {
        if *mode as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *mode as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            (*state).level = *mode as ::core::ffi::c_int - '0' as ::core::ffi::c_int;
        } else {
            match *mode as ::core::ffi::c_int {
                114 => {
                    (*state).mode = crate::gzguts_h::GZ_READ;
                }
                119 => {
                    (*state).mode = crate::gzguts_h::GZ_WRITE;
                }
                97 => {
                    (*state).mode = crate::gzguts_h::GZ_APPEND;
                }
                43 => {
                    crate::stdlib::free(state as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
                }
                101 => {
                    oflag |= crate::stdlib::O_CLOEXEC;
                }
                120 => {
                    exclusive = 1 as ::core::ffi::c_int;
                }
                102 => {
                    (*state).strategy = crate::zlib_h::Z_FILTERED;
                }
                104 => {
                    (*state).strategy = crate::zlib_h::Z_HUFFMAN_ONLY;
                }
                82 => {
                    (*state).strategy = crate::zlib_h::Z_RLE;
                }
                70 => {
                    (*state).strategy = crate::zlib_h::Z_FIXED;
                }
                71 => {
                    (*state).direct = -1 as ::core::ffi::c_int;
                }
                78 => {
                    oflag |= crate::stdlib::O_NONBLOCK;
                }
                84 => {
                    (*state).direct = 1 as ::core::ffi::c_int;
                }
                98 | _ => {}
            }
        }
        mode = mode.offset(1);
    }
    if (*state).mode == crate::gzguts_h::GZ_NONE {
        crate::stdlib::free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    if (*state).mode == crate::gzguts_h::GZ_READ {
        if (*state).direct == 1 as ::core::ffi::c_int {
            crate::stdlib::free(state as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
        }
        if (*state).direct == 0 as ::core::ffi::c_int {
            (*state).direct = 1 as ::core::ffi::c_int;
        }
    } else if (*state).direct == -1 as ::core::ffi::c_int {
        crate::stdlib::free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    len = crate::stdlib::strlen(path) as crate::stdlib::z_size_t;
    (*state).path = crate::stdlib::malloc(
        (len as crate::__stddef_size_t_h::size_t)
            .wrapping_add(1 as crate::__stddef_size_t_h::size_t),
    ) as *mut ::core::ffi::c_char;
    if (*state).path.is_null() {
        crate::stdlib::free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    crate::stdlib::snprintf(
        (*state).path,
        (len as crate::__stddef_size_t_h::size_t)
            .wrapping_add(1 as crate::__stddef_size_t_h::size_t),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        path,
    );
    oflag |= crate::stdlib::O_LARGEFILE
        | (if (*state).mode == crate::gzguts_h::GZ_READ {
            crate::stdlib::O_RDONLY
        } else {
            crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | (if exclusive != 0 {
                    crate::stdlib::O_EXCL
                } else {
                    0 as ::core::ffi::c_int
                })
                | (if (*state).mode == crate::gzguts_h::GZ_WRITE {
                    crate::stdlib::O_TRUNC
                } else {
                    crate::stdlib::O_APPEND
                })
        });
    if fd == -1 as ::core::ffi::c_int {
        (*state).fd = crate::stdlib::open(
            path,
            oflag,
            0o666 as ::core::ffi::c_int,
        );
    } else {
        if oflag & crate::stdlib::O_NONBLOCK != 0 {
            crate::stdlib::fcntl(
                fd,
                crate::stdlib::F_SETFL,
                crate::stdlib::fcntl(fd, crate::stdlib::F_GETFL) | crate::stdlib::O_NONBLOCK,
            );
        }
        if oflag & crate::stdlib::O_CLOEXEC != 0 {
            crate::stdlib::fcntl(
                fd,
                crate::stdlib::F_SETFD,
                crate::stdlib::fcntl(fd, crate::stdlib::F_GETFD) | crate::stdlib::O_CLOEXEC,
            );
        }
        (*state).fd = fd;
    }
    if (*state).fd == -1 as ::core::ffi::c_int {
        crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
        crate::stdlib::free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    if (*state).mode == crate::gzguts_h::GZ_APPEND {
        crate::stdlib::lseek64(
            (*state).fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_END,
        );
        (*state).mode = crate::gzguts_h::GZ_WRITE;
    }
    if (*state).mode == crate::gzguts_h::GZ_READ {
        (*state).start = crate::stdlib::lseek64(
            (*state).fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t;
        if (*state).start == -1 as crate::stdlib::off64_t {
            (*state).start = 0 as crate::stdlib::off64_t;
        }
    }
    gz_reset_state(&mut *state);
    return state as crate::zlib_h::gzFile;
}
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gz_open(path, -1 as ::core::ffi::c_int, mode)
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gz_open(path, -1 as ::core::ffi::c_int, mode)
}
pub unsafe extern "C" fn gzdopen(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gz: crate::zlib_h::gzFile = ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    if fd == -1 as ::core::ffi::c_int || {
        path = crate::stdlib::malloc(
            (7 as crate::__stddef_size_t_h::size_t).wrapping_add(
                (3 as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
            ),
        ) as *mut ::core::ffi::c_char;
        path.is_null()
    } {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    crate::stdlib::snprintf(
        path,
        (7 as crate::__stddef_size_t_h::size_t).wrapping_add(
            (3 as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
        ),
        b"<fd:%d>\0".as_ptr() as *const ::core::ffi::c_char,
        fd,
    );
    gz = gz_open(path, fd, mode);
    crate::stdlib::free(path as *mut ::core::ffi::c_void);
    return gz;
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzdopen(fd, mode)
}
pub fn gzbuffer(
    state: Option<&mut crate::gzguts_h::gz_state>,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let Some(state) = state else {
        return -1 as ::core::ffi::c_int;
    };
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1 as ::core::ffi::c_int;
    }
    if state.size != 0 as ::core::ffi::c_uint {
        return -1 as ::core::ffi::c_int;
    }
    if (size << 1 as ::core::ffi::c_int) < size {
        return -1 as ::core::ffi::c_int;
    }
    if size < 8 as ::core::ffi::c_uint {
        size = 8 as ::core::ffi::c_uint;
    }
    state.want = size;
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return gzbuffer(None, size);
    }
    gzbuffer(Some(&mut *(file as crate::gzguts_h::gz_statep)), size)
}
pub unsafe fn gzrewind(
    state: Option<&mut crate::gzguts_h::gz_state>,
) -> ::core::ffi::c_int {
    let Some(state) = state else {
        return -1 as ::core::ffi::c_int;
    };
    if state.mode != crate::gzguts_h::GZ_READ
        || state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR
    {
        return -1 as ::core::ffi::c_int;
    }
    if crate::stdlib::lseek64(
        state.fd,
        state.start as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_SET,
    ) == -1 as crate::stdlib::__off64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    gz_reset_state(state);
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzrewind((file as crate::gzguts_h::gz_statep).as_mut())
}
pub unsafe extern "C" fn gzseek64(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    let mut n: ::core::ffi::c_uint = 0;
    let mut ret: crate::stdlib::off64_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ && (*state).mode != crate::gzguts_h::GZ_WRITE {
        return -1 as crate::stdlib::off64_t;
    }
    if (*state).err != crate::zlib_h::Z_OK && (*state).err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as crate::stdlib::off64_t;
    }
    if whence != crate::stdlib::SEEK_SET && whence != crate::stdlib::SEEK_CUR {
        return -1 as crate::stdlib::off64_t;
    }
    if whence == crate::stdlib::SEEK_SET {
        offset -= (*state).x.pos;
    } else {
        offset += if (*state).past != 0 {
            0 as crate::stdlib::off64_t
        } else {
            (*state).skip
        };
        (*state).skip = 0 as crate::stdlib::off64_t;
    }
    if (*state).mode == crate::gzguts_h::GZ_READ
        && (*state).how == crate::gzguts_h::COPY
        && (*state).x.pos + offset >= 0 as crate::stdlib::off64_t
    {
        ret = crate::stdlib::lseek64(
            (*state).fd,
            offset as crate::stdlib::__off64_t - (*state).x.have as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t;
        if ret == -1 as crate::stdlib::off64_t {
            return -1 as crate::stdlib::off64_t;
        }
        (*state).x.have = 0 as ::core::ffi::c_uint;
        (*state).eof = 0 as ::core::ffi::c_int;
        (*state).past = 0 as ::core::ffi::c_int;
        (*state).skip = 0 as crate::stdlib::off64_t;
        gz_error(
            state,
            crate::zlib_h::Z_OK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        (*state).strm.avail_in = 0 as crate::stdlib::uInt;
        (*state).x.pos += offset;
        return (*state).x.pos;
    }
    if offset < 0 as crate::stdlib::off64_t {
        if (*state).mode != crate::gzguts_h::GZ_READ {
            return -1 as crate::stdlib::off64_t;
        }
        offset += (*state).x.pos;
        if offset < 0 as crate::stdlib::off64_t {
            return -1 as crate::stdlib::off64_t;
        }
        if gzrewind(state.as_mut()) == -1 as ::core::ffi::c_int {
            return -1 as crate::stdlib::off64_t;
        }
    }
    if (*state).mode == crate::gzguts_h::GZ_READ {
        n = if ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && (*state).x.have > gz_intmax()
            || (*state).x.have as crate::stdlib::off64_t > offset
        {
            offset as ::core::ffi::c_uint
        } else {
            (*state).x.have
        };
        (*state).x.have = (*state).x.have.wrapping_sub(n);
        (*state).x.next = (*state).x.next.offset(n as isize);
        (*state).x.pos += n as crate::stdlib::off64_t;
        offset -= n as crate::stdlib::off64_t;
    }
    (*state).skip = offset;
    return (*state).x.pos + offset;
}
#[export_name = "gzseek64"]

pub unsafe extern "C" fn gzseek64_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    gzseek64(file, offset, whence)
}
fn gzseek_result(ret: crate::stdlib::off64_t) -> crate::stdlib::off_t {
    return if ret == ret {
        ret as crate::stdlib::off_t
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gzseek"]

pub unsafe extern "C" fn gzseek_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    gzseek_result(gzseek64(file, offset as crate::stdlib::off64_t, whence))
}
pub fn gztell64(state: Option<&crate::gzguts_h::gz_state>) -> crate::stdlib::off64_t {
    let Some(state) = state else {
        return -1 as crate::stdlib::off64_t;
    };
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1 as crate::stdlib::off64_t;
    }
    return state.x.pos
        + (if state.past != 0 {
            0 as crate::stdlib::off64_t
        } else {
            state.skip
        });
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    if file.is_null() {
        return gztell64(None);
    }
    gztell64(Some(&*(file as crate::gzguts_h::gz_statep)))
}
pub fn gztell(state: Option<&crate::gzguts_h::gz_state>) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gztell64(state);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    if file.is_null() {
        return gztell(None);
    }
    gztell(Some(&*(file as crate::gzguts_h::gz_statep)))
}
unsafe fn gzoffset64(state: Option<&crate::gzguts_h::gz_state>) -> crate::stdlib::off64_t {
    let Some(state) = state else {
        return -1 as crate::stdlib::off64_t;
    };
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1 as crate::stdlib::off64_t;
    }
    let mut offset = crate::stdlib::lseek64(
        state.fd,
        0 as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_CUR,
    ) as crate::stdlib::off64_t;
    if offset == -1 as crate::stdlib::off64_t {
        return -1 as crate::stdlib::off64_t;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        offset -= state.strm.avail_in as crate::stdlib::off64_t;
    }
    return offset;
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    gzoffset64((file as crate::gzguts_h::gz_statep).as_ref())
}
unsafe fn gzoffset(state: Option<&crate::gzguts_h::gz_state>) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gzoffset64(state);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gzoffset"]

pub unsafe extern "C" fn gzoffset_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    gzoffset((file as crate::gzguts_h::gz_statep).as_ref())
}
pub fn gzeof(state: Option<&crate::gzguts_h::gz_state>) -> ::core::ffi::c_int {
    let Some(state) = state else {
        return 0 as ::core::ffi::c_int;
    };
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return 0 as ::core::ffi::c_int;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.past
    } else {
        0 as ::core::ffi::c_int
    }
}
#[export_name = "gzeof"]

pub unsafe extern "C" fn gzeof_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return gzeof(None);
    }
    gzeof(Some(&*(file as crate::gzguts_h::gz_statep)))
}
pub enum GzErrorMessage {
    OutOfMemory,
    Empty,
    StateMessage,
}

pub fn gzerror(
    state: Option<&crate::gzguts_h::gz_state>,
) -> Option<(::core::ffi::c_int, GzErrorMessage)> {
    let state = state?;
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    let message = if state.err == crate::zlib_h::Z_MEM_ERROR {
        GzErrorMessage::OutOfMemory
    } else if state.msg.is_none() {
        GzErrorMessage::Empty
    } else {
        GzErrorMessage::StateMessage
    };
    Some((state.err, message))
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    mut file: crate::zlib_h::gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_ref() else {
        return ::core::ptr::null::<::core::ffi::c_char>();
    };
    let Some((err, message)) = gzerror(Some(state)) else {
        return ::core::ptr::null::<::core::ffi::c_char>();
    };
    if !errnum.is_null() {
        *errnum = err;
    }
    match message {
        GzErrorMessage::OutOfMemory => {
            b"out of memory\0".as_ptr() as *const ::core::ffi::c_char
        }
        GzErrorMessage::Empty => b"\0".as_ptr() as *const ::core::ffi::c_char,
        GzErrorMessage::StateMessage => state
            .msg
            .as_ref()
            .expect("state message selected only when present")
            .as_ptr(),
    }
}
pub fn gzclearerr(state: Option<&mut crate::gzguts_h::gz_state>) {
    let Some(state) = state else {
        return;
    };
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0 as ::core::ffi::c_int;
        state.past = 0 as ::core::ffi::c_int;
    }
    gz_error_state(state, crate::zlib_h::Z_OK, None, None);
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(mut file: crate::zlib_h::gzFile) {
    gzclearerr((file as crate::gzguts_h::gz_statep).as_mut())
}
pub fn gz_error_state(
    state: &mut crate::gzguts_h::gz_state,
    err: ::core::ffi::c_int,
    path: Option<&::std::ffi::CStr>,
    msg: Option<&::std::ffi::CStr>,
) {
    *state.msg = None;
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && state.again == 0 {
        state.x.have = 0 as ::core::ffi::c_uint;
    }
    state.err = err;
    let Some(msg) = msg else {
        return;
    };
    if err == crate::zlib_h::Z_MEM_ERROR {
        return;
    }
    let Some(path) = path else {
        state.err = crate::zlib_h::Z_MEM_ERROR;
        return;
    };
    let path = path.to_bytes();
    let msg = msg.to_bytes();
    let mut text = Vec::new();
    if text
        .try_reserve_exact(path.len().saturating_add(msg.len()).saturating_add(3))
        .is_err()
    {
        state.err = crate::zlib_h::Z_MEM_ERROR;
        return;
    }
    text.extend_from_slice(path);
    text.extend_from_slice(b": ");
    text.extend_from_slice(msg);
    match ::std::ffi::CString::new(text) {
        Ok(text) => *state.msg = Some(text),
        Err(_) => state.err = crate::zlib_h::Z_MEM_ERROR,
    }
}

pub unsafe fn gz_error(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    let state = &mut *state;
    let message = (!msg.is_null()).then(|| ::std::ffi::CStr::from_ptr(msg));
    let path = message.map(|_| ::std::ffi::CStr::from_ptr(state.path));
    gz_error_state(state, err, path, message);
}
#[export_name = "gz_error"]

pub unsafe extern "C" fn gz_error_ffi(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    gz_error(state, err, msg)
}
pub fn gz_intmax() -> ::core::ffi::c_uint {
    return crate::limits_h::INT_MAX as ::core::ffi::c_uint;
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
