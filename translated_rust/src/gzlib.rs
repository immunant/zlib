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

// Keep gzip I/O requests within the unsigned-int sizes used by zlib's stream
// fields and the POSIX read/write adapters.
pub fn gz_stream_chunk(len: crate::stdlib::z_size_t) -> ::core::ffi::c_uint {
    let max = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    if max as crate::stdlib::z_size_t > len {
        len as ::core::ffi::c_uint
    } else {
        max
    }
}

pub fn gz_syscall_chunk(len: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let max = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2)
        .wrapping_add(1 as ::core::ffi::c_uint);
    if len > max { max } else { len }
}

// Keep byte-count arithmetic out of the raw read/write adapters.  These use
// wrapping operations to retain the translated C behavior for corrupt state.
pub(crate) fn gz_load_request(
    len: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    gz_syscall_chunk(len.wrapping_sub(have))
}

pub(crate) fn gz_add_received(
    have: ::core::ffi::c_uint,
    received: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    have.wrapping_add(received)
}

// The read and write adapters own the descriptor, errno, and raw buffer
// pointers.  Keep their common state transitions here, where they can be
// checked without expanding either raw I/O boundary.
pub(crate) fn gz_begin_io(state: &mut crate::gzguts_h::gz_state) {
    state.again = 0;
}

pub(crate) fn gz_io_result(
    state: &mut crate::gzguts_h::gz_state,
    result: ::core::ffi::c_int,
    errno: ::core::ffi::c_int,
) -> Result<::core::ffi::c_uint, ::core::ffi::c_int> {
    match gz_syscall_result(result, errno) {
        Ok(count) => Ok(count),
        Err(again) => {
            if again {
                state.again = 1;
            }
            Err(errno)
        }
    }
}

pub(crate) fn gz_load_result(
    state: &mut crate::gzguts_h::gz_state,
    result: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
    errno: ::core::ffi::c_int,
) -> Result<(), ::core::ffi::c_int> {
    if result < 0 {
        if let Err(errno) = gz_io_result(state, result, errno) {
            if state.again != 0 && have != 0 {
                return Ok(());
            }
            return Err(errno);
        }
    }
    if result == 0 {
        state.eof = 1;
    }
    Ok(())
}

pub(crate) fn gz_avail_after_load(
    state: &mut crate::gzguts_h::gz_state,
    received: ::core::ffi::c_uint,
) {
    state.strm.avail_in = state.strm.avail_in.wrapping_add(received);
}

pub(crate) fn gz_set_copy_input(
    state: &mut crate::gzguts_h::gz_state,
    copied: ::core::ffi::c_uint,
) {
    state.x.have = copied;
    state.strm.avail_in = 0;
    state.how = crate::gzguts_h::COPY;
}

pub(crate) fn gz_reset_output_buffer(state: &mut crate::gzguts_h::gz_state) {
    state.strm.avail_out = state.size;
}

pub(crate) fn gz_remaining_after_write(
    available: crate::stdlib::uInt,
    written: ::core::ffi::c_uint,
) -> crate::stdlib::uInt {
    available.wrapping_sub(written)
}

pub(crate) fn gz_produced(
    available_before: ::core::ffi::c_uint,
    available_after: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    available_before.wrapping_sub(available_after as ::core::ffi::c_uint)
}

// Account for bytes appended to the gzip input buffer without involving its
// raw buffer pointer.  The caller has already copied exactly `added` bytes.
pub(crate) fn gz_append_input(
    state: &mut crate::gzguts_h::gz_state,
    added: ::core::ffi::c_uint,
) {
    state.strm.avail_in = state.strm.avail_in.wrapping_add(added);
    gz_advance_pos(state, added);
}

// Account for the portion of a stream input request consumed by deflate.
// `avail_in` remains the source of truth for the raw stream adapter.
pub(crate) fn gz_consume_stream_input(
    state: &mut crate::gzguts_h::gz_state,
    requested: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let consumed = gz_produced(requested, state.strm.avail_in);
    gz_advance_pos(state, consumed);
    consumed
}

// Classify a POSIX I/O result without coupling the decision to the raw
// descriptor and buffer adapters.  A non-negative result is a byte count;
// a negative result preserves whether a non-blocking operation stalled.
pub(crate) fn gz_syscall_result(
    result: ::core::ffi::c_int,
    errno: ::core::ffi::c_int,
) -> Result<::core::ffi::c_uint, bool> {
    if result < 0 {
        Err(errno == crate::stdlib::EAGAIN || errno == crate::stdlib::EWOULDBLOCK)
    } else {
        Ok(result as ::core::ffi::c_uint)
    }
}

// gz_comp writes a completed output buffer, or writes while flushing except
// before Z_FINISH reaches the end of the stream.
pub(crate) fn gz_comp_needs_write(
    avail_out: ::core::ffi::c_uint,
    flush: ::core::ffi::c_int,
    ret: ::core::ffi::c_int,
) -> bool {
    avail_out == 0
        || (flush != crate::zlib_h::Z_NO_FLUSH
            && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END))
}

pub(crate) fn gz_comp_should_reset(flush: ::core::ffi::c_int) -> bool {
    flush == crate::zlib_h::Z_FINISH
}

// On a non-blocking write failure, gzip reports only the input consumed so
// far.  Other write failures report no input consumed.
pub(crate) fn gz_write_result(
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
    stalled: bool,
) -> crate::stdlib::z_size_t {
    if stalled {
        requested.wrapping_sub(remaining)
    } else {
        0
    }
}

// Choose the amount a bulk gzip operation may handle in one stream request.
// `available` is only a bound when data is already buffered.
pub(crate) fn gz_buffered_chunk(
    len: crate::stdlib::z_size_t,
    available: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let chunk = gz_stream_chunk(len);
    if chunk > available { available } else { chunk }
}

// Return how much input fits in the gzip input buffer.  Valid gzip state has
// `buffered <= size`; wrapping preserves the translated C arithmetic if a
// corrupt state reaches this internal path.
pub(crate) fn gz_buffer_space(
    size: ::core::ffi::c_uint,
    buffered: ::core::ffi::c_uint,
    remaining: crate::stdlib::z_size_t,
) -> ::core::ffi::c_uint {
    let space = size.wrapping_sub(buffered);
    if space as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        space
    }
}

// Keep the logical gzip position update independent of the raw buffer
// adapters used by the read and write paths.
pub(crate) fn gz_advance_pos(state: &mut crate::gzguts_h::gz_state, count: crate::stdlib::uInt) {
    state.x.pos += count as crate::stdlib::off64_t;
}

unsafe extern "C" fn gz_reset(mut state: crate::gzguts_h::gz_statep) {
    (*state).x.have = 0 as ::core::ffi::c_uint;
    if (*state).mode == crate::gzguts_h::GZ_READ {
        (*state).eof = 0 as ::core::ffi::c_int;
        (*state).past = 0 as ::core::ffi::c_int;
        (*state).how = crate::gzguts_h::LOOK;
        (*state).junk = -1 as ::core::ffi::c_int;
    } else {
        (*state).reset = 0 as ::core::ffi::c_int;
    }
    (*state).again = 0 as ::core::ffi::c_int;
    (*state).skip = 0 as crate::stdlib::off64_t;
    gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    (*state).x.pos = 0 as crate::stdlib::off64_t;
    (*state).strm.avail_in = 0 as crate::stdlib::uInt;
}

unsafe extern "C" fn gz_open(
    mut path: *const ::core::ffi::c_void,
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
    (*state).size = 0 as ::core::ffi::c_uint;
    (*state).want = crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint;
    (*state).err = crate::zlib_h::Z_OK;
    (*state).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).mode = crate::gzguts_h::GZ_NONE;
    (*state).level = crate::zlib_h::Z_DEFAULT_COMPRESSION;
    (*state).strategy = crate::zlib_h::Z_DEFAULT_STRATEGY;
    (*state).direct = 0 as ::core::ffi::c_int;
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
    len = crate::stdlib::strlen(path as *const ::core::ffi::c_char) as crate::stdlib::z_size_t;
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
        path as *const ::core::ffi::c_char,
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
    gzopen(path, mode)
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
    gzdopen(fd, mode)
}
pub unsafe extern "C" fn gzbuffer(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gz_buffer(&mut *(file as crate::gzguts_h::gz_statep), size)
}

fn gz_buffer(
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
    0 as ::core::ffi::c_int
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
    gz_reset(state);
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
        n = crate::src::gzread::gz_consume(&mut *state, offset);
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
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    gz_tell(&*(file as crate::gzguts_h::gz_statep))
}

fn gz_tell(state: &crate::gzguts_h::gz_state) -> crate::stdlib::off64_t {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1 as crate::stdlib::off64_t;
    }
    state.x.pos
        + (if state.past != 0 {
            0 as crate::stdlib::off64_t
        } else {
            state.skip
        })
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
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    gz_eof(&*(file as crate::gzguts_h::gz_statep))
}

fn gz_eof(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
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
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    if !(*state).msg.is_null() {
        if (*state).err != crate::zlib_h::Z_MEM_ERROR {
            crate::stdlib::free((*state).msg as *mut ::core::ffi::c_void);
        }
        (*state).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && (*state).again == 0 {
        (*state).x.have = 0 as ::core::ffi::c_uint;
    }
    (*state).err = err;
    if msg.is_null() {
        return;
    }
    if err == crate::zlib_h::Z_MEM_ERROR {
        return;
    }
    (*state).msg = crate::stdlib::malloc(
        crate::stdlib::strlen((*state).path)
            .wrapping_add(crate::stdlib::strlen(msg))
            .wrapping_add(3 as crate::__stddef_size_t_h::size_t),
    ) as *mut ::core::ffi::c_char;
    if (*state).msg.is_null() {
        (*state).err = crate::zlib_h::Z_MEM_ERROR;
        return;
    }
    crate::stdlib::snprintf(
        (*state).msg,
        crate::stdlib::strlen((*state).path)
            .wrapping_add(crate::stdlib::strlen(msg))
            .wrapping_add(3 as crate::__stddef_size_t_h::size_t),
        b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*state).path,
        b": \0".as_ptr() as *const ::core::ffi::c_char,
        msg,
    );
}
#[export_name = "gz_error"]

pub unsafe extern "C" fn gz_error_ffi(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    gz_error(state, err, msg)
}

pub fn gz_skip_chunk(
    available: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
) -> ::core::ffi::c_uint {
    if (::core::mem::size_of::<::core::ffi::c_int>()
        == ::core::mem::size_of::<crate::stdlib::off64_t>()
        && available > gz_intmax())
        || available as crate::stdlib::off64_t > skip
    {
        skip as ::core::ffi::c_uint
    } else {
        available
    }
}

pub extern "C" fn gz_intmax() -> ::core::ffi::c_uint {
    return crate::limits_h::INT_MAX as ::core::ffi::c_uint;
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
