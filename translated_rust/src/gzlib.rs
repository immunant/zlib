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

// This is the scalar portion of a gzip handle that position queries need.
// Keep it pointer-free so the query rules can move out of the ABI state before
// the resource-owning gzip facade is introduced.
struct GzPosition {
    mode: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
}

impl GzPosition {
    fn active(&self) -> bool {
        self.mode == crate::gzguts_h::GZ_READ || self.mode == crate::gzguts_h::GZ_WRITE
    }

    fn tell(&self) -> crate::stdlib::off64_t {
        if !self.active() {
            return -1 as crate::stdlib::off64_t;
        }
        self.pos
            + if self.past != 0 {
                0 as crate::stdlib::off64_t
            } else {
                self.skip
            }
    }

    fn offset(
        &self,
        current: crate::stdlib::off64_t,
        buffered_input: crate::stdlib::uInt,
    ) -> crate::stdlib::off64_t {
        if !self.active() || current == -1 as crate::stdlib::off64_t {
            return -1 as crate::stdlib::off64_t;
        }
        if self.mode == crate::gzguts_h::GZ_READ {
            current - buffered_input as crate::stdlib::off64_t
        } else {
            current
        }
    }
}

// Pointer-free seek policy.  The ABI-facing implementation applies this plan
// to the gzip state and performs I/O; a future owner facade can reuse the
// same checked transition without borrowing the raw state.
enum GzSeekAction {
    Direct {
        seek_by: crate::stdlib::off64_t,
        position: crate::stdlib::off64_t,
    },
    Rewind { offset: crate::stdlib::off64_t },
    Skip { offset: crate::stdlib::off64_t },
    Reject,
}

struct GzSeekPlan {
    clear_skip: bool,
    action: GzSeekAction,
}

fn gzseek_plan(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    how: ::core::ffi::c_int,
    have: crate::stdlib::uInt,
    mut offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
) -> Option<GzSeekPlan> {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR {
        return None;
    }
    if whence != crate::stdlib::SEEK_SET && whence != crate::stdlib::SEEK_CUR {
        return None;
    }
    let clear_skip = whence == crate::stdlib::SEEK_CUR;
    if whence == crate::stdlib::SEEK_SET {
        offset -= pos;
    } else {
        offset += if past != 0 { 0 } else { skip };
    }
    if mode == crate::gzguts_h::GZ_READ
        && how == crate::gzguts_h::COPY
        && pos + offset >= 0
    {
        return Some(GzSeekPlan {
            clear_skip,
            action: GzSeekAction::Direct {
                seek_by: offset - have as crate::stdlib::off64_t,
                position: pos + offset,
            },
        });
    }
    if offset < 0 {
        if mode != crate::gzguts_h::GZ_READ {
            return Some(GzSeekPlan { clear_skip, action: GzSeekAction::Reject });
        }
        offset += pos;
        if offset < 0 {
            return Some(GzSeekPlan { clear_skip, action: GzSeekAction::Reject });
        }
        return Some(GzSeekPlan {
            clear_skip,
            action: GzSeekAction::Rewind { offset },
        });
    }
    Some(GzSeekPlan {
        clear_skip,
        action: GzSeekAction::Skip { offset },
    })
}

pub fn gzeof(mode: ::core::ffi::c_int, past: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if mode == crate::gzguts_h::GZ_READ {
        past
    } else {
        0 as ::core::ffi::c_int
    }
}

fn gzbuffer_want(
    mode: ::core::ffi::c_int,
    current_size: ::core::ffi::c_uint,
    requested_size: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    if current_size != 0 as ::core::ffi::c_uint {
        return None;
    }
    if requested_size.wrapping_shl(1) < requested_size {
        return None;
    }
    Some(if requested_size < 8 as ::core::ffi::c_uint {
        8 as ::core::ffi::c_uint
    } else {
        requested_size
    })
}

struct GzReadResetFields {
    eof: ::core::ffi::c_int,
    past: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
}

struct GzResetFields {
    have: ::core::ffi::c_uint,
    read: Option<GzReadResetFields>,
    reset: Option<::core::ffi::c_int>,
    again: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    pos: crate::stdlib::off64_t,
    avail_in: crate::stdlib::uInt,
}

fn gz_reset_fields(mode: ::core::ffi::c_int) -> GzResetFields {
    let read = mode == crate::gzguts_h::GZ_READ;
    GzResetFields {
        have: 0,
        read: read.then_some(GzReadResetFields {
            eof: 0,
            past: 0,
            how: crate::gzguts_h::LOOK,
            junk: -1,
        }),
        reset: (!read).then_some(0),
        again: 0,
        skip: 0,
        pos: 0,
        avail_in: 0,
    }
}

unsafe fn gz_reset(state: crate::gzguts_h::gz_statep) {
    let state = &mut *state;
    let fields = gz_reset_fields(state.mode);
    state.x.have = fields.have;
    if let Some(read) = fields.read {
        state.eof = read.eof;
        state.past = read.past;
        state.how = read.how;
        state.junk = read.junk;
    }
    if let Some(reset) = fields.reset {
        state.reset = reset;
    }
    state.again = fields.again;
    state.skip = fields.skip;
    gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    state.x.pos = fields.pos;
    state.strm.avail_in = fields.avail_in;
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
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    )
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    )
}
pub unsafe extern "C" fn gzdopen(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if fd == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    // This is the same bound used by the C implementation: enough for the
    // literal label, every decimal digit of a C int, its sign, and the NUL.
    let mut path = [0 as ::core::ffi::c_char; 7 + 3 * ::core::mem::size_of::<::core::ffi::c_int>()];
    path[..4].copy_from_slice(&[
        b'<' as ::core::ffi::c_char,
        b'f' as ::core::ffi::c_char,
        b'd' as ::core::ffi::c_char,
        b':' as ::core::ffi::c_char,
    ]);
    let mut at = 4usize;
    if fd < 0 as ::core::ffi::c_int {
        path[at] = b'-' as ::core::ffi::c_char;
        at += 1;
    }
    let mut digits = [0u8; 10];
    let mut value = fd.unsigned_abs();
    let mut count = 0usize;
    loop {
        digits[count] = (value % 10) as u8;
        count += 1;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    while count != 0 {
        count -= 1;
        path[at] = (b'0' + digits[count]) as ::core::ffi::c_char;
        at += 1;
    }
    path[at] = b'>' as ::core::ffi::c_char;
    gz_open(path.as_ptr().cast::<::core::ffi::c_void>(), fd, mode)
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzdopen(fd, mode)
}
unsafe fn gzbuffer(
    mut state: Option<::core::ptr::NonNull<crate::gzguts_h::gz_state>>,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let Some(mut state) = state else {
        return -1 as ::core::ffi::c_int;
    };
    let state = state.as_mut();
    match gzbuffer_want(state.mode, state.size, size) {
        Some(want) => {
            state.want = want;
            0 as ::core::ffi::c_int
        }
        None => -1 as ::core::ffi::c_int,
    }
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzbuffer(
        ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep),
        size,
    )
}
unsafe fn gzrewind(
    mut state: Option<::core::ptr::NonNull<crate::gzguts_h::gz_state>>,
) -> ::core::ffi::c_int {
    let Some(mut state) = state else {
        return -1 as ::core::ffi::c_int;
    };
    let state = state.as_mut();
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
    gz_reset(state as *mut crate::gzguts_h::gz_state);
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzrewind(::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep))
}
pub unsafe extern "C" fn gzseek64(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    let plan = gzseek_plan(
        state.mode,
        state.err,
        state.x.pos,
        state.past,
        state.skip,
        state.how,
        state.x.have,
        offset,
        whence,
    );
    let Some(plan) = plan else {
        return -1 as crate::stdlib::off64_t;
    };
    if plan.clear_skip {
        state.skip = 0 as crate::stdlib::off64_t;
    }
    offset = match plan.action {
        GzSeekAction::Direct { seek_by, position } => {
            if crate::stdlib::lseek64(
                state.fd,
                seek_by as crate::stdlib::__off64_t,
                crate::stdlib::SEEK_CUR,
            ) == -1 as crate::stdlib::__off64_t
            {
                return -1 as crate::stdlib::off64_t;
            }
            state.x.have = 0 as ::core::ffi::c_uint;
            state.eof = 0 as ::core::ffi::c_int;
            state.past = 0 as ::core::ffi::c_int;
            state.skip = 0 as crate::stdlib::off64_t;
            gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_OK,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            state.strm.avail_in = 0 as crate::stdlib::uInt;
            state.x.pos = position;
            return state.x.pos;
        }
        GzSeekAction::Rewind { offset } => {
            if gzrewind(Some(::core::ptr::NonNull::from(&mut *state)))
                == -1 as ::core::ffi::c_int
            {
                return -1 as crate::stdlib::off64_t;
            }
            offset
        }
        GzSeekAction::Skip { offset } => offset,
        GzSeekAction::Reject => return -1 as crate::stdlib::off64_t,
    };
    let mut n: ::core::ffi::c_uint = 0;
    if state.mode == crate::gzguts_h::GZ_READ {
        n = if ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && state.x.have > gz_intmax()
            || state.x.have as crate::stdlib::off64_t > offset
        {
            offset as ::core::ffi::c_uint
        } else {
            state.x.have
        };
        state.x.have = state.x.have.wrapping_sub(n);
        state.x.next = state.x.next.offset(n as isize);
        state.x.pos += n as crate::stdlib::off64_t;
        offset -= n as crate::stdlib::off64_t;
    }
    state.skip = offset;
    state.x.pos + offset
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
fn gztell64(position: &GzPosition) -> crate::stdlib::off64_t {
    position.tell()
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as crate::stdlib::off64_t;
    };
    let state = state.as_ref();
    gztell64(&GzPosition {
        mode: state.mode,
        pos: state.x.pos,
        past: state.past,
        skip: state.skip,
    })
}
fn gztell(position: &GzPosition) -> crate::stdlib::off_t {
    let ret = gztell64(position);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as crate::stdlib::off_t;
    };
    let state = state.as_ref();
    gztell(&GzPosition {
        mode: state.mode,
        pos: state.x.pos,
        past: state.past,
        skip: state.skip,
    })
}
pub unsafe extern "C" fn gzoffset64(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    let position = GzPosition {
        mode: state.mode,
        pos: 0 as crate::stdlib::off64_t,
        past: 0 as ::core::ffi::c_int,
        skip: 0 as crate::stdlib::off64_t,
    };
    if !position.active() {
        return -1 as crate::stdlib::off64_t;
    }
    let offset = crate::stdlib::lseek64(
        state.fd,
        0 as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_CUR,
    ) as crate::stdlib::off64_t;
    position.offset(offset, state.strm.avail_in)
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
#[export_name = "gzeof"]

pub unsafe extern "C" fn gzeof_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return 0 as ::core::ffi::c_int;
    };
    let state = state.as_ref();
    gzeof(state.mode, state.past)
}
enum GzErrorMessage {
    OutOfMemory,
    Empty,
    State,
}

fn gzerror(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    has_message: bool,
    errnum: Option<&mut ::core::ffi::c_int>,
) -> Option<GzErrorMessage> {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    if let Some(errnum) = errnum {
        *errnum = err;
    }
    if err == crate::zlib_h::Z_MEM_ERROR {
        Some(GzErrorMessage::OutOfMemory)
    } else if has_message {
        Some(GzErrorMessage::State)
    } else {
        Some(GzErrorMessage::Empty)
    }
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    mut file: crate::zlib_h::gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return ::core::ptr::null::<::core::ffi::c_char>();
    };
    let state = state.as_ref();
    let errnum = ::core::ptr::NonNull::new(errnum).map(|mut errnum| errnum.as_mut());
    match gzerror(state.mode, state.err, !state.msg.is_null(), errnum) {
        None => ::core::ptr::null::<::core::ffi::c_char>(),
        Some(GzErrorMessage::OutOfMemory) => b"out of memory\0".as_ptr().cast(),
        Some(GzErrorMessage::Empty) => b"\0".as_ptr().cast(),
        Some(GzErrorMessage::State) => state.msg.cast_const(),
    }
}
unsafe fn gzclearerr(mut state: Option<::core::ptr::NonNull<crate::gzguts_h::gz_state>>) {
    let Some(mut state) = state else {
        return;
    };
    let state = state.as_mut();
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0 as ::core::ffi::c_int;
        state.past = 0 as ::core::ffi::c_int;
    }
    gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(mut file: crate::zlib_h::gzFile) {
    gzclearerr(::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep))
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
    let msg_len = crate::stdlib::strlen(msg);
    let path_len = crate::stdlib::strlen((*state).path);
    let len = path_len
        .wrapping_add(msg_len)
        .wrapping_add(3 as crate::__stddef_size_t_h::size_t);
    (*state).msg = crate::stdlib::malloc(len) as *mut ::core::ffi::c_char;
    if (*state).msg.is_null() {
        (*state).err = crate::zlib_h::Z_MEM_ERROR;
        return;
    }
    crate::stdlib::snprintf(
        (*state).msg,
        len,
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
pub fn gz_intmax() -> ::core::ffi::c_uint {
    return crate::limits_h::INT_MAX as ::core::ffi::c_uint;
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
