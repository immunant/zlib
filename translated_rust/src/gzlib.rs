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

/// Open a named gzip stream without exposing C pointers to implementation code.
pub fn gzopen(path: &std::ffi::CStr, mode: &std::ffi::CStr) -> Option<Box<crate::gzguts_h::gz_state>> {
    use std::os::unix::ffi::OsStrExt as _;
    use std::os::unix::fs::OpenOptionsExt as _;

    let mode = parse_gz_mode(mode)?;
    let mut options = std::fs::OpenOptions::new();
    if mode.mode == crate::gzguts_h::GZ_READ {
        options.read(true);
    } else {
        options.write(true).create(true);
        if mode.exclusive {
            options.create_new(true);
        } else if mode.mode == crate::gzguts_h::GZ_WRITE {
            options.truncate(true);
        } else {
            options.append(true);
        }
    }
    let flags = (if mode.close_on_exec { crate::stdlib::O_CLOEXEC } else { 0 })
        | (if mode.nonblocking { crate::stdlib::O_NONBLOCK } else { 0 });
    options.mode(0o666).custom_flags(flags);
    let file = options
        .open(std::path::Path::new(std::ffi::OsStr::from_bytes(path.to_bytes())))
        .ok()?;
    Some(gzopen_with_file(path.to_owned(), mode, file))
}

/// `gzopen64` has the same safe implementation as `gzopen` on this target.
pub fn gzopen64(path: &std::ffi::CStr, mode: &std::ffi::CStr) -> Option<Box<crate::gzguts_h::gz_state>> {
    gzopen(path, mode)
}
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    path: *const ::core::ffi::c_char,
    mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut();
    }
    let path = unsafe { std::ffi::CStr::from_ptr(path) };
    let mode = unsafe { std::ffi::CStr::from_ptr(mode) };
    let Some(state) = gzopen(path, mode) else {
        return ::core::ptr::null_mut();
    };
    Box::into_raw(state) as crate::zlib_h::gzFile
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    path: *const ::core::ffi::c_char,
    mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut();
    }
    let path = unsafe { std::ffi::CStr::from_ptr(path) };
    let mode = unsafe { std::ffi::CStr::from_ptr(mode) };
    let Some(state) = gzopen64(path, mode) else {
        return ::core::ptr::null_mut();
    };
    Box::into_raw(state) as crate::zlib_h::gzFile
}
/// Adopt an already-open file for gzip I/O using only Rust-owned state.
pub fn gzdopen(
    file: std::fs::File,
    mode: &std::ffi::CStr,
) -> Option<Box<crate::gzguts_h::gz_state>> {
    use std::os::fd::AsRawFd as _;

    let mode = parse_gz_mode(mode)?;
    let path = std::ffi::CString::new(format!("<fd:{}>", file.as_raw_fd())).ok()?;
    Some(gzopen_with_file(path, mode, file))
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    fd: ::core::ffi::c_int,
    mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    use std::os::fd::FromRawFd as _;

    if fd < 0 || mode.is_null() {
        return ::core::ptr::null_mut();
    }
    let mode = unsafe { std::ffi::CStr::from_ptr(mode) };
    let Some(settings) = parse_gz_mode(mode) else {
        return ::core::ptr::null_mut();
    };
    if settings.nonblocking {
        unsafe {
            crate::stdlib::fcntl(
                fd,
                crate::stdlib::F_SETFL,
                crate::stdlib::fcntl(fd, crate::stdlib::F_GETFL) | crate::stdlib::O_NONBLOCK,
            );
        }
    }
    if settings.close_on_exec {
        unsafe {
            crate::stdlib::fcntl(
                fd,
                crate::stdlib::F_SETFD,
                crate::stdlib::fcntl(fd, crate::stdlib::F_GETFD) | crate::stdlib::O_CLOEXEC,
            );
        }
    }
    let file = unsafe { std::fs::File::from_raw_fd(fd) };
    let Some(state) = gzdopen(file, mode) else {
        return ::core::ptr::null_mut();
    };
    Box::into_raw(state) as crate::zlib_h::gzFile
}
/// Set the requested buffer size before any gzip I/O has started.
pub fn gzbuffer(
    state: &mut crate::gzguts_h::gz_state,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1;
    }
    if state.size != 0 {
        return -1;
    }
    if size.checked_mul(2).is_none() {
        return -1;
    }
    if size < 8 {
        size = 8;
    }
    state.want = size;
    0
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    file: crate::zlib_h::gzFile,
    size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let Some(state) = (unsafe { (file as crate::gzguts_h::gz_statep).as_mut() }) else {
        return -1;
    };
    gzbuffer(state, size)
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
    state: crate::gzguts_h::gz_statep,
    err: ::core::ffi::c_int,
    msg: *const ::core::ffi::c_char,
) {
    let Some(state) = (unsafe { state.as_mut() }) else {
        return;
    };
    let msg = if msg.is_null() {
        None
    } else {
        Some(unsafe { std::ffi::CStr::from_ptr(msg) })
    };
    gz_error_safe(state, err, msg);
}
pub fn gz_intmax() -> ::core::ffi::c_uint {
    crate::limits_h::INT_MAX as ::core::ffi::c_uint
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}

struct GzOpenMode {
    mode: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    close_on_exec: bool,
    nonblocking: bool,
    exclusive: bool,
}

fn parse_gz_mode(mode: &std::ffi::CStr) -> Option<GzOpenMode> {
    let mut result = GzOpenMode {
        mode: crate::gzguts_h::GZ_NONE,
        level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
        strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
        direct: 0,
        close_on_exec: false,
        nonblocking: false,
        exclusive: false,
    };
    for byte in mode.to_bytes() {
        match *byte {
            b'0'..=b'9' => result.level = (byte - b'0') as ::core::ffi::c_int,
            b'r' => result.mode = crate::gzguts_h::GZ_READ,
            b'w' => result.mode = crate::gzguts_h::GZ_WRITE,
            b'a' => result.mode = crate::gzguts_h::GZ_APPEND,
            b'+' => return None,
            b'e' => result.close_on_exec = true,
            b'x' => result.exclusive = true,
            b'f' => result.strategy = crate::zlib_h::Z_FILTERED,
            b'h' => result.strategy = crate::zlib_h::Z_HUFFMAN_ONLY,
            b'R' => result.strategy = crate::zlib_h::Z_RLE,
            b'F' => result.strategy = crate::zlib_h::Z_FIXED,
            b'G' => result.direct = -1,
            b'N' => result.nonblocking = true,
            b'T' => result.direct = 1,
            _ => {},
        }
    }
    if result.mode == crate::gzguts_h::GZ_NONE
        || (result.mode == crate::gzguts_h::GZ_READ && result.direct == 1)
        || (result.mode != crate::gzguts_h::GZ_READ && result.direct == -1)
    {
        return None;
    }
    if result.mode == crate::gzguts_h::GZ_READ && result.direct == 0 {
        result.direct = 1;
    }
    Some(result)
}

fn new_gz_state(path: std::ffi::CString) -> crate::gzguts_h::gz_state {
    crate::gzguts_h::gz_state {
        x: crate::zlib_h::gzFile_s { have: 0, next: 0, pos: 0 },
        mode: crate::gzguts_h::GZ_NONE,
        fd: -1,
        write_file: None,
        read_file: None,
        gzip_inflater: None,
        path,
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
    }
}

fn gzopen_with_file(
    path: std::ffi::CString,
    mode: GzOpenMode,
    mut file: std::fs::File,
) -> Box<crate::gzguts_h::gz_state> {
    use std::io::{Seek as _, SeekFrom};
    use std::os::fd::AsRawFd as _;

    let mut state = new_gz_state(path);
    state.mode = mode.mode;
    state.level = mode.level;
    state.strategy = mode.strategy;
    state.direct = mode.direct;
    if state.mode == crate::gzguts_h::GZ_APPEND {
        let _ = file.seek(SeekFrom::End(0));
        state.mode = crate::gzguts_h::GZ_WRITE;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.start = file.stream_position().unwrap_or(0) as crate::stdlib::off64_t;
    }
    state.fd = file.as_raw_fd();
    if state.mode == crate::gzguts_h::GZ_READ {
        state.read_file = Some(file);
    } else {
        state.write_file = Some(file);
    }
    gz_reset_safe(&mut state);
    Box::new(state)
}
