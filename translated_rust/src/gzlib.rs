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

/// Allocate the gzip input and output buffers using Rust-owned storage.
///
pub fn gz_init_buffers(
    state: &mut crate::gzguts_h::gz_state,
    input_len: usize,
    output_len: usize,
) -> bool {
    state.in_buf.clear();
    state.out_buf.clear();
    state.in_end = 0;
    state.out_start = 0;

    if state.in_buf.try_reserve_exact(input_len).is_err() {
        return false;
    }
    state.in_buf.resize(input_len, 0);

    if state.out_buf.try_reserve_exact(output_len).is_err() {
        state.in_buf.clear();
        return false;
    }
    state.out_buf.resize(output_len, 0);
    true
}

fn gz_reset_safe(state: &mut crate::gzguts_h::gz_state) {
    state.x.have = 0;
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0;
        state.past = 0;
        state.how = crate::gzguts_h::LOOK;
        state.junk = -1;
    } else {
        state.reset = 0;
    }
    state.again = 0;
    state.skip = 0;
    gz_error_safe(state, crate::zlib_h::Z_OK, None);
    state.x.pos = 0;
    state.strm.avail_in = 0;
}

unsafe extern "C" fn gz_reset(state: crate::gzguts_h::gz_statep) {
    gz_reset_safe(&mut *state);
}

unsafe extern "C" fn gz_open(
    mut path: *const ::core::ffi::c_void,
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut oflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut exclusive: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    state = Box::into_raw(Box::new(crate::gzguts_h::gz_state {
        x: crate::zlib_h::gzFile_s {
            have: 0,
            next: 0,
            pos: 0,
        },
        mode: crate::gzguts_h::GZ_NONE,
        fd: -1,
        write_file: None,
        read_file: None,
        gzip_inflater: None,
        path: std::ffi::CString::default(),
        size: 0,
        want: crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint,
        in_buf: Vec::new(),
        out_buf: Vec::new(),
        in_end: 0,
        out_start: 0,
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
        msg: None,
        strm: crate::zlib_h::z_stream_s {
            next_in: crate::zlib_h::InputBuffer::default(),
            avail_in: 0,
            total_in: 0,
            next_out: crate::zlib_h::OutputBuffer::default(),
            avail_out: 0,
            total_out: 0,
            msg: None,
            state: None,
            zalloc: None,
            zfree: None,
            opaque: crate::zlib_h::Opaque::default(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        },
    }));
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
                    drop(Box::from_raw(state));
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
        drop(Box::from_raw(state));
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    if (*state).mode == crate::gzguts_h::GZ_READ {
        if (*state).direct == 1 as ::core::ffi::c_int {
            drop(Box::from_raw(state));
            return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
        }
        if (*state).direct == 0 as ::core::ffi::c_int {
            (*state).direct = 1 as ::core::ffi::c_int;
        }
    } else if (*state).direct == -1 as ::core::ffi::c_int {
        drop(Box::from_raw(state));
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    (*state).path = std::ffi::CStr::from_ptr(path as *const ::core::ffi::c_char).to_owned();
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
            path as *const ::core::ffi::c_char,
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
        drop(Box::from_raw(state));
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
    gz_reset(state);
    return state as crate::zlib_h::gzFile;
}
pub unsafe extern "C" fn gzopen(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    return gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    );
}
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let file = gzopen(path, mode);
    if !file.is_null() {
        let state = &mut *(file as crate::gzguts_h::gz_statep);
        if state.mode == crate::gzguts_h::GZ_WRITE {
            state.write_file = Some(std::os::fd::FromRawFd::from_raw_fd(state.fd));
        } else if state.mode == crate::gzguts_h::GZ_READ {
            state.read_file = Some(std::os::fd::FromRawFd::from_raw_fd(state.fd));
        }
    }
    file
}
pub unsafe extern "C" fn gzopen64(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    return gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    );
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let file = gzopen64(path, mode);
    if !file.is_null() {
        let state = &mut *(file as crate::gzguts_h::gz_statep);
        if state.mode == crate::gzguts_h::GZ_WRITE {
            state.write_file = Some(std::os::fd::FromRawFd::from_raw_fd(state.fd));
        } else if state.mode == crate::gzguts_h::GZ_READ {
            state.read_file = Some(std::os::fd::FromRawFd::from_raw_fd(state.fd));
        }
    }
    file
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
    gz = gz_open(path as *const ::core::ffi::c_void, fd, mode);
    crate::stdlib::free(path as *mut ::core::ffi::c_void);
    return gz;
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let file = gzdopen(fd, mode);
    if !file.is_null() {
        let state = &mut *(file as crate::gzguts_h::gz_statep);
        if state.mode == crate::gzguts_h::GZ_WRITE {
            state.write_file = Some(std::os::fd::FromRawFd::from_raw_fd(state.fd));
        } else if state.mode == crate::gzguts_h::GZ_READ {
            state.read_file = Some(std::os::fd::FromRawFd::from_raw_fd(state.fd));
        }
    }
    file
}
pub unsafe extern "C" fn gzbuffer(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ && (*state).mode != crate::gzguts_h::GZ_WRITE {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).size != 0 as ::core::ffi::c_uint {
        return -1 as ::core::ffi::c_int;
    }
    if (size << 1 as ::core::ffi::c_int) < size {
        return -1 as ::core::ffi::c_int;
    }
    if size < 8 as ::core::ffi::c_uint {
        size = 8 as ::core::ffi::c_uint;
    }
    (*state).want = size;
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzbuffer(file, size)
}
pub fn gzrewind(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ
        || state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR
    {
        return -1;
    }
    let Ok(start) = u64::try_from(state.start) else {
        return -1;
    };
    let Some(file) = state.read_file.as_mut() else {
        return -1;
    };
    if std::io::Seek::seek(file, std::io::SeekFrom::Start(start)).is_err() {
        return -1;
    }
    gz_reset_safe(state);
    0
}
#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_mut() }) else {
        return -1;
    };
    gzrewind(state)
}
pub fn gzseek64(
    state: &mut crate::gzguts_h::gz_state,
    mut offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1;
    }
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1;
    }
    if whence != crate::stdlib::SEEK_SET && whence != crate::stdlib::SEEK_CUR {
        return -1;
    }
    if whence == crate::stdlib::SEEK_SET {
        offset -= state.x.pos;
    } else {
        offset += if state.past != 0 { 0 } else { state.skip };
        state.skip = 0;
    }
    if state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::COPY
        && state.x.pos + offset >= 0
    {
        let Some(file) = state.read_file.as_mut() else {
            return -1;
        };
        if std::io::Seek::seek(
            file,
            std::io::SeekFrom::Current(offset - state.x.have as crate::stdlib::off64_t),
        )
        .is_err()
        {
            return -1;
        }
        state.x.have = 0;
        state.eof = 0;
        state.past = 0;
        state.skip = 0;
        gz_error_safe(state, crate::zlib_h::Z_OK, None);
        state.strm.avail_in = 0;
        state.x.pos += offset;
        return state.x.pos;
    }
    if offset < 0 {
        if state.mode != crate::gzguts_h::GZ_READ {
            return -1;
        }
        offset += state.x.pos;
        if offset < 0 {
            return -1;
        }
        if gzrewind(state) == -1 {
            return -1;
        }
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        let n = if ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && state.x.have > gz_intmax()
            || state.x.have as crate::stdlib::off64_t > offset
        {
            offset as ::core::ffi::c_uint
        } else {
            state.x.have
        };
        state.x.have = state.x.have.wrapping_sub(n);
        state.x.next += n as usize;
        state.x.pos += n as crate::stdlib::off64_t;
        offset -= n as crate::stdlib::off64_t;
    }
    state.skip = offset;
    state.x.pos + offset
}
#[export_name = "gzseek64"]

pub unsafe extern "C" fn gzseek64_ffi(
    file: crate::zlib_h::gzFile,
    offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_mut() }) else {
        return -1;
    };
    gzseek64(state, offset, whence)
}
pub fn gzseek(
    state: &mut crate::gzguts_h::gz_state,
    offset: crate::stdlib::off_t,
    whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    let ret = gzseek64(state, offset, whence);
    if ret == ret {
        ret
    } else {
        -1
    }
}
#[export_name = "gzseek"]

pub unsafe extern "C" fn gzseek_ffi(
    file: crate::zlib_h::gzFile,
    offset: crate::stdlib::off_t,
    whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_mut() }) else {
        return -1;
    };
    gzseek(state, offset, whence)
}
pub fn gztell64(state: &crate::gzguts_h::gz_state) -> crate::stdlib::off64_t {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1;
    }
    state.x.pos + (if state.past != 0 { 0 } else { state.skip })
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_ref() }) else {
        return -1;
    };
    gztell64(state)
}
pub fn gztell(state: &crate::gzguts_h::gz_state) -> crate::stdlib::off_t {
    let ret = gztell64(state);
    if ret == ret {
        ret
    } else {
        -1
    }
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_ref() }) else {
        return -1;
    };
    gztell(state)
}
pub fn gzoffset64(state: &mut crate::gzguts_h::gz_state) -> crate::stdlib::off64_t {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1;
    }
    let Some(file) = (if state.mode == crate::gzguts_h::GZ_READ {
        state.read_file.as_mut()
    } else {
        state.write_file.as_mut()
    }) else {
        return -1;
    };
    let Ok(offset) = std::io::Seek::seek(file, std::io::SeekFrom::Current(0)) else {
        return -1;
    };
    let Ok(mut offset) = crate::stdlib::off64_t::try_from(offset) else {
        return -1;
    };
    if state.mode == crate::gzguts_h::GZ_READ {
        offset -= state.strm.avail_in as crate::stdlib::off64_t;
    }
    offset
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_mut() }) else {
        return -1;
    };
    gzoffset64(state)
}
pub fn gzoffset(state: &mut crate::gzguts_h::gz_state) -> crate::stdlib::off_t {
    let ret = gzoffset64(state);
    if ret == ret {
        ret
    } else {
        -1
    }
}
#[export_name = "gzoffset"]

pub unsafe extern "C" fn gzoffset_ffi(file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_mut() }) else {
        return -1;
    };
    gzoffset(state)
}
pub fn gzeof(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return 0;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.past
    } else {
        0
    }
}
#[export_name = "gzeof"]

pub unsafe extern "C" fn gzeof_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_ref() }) else {
        return 0;
    };
    gzeof(state)
}
pub fn gzerror(state: &crate::gzguts_h::gz_state) -> (::core::ffi::c_int, &std::ffi::CStr) {
    let message = if state.err == crate::zlib_h::Z_MEM_ERROR {
        c"out of memory"
    } else {
        state.msg.as_deref().unwrap_or(c"")
    };
    (state.err, message)
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    file: crate::zlib_h::gzFile,
    errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_ref() }) else {
        return ::core::ptr::null();
    };
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return ::core::ptr::null();
    }
    let (error, message) = gzerror(state);
    if !errnum.is_null() {
        unsafe { *errnum = error };
    }
    message.as_ptr()
}
pub fn gzclearerr(state: &mut crate::gzguts_h::gz_state) {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0;
        state.past = 0;
    }
    gz_error_safe(state, crate::zlib_h::Z_OK, None);
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(file: crate::zlib_h::gzFile) {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_mut() }) else {
        return;
    };
    gzclearerr(state)
}
pub unsafe extern "C" fn gz_error(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    let msg = if msg.is_null() {
        None
    } else {
        Some(std::ffi::CStr::from_ptr(msg))
    };
    gz_error_safe(&mut *state, err, msg);
}

/// Set the gzip error state using Rust-owned path and message buffers.
///
/// `CString` keeps the diagnostic stable for `gzerror()` until the next
/// error update, matching zlib's C-facing lifetime without manual malloc/free.
pub fn gz_error_safe(
    state: &mut crate::gzguts_h::gz_state,
    mut err: ::core::ffi::c_int,
    msg: Option<&std::ffi::CStr>,
) {
    state.msg = None;
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && state.again == 0 {
        state.x.have = 0;
    }
    state.err = err;
    let Some(msg) = msg else {
        return;
    };
    if err == crate::zlib_h::Z_MEM_ERROR {
        return;
    }

    let path = state.path.as_bytes();
    let message = msg.to_bytes();
    let Some(capacity) = path
        .len()
        .checked_add(2)
        .and_then(|len| len.checked_add(message.len()))
        .and_then(|len| len.checked_add(1))
    else {
        state.err = crate::zlib_h::Z_MEM_ERROR;
        return;
    };
    let mut text = Vec::new();
    if text.try_reserve_exact(capacity).is_err() {
        state.err = crate::zlib_h::Z_MEM_ERROR;
        return;
    }
    text.extend_from_slice(path);
    text.extend_from_slice(b": ");
    text.extend_from_slice(message);
    text.push(0);
    // CStr excludes its terminating NUL, so the constructed vector is always
    // a valid CString.
    state.msg = Some(std::ffi::CString::from_vec_with_nul(text).unwrap());
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
    crate::limits_h::INT_MAX as ::core::ffi::c_uint
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
