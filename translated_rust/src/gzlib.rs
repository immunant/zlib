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

// The offset query needs one resource in addition to the scalar position
// state.  Borrow it separately so the query implementation stays independent
// of the ABI-shaped gzip handle.
struct GzOffsetQuery<'a> {
    position: GzPosition,
    fd: Option<&'a rustix::fd::OwnedFd>,
    buffered_input: crate::stdlib::uInt,
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
    Rewind {
        offset: crate::stdlib::off64_t,
    },
    Skip {
        offset: crate::stdlib::off64_t,
    },
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
    if mode == crate::gzguts_h::GZ_READ && how == crate::gzguts_h::COPY && pos + offset >= 0 {
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
            return Some(GzSeekPlan {
                clear_skip,
                action: GzSeekAction::Reject,
            });
        }
        offset += pos;
        if offset < 0 {
            return Some(GzSeekPlan {
                clear_skip,
                action: GzSeekAction::Reject,
            });
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

pub(crate) fn gz_clear_error(message: &mut Option<Box<[u8]>>, error: &mut ::core::ffi::c_int) {
    *message = None;
    *error = crate::zlib_h::Z_OK;
}

// Keep the error-state transition independent of the ABI-shaped gzip handle.
// The callers that still hold that handle only provide the scalar fields and
// owned byte views; a later gzip owner facade can use this directly.
pub(crate) fn gz_set_error(
    stored_message: &mut Option<Box<[u8]>>,
    error: &mut ::core::ffi::c_int,
    buffered: &mut ::core::ffi::c_uint,
    again: ::core::ffi::c_int,
    path: Option<&[u8]>,
    err: ::core::ffi::c_int,
    message: Option<&[u8]>,
) {
    *stored_message = None;
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && again == 0 {
        *buffered = 0;
    }
    *error = err;
    let Some(message) = message else {
        return;
    };
    if err == crate::zlib_h::Z_MEM_ERROR {
        return;
    }
    let Some(len) = path
        .and_then(|path| path.len().checked_add(message.len()))
        .and_then(|len| len.checked_add(3))
    else {
        *error = crate::zlib_h::Z_MEM_ERROR;
        return;
    };
    let mut text = Vec::new();
    if text.try_reserve_exact(len).is_err() {
        *error = crate::zlib_h::Z_MEM_ERROR;
        return;
    }
    text.extend_from_slice(path.unwrap());
    text.extend_from_slice(b": ");
    text.extend_from_slice(message);
    text.push(0);
    *stored_message = Some(text.into_boxed_slice());
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

pub(crate) fn gz_buffer(size: ::core::ffi::c_uint) -> Option<Box<[u8]>> {
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size as usize).ok()?;
    bytes.resize(size as usize, 0);
    Some(bytes.into_boxed_slice())
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

// The reset transition deliberately excludes the ABI cursors themselves.
// Keeping this view pointer-free lets the state-machine update live in safe
// code while the opaque-handle projection remains at the boundary.
struct GzResetState {
    mode: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    past: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
    reset: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    err: ::core::ffi::c_int,
    msg: Option<Box<[u8]>>,
    pos: crate::stdlib::off64_t,
    avail_in: crate::stdlib::uInt,
}

// This is the mutable, pointer-free portion of a gzip reset projection.  It
// deliberately contains only scalar fields and owned error storage, so the
// reset transition can be shared by the temporary ABI state and a future
// owned gzip handle without retaining a raw state reference.
struct GzResetTarget<'a> {
    have: &'a mut ::core::ffi::c_uint,
    eof: &'a mut ::core::ffi::c_int,
    past: &'a mut ::core::ffi::c_int,
    how: &'a mut ::core::ffi::c_int,
    junk: &'a mut ::core::ffi::c_int,
    reset: &'a mut ::core::ffi::c_int,
    again: &'a mut ::core::ffi::c_int,
    skip: &'a mut crate::stdlib::off64_t,
    err: &'a mut ::core::ffi::c_int,
    msg: &'a mut Option<Box<[u8]>>,
    pos: &'a mut crate::stdlib::off64_t,
    avail_in: &'a mut crate::stdlib::uInt,
}

impl GzResetState {
    fn apply_reset(&mut self) {
        let fields = gz_reset_fields(self.mode);
        self.have = fields.have;
        if let Some(read) = fields.read {
            self.eof = read.eof;
            self.past = read.past;
            self.how = read.how;
            self.junk = read.junk;
        }
        if let Some(reset) = fields.reset {
            self.reset = reset;
        }
        self.again = fields.again;
        self.skip = fields.skip;
        gz_clear_error(&mut self.msg, &mut self.err);
        self.pos = fields.pos;
        self.avail_in = fields.avail_in;
    }
}

fn reset_gz_target(mode: ::core::ffi::c_int, target: GzResetTarget<'_>) {
    let reset = gz_reset(GzResetState {
        mode,
        have: *target.have,
        eof: *target.eof,
        past: *target.past,
        how: *target.how,
        junk: *target.junk,
        reset: *target.reset,
        again: *target.again,
        skip: *target.skip,
        err: *target.err,
        msg: target.msg.take(),
        pos: *target.pos,
        avail_in: *target.avail_in,
    });
    *target.have = reset.have;
    *target.eof = reset.eof;
    *target.past = reset.past;
    *target.how = reset.how;
    *target.junk = reset.junk;
    *target.reset = reset.reset;
    *target.again = reset.again;
    *target.skip = reset.skip;
    *target.err = reset.err;
    *target.msg = reset.msg;
    *target.pos = reset.pos;
    *target.avail_in = reset.avail_in;
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

struct GzOpenMode {
    mode: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
}

fn parse_gz_open_mode(mode: &[u8]) -> Option<GzOpenMode> {
    let mut parsed = GzOpenMode {
        mode: crate::gzguts_h::GZ_NONE,
        level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
        strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
        direct: 0,
        oflag: 0,
        exclusive: 0,
    };
    for &option in mode {
        if option.is_ascii_digit() {
            parsed.level = (option - b'0') as ::core::ffi::c_int;
            continue;
        }
        match option {
            b'r' => parsed.mode = crate::gzguts_h::GZ_READ,
            b'w' => parsed.mode = crate::gzguts_h::GZ_WRITE,
            b'a' => parsed.mode = crate::gzguts_h::GZ_APPEND,
            b'+' => return None,
            b'e' => parsed.oflag |= crate::stdlib::O_CLOEXEC,
            b'x' => parsed.exclusive = 1,
            b'f' => parsed.strategy = crate::zlib_h::Z_FILTERED,
            b'h' => parsed.strategy = crate::zlib_h::Z_HUFFMAN_ONLY,
            b'R' => parsed.strategy = crate::zlib_h::Z_RLE,
            b'F' => parsed.strategy = crate::zlib_h::Z_FIXED,
            b'G' => parsed.direct = -1,
            b'N' => parsed.oflag |= crate::stdlib::O_NONBLOCK,
            b'T' => parsed.direct = 1,
            _ => {}
        }
    }
    Some(parsed)
}

fn gz_reset(mut reset: GzResetState) -> GzResetState {
    reset.apply_reset();
    reset
}

unsafe fn gz_open(path: &[u8], fd: ::core::ffi::c_int, mode: &[u8]) -> crate::zlib_h::gzFile {
    // The gzip handle is opaque at the ABI.  Keep its allocation owned until
    // the handle is successfully returned, rather than using malloc/free for
    // the state record itself.
    let mut state_owner = Vec::new();
    if state_owner.try_reserve_exact(1).is_err() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    state_owner.push(crate::gzguts_h::gz_state {
        x: crate::zlib_h::gzFile_s {
            have: 0,
            next: ::core::ptr::null_mut(),
            pos: 0,
        },
        mode: crate::gzguts_h::GZ_NONE,
        fd: None,
        path: None,
        size: 0,
        want: crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint,
        in_0: None,
        out: None,
        direct: 0,
        junk: 0,
        how: crate::gzguts_h::LOOK,
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
        strm: crate::zlib_h::z_stream {
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
    let Some(parsed_mode) = parse_gz_open_mode(mode) else {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    };
    let mut oflag = parsed_mode.oflag;
    let exclusive = parsed_mode.exclusive;
    let state_ref = state_owner
        .first_mut()
        .expect("gzip state owner contains its reserved state");
    state_ref.size = 0 as ::core::ffi::c_uint;
    state_ref.want = crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint;
    state_ref.err = crate::zlib_h::Z_OK;
    state_ref.mode = parsed_mode.mode;
    state_ref.level = parsed_mode.level;
    state_ref.strategy = parsed_mode.strategy;
    state_ref.direct = parsed_mode.direct;
    if state_ref.mode == crate::gzguts_h::GZ_NONE {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    if state_ref.mode == crate::gzguts_h::GZ_READ {
        if state_ref.direct == 1 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
        }
        if state_ref.direct == 0 as ::core::ffi::c_int {
            state_ref.direct = 1 as ::core::ffi::c_int;
        }
    } else if state_ref.direct == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let path_input = path;
    let mut path_bytes = Vec::new();
    if path_bytes.try_reserve_exact(path_input.len()).is_err() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    path_bytes.extend_from_slice(path_input);
    state_ref.path = Some(path_bytes.into_boxed_slice());
    oflag |= crate::stdlib::O_LARGEFILE
        | (if state_ref.mode == crate::gzguts_h::GZ_READ {
            crate::stdlib::O_RDONLY
        } else {
            crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | (if exclusive != 0 {
                    crate::stdlib::O_EXCL
                } else {
                    0 as ::core::ffi::c_int
                })
                | (if state_ref.mode == crate::gzguts_h::GZ_WRITE {
                    crate::stdlib::O_TRUNC
                } else {
                    crate::stdlib::O_APPEND
                })
        });
    if fd == -1 as ::core::ffi::c_int {
        match rustix::fs::open(
            path,
            rustix::fs::OFlags::from_bits_retain(oflag as u32),
            rustix::fs::Mode::from_raw_mode(0o666),
        ) {
            Ok(opened) => state_ref.fd = Some(opened),
            Err(error) => errno::set_errno(errno::Errno(error.raw_os_error())),
        }
    } else {
        state_ref.fd = Some(<rustix::fd::OwnedFd as rustix::fd::FromRawFd>::from_raw_fd(
            fd,
        ));
        if oflag & crate::stdlib::O_NONBLOCK != 0 {
            if let Ok(flags) = rustix::fs::fcntl_getfl(state_ref.fd.as_ref().unwrap()) {
                let _ = rustix::fs::fcntl_setfl(
                    state_ref.fd.as_ref().unwrap(),
                    flags | rustix::fs::OFlags::NONBLOCK,
                );
            }
        }
        if oflag & crate::stdlib::O_CLOEXEC != 0 {
            if let Ok(flags) = rustix::io::fcntl_getfd(state_ref.fd.as_ref().unwrap()) {
                let _ = rustix::io::fcntl_setfd(
                    state_ref.fd.as_ref().unwrap(),
                    flags | rustix::io::FdFlags::CLOEXEC,
                );
            }
        }
    }
    if state_ref.fd.is_none() {
        state_ref.path = None;
        state_ref.msg = None;
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    if state_ref.mode == crate::gzguts_h::GZ_APPEND {
        let _ = rustix::fs::seek(state_ref.fd.as_ref().unwrap(), rustix::fs::SeekFrom::End(0));
        state_ref.mode = crate::gzguts_h::GZ_WRITE;
    }
    if state_ref.mode == crate::gzguts_h::GZ_READ {
        state_ref.start = rustix::fs::tell(state_ref.fd.as_ref().unwrap())
            .map(|position| position as crate::stdlib::off64_t)
            .unwrap_or(0 as crate::stdlib::off64_t);
    }
    let state_ref = state_owner.first_mut().unwrap();
    let mode = state_ref.mode;
    reset_gz_target(
        mode,
        GzResetTarget {
            have: &mut state_ref.x.have,
            eof: &mut state_ref.eof,
            past: &mut state_ref.past,
            how: &mut state_ref.how,
            junk: &mut state_ref.junk,
            reset: &mut state_ref.reset,
            again: &mut state_ref.again,
            skip: &mut state_ref.skip,
            err: &mut state_ref.err,
            msg: &mut state_ref.msg,
            pos: &mut state_ref.x.pos,
            avail_in: &mut state_ref.strm.avail_in,
        },
    );
    let state = state_owner.as_mut_ptr();
    ::core::mem::forget(state_owner);
    return state as crate::zlib_h::gzFile;
}

#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    gz_open(
        ::core::ffi::CStr::from_ptr(path).to_bytes(),
        -1 as ::core::ffi::c_int,
        ::core::ffi::CStr::from_ptr(mode).to_bytes(),
    )
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    gz_open(
        ::core::ffi::CStr::from_ptr(path).to_bytes(),
        -1 as ::core::ffi::c_int,
        ::core::ffi::CStr::from_ptr(mode).to_bytes(),
    )
}
unsafe fn gzdopen(fd: ::core::ffi::c_int, mode: &[u8]) -> crate::zlib_h::gzFile {
    if fd == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    // This is the same bound used by the C implementation: enough for the
    // literal label, every decimal digit of a C int, its sign, and the NUL.
    let mut path = [0u8; 7 + 3 * ::core::mem::size_of::<::core::ffi::c_int>()];
    path[..4].copy_from_slice(b"<fd:");
    let mut at = 4usize;
    if fd < 0 as ::core::ffi::c_int {
        path[at] = b'-';
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
        path[at] = b'0' + digits[count];
        at += 1;
    }
    path[at] = b'>';
    gz_open(&path[..at + 1], fd, mode)
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if fd == -1 as ::core::ffi::c_int || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    gzdopen(fd, ::core::ffi::CStr::from_ptr(mode).to_bytes())
}
fn gzbuffer(
    mode: ::core::ffi::c_int,
    current_size: ::core::ffi::c_uint,
    want: &mut ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    match gzbuffer_want(mode, current_size, size) {
        Some(requested_want) => {
            *want = requested_want;
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
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzbuffer(state.mode, state.size, &mut state.want, size)
}
unsafe fn gzrewind(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ
        || state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR
    {
        return -1 as ::core::ffi::c_int;
    }
    if rustix::fs::seek(
        state.fd.as_ref().unwrap(),
        rustix::fs::SeekFrom::Start(state.start as u64),
    )
    .is_err()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mode = state.mode;
    reset_gz_target(
        mode,
        GzResetTarget {
            have: &mut state.x.have,
            eof: &mut state.eof,
            past: &mut state.past,
            how: &mut state.how,
            junk: &mut state.junk,
            reset: &mut state.reset,
            again: &mut state.again,
            skip: &mut state.skip,
            err: &mut state.err,
            msg: &mut state.msg,
            pos: &mut state.x.pos,
            avail_in: &mut state.strm.avail_in,
        },
    );
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzrewind(state)
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
            if rustix::fs::seek(
                state.fd.as_ref().unwrap(),
                rustix::fs::SeekFrom::Current(seek_by as i64),
            )
            .is_err()
            {
                return -1 as crate::stdlib::off64_t;
            }
            state.x.have = 0 as ::core::ffi::c_uint;
            state.eof = 0 as ::core::ffi::c_int;
            state.past = 0 as ::core::ffi::c_int;
            state.skip = 0 as crate::stdlib::off64_t;
            gz_clear_error(&mut state.msg, &mut state.err);
            state.strm.avail_in = 0 as crate::stdlib::uInt;
            state.x.pos = position;
            return state.x.pos;
        }
        GzSeekAction::Rewind { offset } => {
            if gzrewind(state) == -1 as ::core::ffi::c_int {
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
        state.x.next = state.x.next.wrapping_add(n as usize);
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
fn gzoffset64(query: GzOffsetQuery<'_>) -> crate::stdlib::off64_t {
    if !query.position.active() {
        return -1 as crate::stdlib::off64_t;
    }
    let Some(fd) = query.fd else {
        return -1 as crate::stdlib::off64_t;
    };
    let Ok(offset) = rustix::fs::tell(fd) else {
        return -1 as crate::stdlib::off64_t;
    };
    let offset = offset as crate::stdlib::off64_t;
    query.position.offset(offset, query.buffered_input)
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as crate::stdlib::off64_t;
    };
    let state = state.as_ref();
    gzoffset64(GzOffsetQuery {
        position: GzPosition {
            mode: state.mode,
            pos: 0 as crate::stdlib::off64_t,
            past: 0 as ::core::ffi::c_int,
            skip: 0 as crate::stdlib::off64_t,
        },
        fd: state.fd.as_ref(),
        buffered_input: state.strm.avail_in,
    })
}
fn gzoffset(query: GzOffsetQuery<'_>) -> crate::stdlib::off_t {
    let ret = gzoffset64(query);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gzoffset"]

pub unsafe extern "C" fn gzoffset_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as crate::stdlib::off_t;
    };
    let state = state.as_ref();
    gzoffset(GzOffsetQuery {
        position: GzPosition {
            mode: state.mode,
            pos: 0 as crate::stdlib::off64_t,
            past: 0 as ::core::ffi::c_int,
            skip: 0 as crate::stdlib::off64_t,
        },
        fd: state.fd.as_ref(),
        buffered_input: state.strm.avail_in,
    })
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
    match gzerror(state.mode, state.err, state.msg.is_some(), errnum) {
        None => ::core::ptr::null::<::core::ffi::c_char>(),
        Some(GzErrorMessage::OutOfMemory) => b"out of memory\0".as_ptr().cast(),
        Some(GzErrorMessage::Empty) => b"\0".as_ptr().cast(),
        Some(GzErrorMessage::State) => state
            .msg
            .as_deref()
            .map_or(::core::ptr::null(), |msg| msg.as_ptr().cast()),
    }
}
fn gzclearerr(
    mode: ::core::ffi::c_int,
    eof: &mut ::core::ffi::c_int,
    past: &mut ::core::ffi::c_int,
    message: &mut Option<Box<[u8]>>,
    error: &mut ::core::ffi::c_int,
) {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return;
    }
    if mode == crate::gzguts_h::GZ_READ {
        *eof = 0 as ::core::ffi::c_int;
        *past = 0 as ::core::ffi::c_int;
    }
    gz_clear_error(message, error);
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(mut file: crate::zlib_h::gzFile) {
    let Some(mut state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return;
    };
    let state = state.as_mut();
    gzclearerr(
        state.mode,
        &mut state.eof,
        &mut state.past,
        &mut state.msg,
        &mut state.err,
    )
}
pub unsafe extern "C" fn gz_error(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    let state = &mut *state;
    let message = (!msg.is_null()).then(|| ::core::ffi::CStr::from_ptr(msg).to_bytes());
    gz_set_error(
        &mut state.msg,
        &mut state.err,
        &mut state.x.have,
        state.again,
        state.path.as_deref(),
        err,
        message,
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
