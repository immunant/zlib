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

unsafe fn gz_reset(state: &mut crate::gzguts_h::gz_state) {
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
    gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    state.x.pos = 0 as crate::stdlib::off64_t;
    state.strm.avail_in = 0 as crate::stdlib::uInt;
}

fn gz_open_state(
    mode: &::core::ffi::CStr,
) -> Option<(
    crate::gzguts_h::gz_state,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
)> {
    let mut state = crate::gzguts_h::gz_state {
        x: crate::zlib_h::gzFile_s {
            have: 0,
            next: ::core::ptr::null_mut(),
            pos: 0,
        },
        mode: crate::gzguts_h::GZ_NONE,
        fd: -1,
        path: std::ffi::CString::default(),
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
        msg: ::core::ptr::null_mut(),
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
    };
    let mut oflag = 0;
    let mut exclusive = 0;

    for &byte in mode.to_bytes() {
        if byte.is_ascii_digit() {
            state.level = (byte - b'0') as ::core::ffi::c_int;
            continue;
        }
        match byte {
            b'r' => state.mode = crate::gzguts_h::GZ_READ,
            b'w' => state.mode = crate::gzguts_h::GZ_WRITE,
            b'a' => state.mode = crate::gzguts_h::GZ_APPEND,
            b'+' => return None,
            b'e' => oflag |= crate::stdlib::O_CLOEXEC,
            b'x' => exclusive = 1,
            b'f' => state.strategy = crate::zlib_h::Z_FILTERED,
            b'h' => state.strategy = crate::zlib_h::Z_HUFFMAN_ONLY,
            b'R' => state.strategy = crate::zlib_h::Z_RLE,
            b'F' => state.strategy = crate::zlib_h::Z_FIXED,
            b'G' => state.direct = -1,
            b'N' => oflag |= crate::stdlib::O_NONBLOCK,
            b'T' => state.direct = 1,
            b'b' | _ => {}
        }
    }
    if state.mode == crate::gzguts_h::GZ_NONE
        || state.mode == crate::gzguts_h::GZ_READ && state.direct == 1
        || state.mode != crate::gzguts_h::GZ_READ && state.direct == -1
    {
        return None;
    }
    if state.mode == crate::gzguts_h::GZ_READ && state.direct == 0 {
        state.direct = 1;
    }
    Some((state, oflag, exclusive))
}

fn gz_open_file(
    path: &::core::ffi::CStr,
    mode: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
) -> Option<::core::ffi::c_int> {
    use std::os::fd::IntoRawFd;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::OpenOptionsExt;

    let mut options = std::fs::OpenOptions::new();
    if mode == crate::gzguts_h::GZ_READ {
        options.read(true);
    } else {
        options.write(true).create(true);
        if exclusive != 0 {
            options.create_new(true);
        } else if mode == crate::gzguts_h::GZ_WRITE {
            options.truncate(true);
        } else {
            options.append(true);
        }
    }
    options.mode(0o666).custom_flags(
        oflag
            & (crate::stdlib::O_CLOEXEC
                | crate::stdlib::O_LARGEFILE
                | crate::stdlib::O_NONBLOCK),
    );
    options
        .open(std::ffi::OsStr::from_bytes(path.to_bytes()))
        .ok()
        .map(|file| file.into_raw_fd())
}

unsafe extern "C" fn gz_open(
    path: *const ::core::ffi::c_void,
    fd: ::core::ffi::c_int,
    mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut();
    }
    let path = ::core::ffi::CStr::from_ptr(path.cast());
    let mode = ::core::ffi::CStr::from_ptr(mode);
    let (mut state, mut oflag, exclusive) = match gz_open_state(mode) {
        Some(open) => open,
        None => return ::core::ptr::null_mut(),
    };
    let path_bytes = path.to_bytes_with_nul();
    let mut owned_path = Vec::new();
    if owned_path.try_reserve_exact(path_bytes.len()).is_err() {
        return ::core::ptr::null_mut();
    }
    owned_path.extend_from_slice(path_bytes);
    state.path = std::ffi::CString::from_vec_with_nul(owned_path)
        .expect("a C string is always a valid C string");
    oflag |= crate::stdlib::O_LARGEFILE
        | (if state.mode == crate::gzguts_h::GZ_READ {
            crate::stdlib::O_RDONLY
        } else {
            crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | (if exclusive != 0 {
                    crate::stdlib::O_EXCL
                } else {
                    0 as ::core::ffi::c_int
                })
                | (if state.mode == crate::gzguts_h::GZ_WRITE {
                    crate::stdlib::O_TRUNC
                } else {
                    crate::stdlib::O_APPEND
                })
        });
    if fd == -1 as ::core::ffi::c_int {
        state.fd = match gz_open_file(path, state.mode, exclusive, oflag) {
            Some(fd) => fd,
            None => -1 as ::core::ffi::c_int,
        };
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
        state.fd = fd;
    }
    if state.fd == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    if state.mode == crate::gzguts_h::GZ_APPEND {
        crate::stdlib::lseek64(
            state.fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_END,
        );
        state.mode = crate::gzguts_h::GZ_WRITE;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.start = crate::stdlib::lseek64(
            state.fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t;
        if state.start == -1 as crate::stdlib::off64_t {
            state.start = 0 as crate::stdlib::off64_t;
        }
    }
    // `state` is freshly constructed, so clearing it here only needs to establish
    // the mode-specific cursor state.  In particular, there is no prior error
    // message for `gz_reset()` to release.
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
    state.err = crate::zlib_h::Z_OK;
    state.x.pos = 0;
    state.strm.avail_in = 0;
    Box::into_raw(Box::new(state)) as crate::zlib_h::gzFile
}
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gz_open(path.cast(), -1 as ::core::ffi::c_int, mode)
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
    gzopen64(path, mode)
}
pub unsafe extern "C" fn gzdopen(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if fd == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let path = std::ffi::CString::new(format!("<fd:{fd}>"))
        .expect("a formatted file descriptor cannot contain a NUL byte");
    gz_open(path.as_ptr().cast(), fd, mode)
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzdopen(fd, mode)
}
unsafe fn gzbuffer_state(
    state: &mut crate::gzguts_h::gz_state,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
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

pub unsafe extern "C" fn gzbuffer(
    file: crate::zlib_h::gzFile,
    size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzbuffer_state(&mut *(file as crate::gzguts_h::gz_statep), size)
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzbuffer(file, size)
}
pub unsafe extern "C" fn gzrewind(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ
        || (*state).err != crate::zlib_h::Z_OK && (*state).err != crate::zlib_h::Z_BUF_ERROR
    {
        return -1 as ::core::ffi::c_int;
    }
    if crate::stdlib::lseek64(
        (*state).fd,
        (*state).start as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_SET,
    ) == -1 as crate::stdlib::__off64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    gz_reset(&mut *state);
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzrewind(file)
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
        if gzrewind(file) == -1 as ::core::ffi::c_int {
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
pub unsafe extern "C" fn gzseek(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gzseek64(file, offset, whence);
    return if ret == ret {
        ret
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
    gzseek(file, offset, whence)
}
pub unsafe extern "C" fn gztell64(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ && (*state).mode != crate::gzguts_h::GZ_WRITE {
        return -1 as crate::stdlib::off64_t;
    }
    return (*state).x.pos
        + (if (*state).past != 0 {
            0 as crate::stdlib::off64_t
        } else {
            (*state).skip
        });
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    gztell64(file)
}
pub unsafe extern "C" fn gztell(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gztell64(file);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    gztell(file)
}
pub unsafe extern "C" fn gzoffset64(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let mut offset: crate::stdlib::off64_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ && (*state).mode != crate::gzguts_h::GZ_WRITE {
        return -1 as crate::stdlib::off64_t;
    }
    offset = crate::stdlib::lseek64(
        (*state).fd,
        0 as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_CUR,
    ) as crate::stdlib::off64_t;
    if offset == -1 as crate::stdlib::off64_t {
        return -1 as crate::stdlib::off64_t;
    }
    if (*state).mode == crate::gzguts_h::GZ_READ {
        offset -= (*state).strm.avail_in as crate::stdlib::off64_t;
    }
    return offset;
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    gzoffset64(file)
}
pub unsafe extern "C" fn gzoffset(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gzoffset64(file);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gzoffset"]

pub unsafe extern "C" fn gzoffset_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    gzoffset(file)
}
pub unsafe extern "C" fn gzeof(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ && (*state).mode != crate::gzguts_h::GZ_WRITE {
        return 0 as ::core::ffi::c_int;
    }
    return if (*state).mode == crate::gzguts_h::GZ_READ {
        (*state).past
    } else {
        0 as ::core::ffi::c_int
    };
}
#[export_name = "gzeof"]

pub unsafe extern "C" fn gzeof_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzeof(file)
}
pub unsafe extern "C" fn gzerror(
    mut file: crate::zlib_h::gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ && (*state).mode != crate::gzguts_h::GZ_WRITE {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !errnum.is_null() {
        *errnum = (*state).err;
    }
    return if (*state).err == crate::zlib_h::Z_MEM_ERROR {
        b"out of memory\0".as_ptr() as *const ::core::ffi::c_char
    } else if (*state).msg.is_null() {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        (*state).msg as *const ::core::ffi::c_char
    };
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    mut file: crate::zlib_h::gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    gzerror(file, errnum)
}
pub unsafe extern "C" fn gzclearerr(mut file: crate::zlib_h::gzFile) {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ && (*state).mode != crate::gzguts_h::GZ_WRITE {
        return;
    }
    if (*state).mode == crate::gzguts_h::GZ_READ {
        (*state).eof = 0 as ::core::ffi::c_int;
        (*state).past = 0 as ::core::ffi::c_int;
    }
    gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(mut file: crate::zlib_h::gzFile) {
    gzclearerr(file)
}
pub unsafe extern "C" fn gz_error(
    state: crate::gzguts_h::gz_statep,
    err: ::core::ffi::c_int,
    msg: *const ::core::ffi::c_char,
) {
    let state = &mut *state;
    if !state.msg.is_null() {
        if state.err != crate::zlib_h::Z_MEM_ERROR {
            crate::stdlib::free(state.msg.cast());
        }
        state.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && state.again == 0 {
        state.x.have = 0 as ::core::ffi::c_uint;
    }
    state.err = err;
    if msg.is_null() {
        return;
    }
    if err == crate::zlib_h::Z_MEM_ERROR {
        return;
    }
    let path = state.path.to_bytes();
    let message = ::core::ffi::CStr::from_ptr(msg).to_bytes();
    let size = match path
        .len()
        .checked_add(2)
        .and_then(|size| size.checked_add(message.len()))
        .and_then(|size| size.checked_add(1))
    {
        Some(size) => size,
        None => {
            state.err = crate::zlib_h::Z_MEM_ERROR;
            return;
        }
    };
    let allocated = crate::stdlib::malloc(size).cast::<::core::ffi::c_char>();
    if allocated.is_null() {
        state.err = crate::zlib_h::Z_MEM_ERROR;
        return;
    }
    let destination = ::core::slice::from_raw_parts_mut(allocated.cast::<u8>(), size);
    destination[..path.len()].copy_from_slice(path);
    destination[path.len()..path.len() + 2].copy_from_slice(b": ");
    destination[path.len() + 2..size - 1].copy_from_slice(message);
    destination[size - 1] = 0;
    state.msg = allocated;
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
