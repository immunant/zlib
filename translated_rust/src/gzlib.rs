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

fn gz_is_read_or_write_mode(mode: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_READ || mode == crate::gzguts_h::GZ_WRITE
}

fn gz_clear_read_flags(eof: &mut ::core::ffi::c_int, past: &mut ::core::ffi::c_int) {
    *eof = 0;
    *past = 0;
}

fn gzseek_fast_forward_reset(state: &mut crate::gzguts_h::gz_state) {
    state.x.have = 0;
    gz_clear_read_flags(&mut state.eof, &mut state.past);
    state.skip = 0;
}

fn gzclearerr_core(
    mode: ::core::ffi::c_int,
    eof: &mut ::core::ffi::c_int,
    past: &mut ::core::ffi::c_int,
) -> bool {
    if !gz_is_read_or_write_mode(mode) {
        return false;
    }
    if mode == crate::gzguts_h::GZ_READ {
        gz_clear_read_flags(eof, past);
    }
    true
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzResetFields {
    mode: ::core::ffi::c_int,
    have: crate::stdlib::uInt,
    eof: ::core::ffi::c_int,
    past: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
    reset: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    pos: crate::stdlib::off64_t,
    avail_in: crate::stdlib::uInt,
}

fn gz_reset_core(fields: &mut GzResetFields) {
    fields.have = 0;
    if fields.mode == crate::gzguts_h::GZ_READ {
        gz_clear_read_flags(&mut fields.eof, &mut fields.past);
        fields.how = crate::gzguts_h::LOOK;
        fields.junk = -1;
    } else {
        fields.reset = 0;
    }
    fields.again = 0;
    fields.skip = 0;
    fields.pos = 0;
    fields.avail_in = 0;
}

fn gz_reset_state(state: &mut crate::gzguts_h::gz_state) {
    let mut fields = GzResetFields {
        mode: state.mode,
        have: state.x.have,
        eof: state.eof,
        past: state.past,
        how: state.how,
        junk: state.junk,
        reset: state.reset,
        again: state.again,
        skip: state.skip,
        pos: state.x.pos,
        avail_in: state.strm.avail_in,
    };
    gz_reset_core(&mut fields);
    state.x.have = fields.have;
    state.eof = fields.eof;
    state.past = fields.past;
    state.how = fields.how;
    state.junk = fields.junk;
    state.reset = fields.reset;
    state.again = fields.again;
    state.skip = fields.skip;
    state.x.pos = fields.pos;
    state.strm.avail_in = fields.avail_in;
}

fn gz_open_defaults(state: &mut crate::gzguts_h::gz_state) {
    state.size = 0;
    state.want = crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint;
    state.err = crate::zlib_h::Z_OK;
    state.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    state.mode = crate::gzguts_h::GZ_NONE;
    state.level = crate::zlib_h::Z_DEFAULT_COMPRESSION;
    state.strategy = crate::zlib_h::Z_DEFAULT_STRATEGY;
    state.direct = 0;
}

fn gzseek_read_buffer_consumed(
    avail_in: crate::stdlib::uInt,
    offset: crate::stdlib::off64_t,
    int_and_off64_same_width: bool,
    int_max: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    if (int_and_off64_same_width && avail_in > int_max)
        || avail_in as crate::stdlib::off64_t > offset
    {
        offset as crate::stdlib::uInt
    } else {
        avail_in
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzSeekReadBufferPlan {
    consumed: crate::stdlib::uInt,
    remaining_offset: crate::stdlib::off64_t,
}

fn gzseek_plan_read_buffer_consumption(
    avail_in: crate::stdlib::uInt,
    offset: crate::stdlib::off64_t,
    int_and_off64_same_width: bool,
    int_max: crate::stdlib::uInt,
) -> GzSeekReadBufferPlan {
    let consumed = gzseek_read_buffer_consumed(avail_in, offset, int_and_off64_same_width, int_max);
    GzSeekReadBufferPlan {
        consumed,
        remaining_offset: offset - consumed as crate::stdlib::off64_t,
    }
}

fn gzseek_request_is_valid(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    whence: ::core::ffi::c_int,
) -> bool {
    gz_is_read_or_write_mode(mode)
        && gzseek_error_allows_positioning(err)
        && (whence == crate::stdlib::SEEK_SET || whence == crate::stdlib::SEEK_CUR)
}

fn gzrewind_request_is_valid(mode: ::core::ffi::c_int, err: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_READ && gzseek_error_allows_positioning(err)
}

fn gzseek_error_allows_positioning(err: ::core::ffi::c_int) -> bool {
    err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR
}

fn gzseek_adjust_offset(
    offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
    position: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    if whence == crate::stdlib::SEEK_SET {
        offset - position
    } else {
        offset
            + if past != 0 {
                0 as crate::stdlib::off64_t
            } else {
                skip
            }
    }
}

fn gzseek_can_fast_forward(
    mode: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    position: crate::stdlib::off64_t,
    offset: crate::stdlib::off64_t,
) -> bool {
    mode == crate::gzguts_h::GZ_READ
        && how == crate::gzguts_h::COPY
        && position + offset >= 0 as crate::stdlib::off64_t
}

fn gzseek_fast_forward_lseek_offset(
    offset: crate::stdlib::off64_t,
    buffered_input: crate::stdlib::uInt,
) -> crate::stdlib::off64_t {
    offset - buffered_input as crate::stdlib::off64_t
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzSeekOffsetPlan {
    offset: crate::stdlib::off64_t,
    rewind: bool,
}

fn gzseek_plan_remaining_offset(
    mode: ::core::ffi::c_int,
    position: crate::stdlib::off64_t,
    offset: crate::stdlib::off64_t,
) -> Option<GzSeekOffsetPlan> {
    if offset >= 0 {
        return Some(GzSeekOffsetPlan {
            offset,
            rewind: false,
        });
    }
    if mode != crate::gzguts_h::GZ_READ {
        return None;
    }

    let offset = offset + position;
    if offset < 0 {
        return None;
    }

    Some(GzSeekOffsetPlan {
        offset,
        rewind: true,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzOpenOptions {
    mode: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzOpenPlan {
    mode: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
}

fn gz_parse_open_mode(mode: &[u8]) -> Option<GzOpenOptions> {
    let mut options = GzOpenOptions {
        mode: crate::gzguts_h::GZ_NONE,
        level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
        strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
        direct: 0,
        oflag: 0,
        exclusive: 0,
    };

    for &option in mode {
        if option.is_ascii_digit() {
            options.level = (option - b'0') as ::core::ffi::c_int;
            continue;
        }

        match option {
            b'r' => options.mode = crate::gzguts_h::GZ_READ,
            b'w' => options.mode = crate::gzguts_h::GZ_WRITE,
            b'a' => options.mode = crate::gzguts_h::GZ_APPEND,
            b'+' => return None,
            b'e' => options.oflag |= crate::stdlib::O_CLOEXEC,
            b'x' => options.exclusive = 1,
            b'f' => options.strategy = crate::zlib_h::Z_FILTERED,
            b'h' => options.strategy = crate::zlib_h::Z_HUFFMAN_ONLY,
            b'R' => options.strategy = crate::zlib_h::Z_RLE,
            b'F' => options.strategy = crate::zlib_h::Z_FIXED,
            b'G' => options.direct = -1,
            b'N' => options.oflag |= crate::stdlib::O_NONBLOCK,
            b'T' => options.direct = 1,
            b'b' | _ => {}
        }
    }

    Some(options)
}

fn gz_prepare_open(options: GzOpenOptions) -> Option<GzOpenPlan> {
    if options.mode == crate::gzguts_h::GZ_NONE {
        return None;
    }

    let direct = if options.mode == crate::gzguts_h::GZ_READ {
        if options.direct == 1 {
            return None;
        }
        if options.direct == 0 {
            1
        } else {
            options.direct
        }
    } else {
        if options.direct == -1 {
            return None;
        }
        options.direct
    };
    let oflag = options.oflag
        | crate::stdlib::O_LARGEFILE
        | if options.mode == crate::gzguts_h::GZ_READ {
            crate::stdlib::O_RDONLY
        } else {
            crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | if options.exclusive != 0 {
                    crate::stdlib::O_EXCL
                } else {
                    0
                }
                | if options.mode == crate::gzguts_h::GZ_WRITE {
                    crate::stdlib::O_TRUNC
                } else {
                    crate::stdlib::O_APPEND
                }
        };

    Some(GzOpenPlan {
        mode: options.mode,
        level: options.level,
        strategy: options.strategy,
        direct,
        oflag,
    })
}

fn gz_apply_open_plan(state: &mut crate::gzguts_h::gz_state, plan: GzOpenPlan) {
    state.mode = plan.mode;
    state.level = plan.level;
    state.strategy = plan.strategy;
    state.direct = plan.direct;
}

fn gz_post_open_metadata(
    mode: ::core::ffi::c_int,
    current_offset: crate::stdlib::off64_t,
) -> (::core::ffi::c_int, Option<crate::stdlib::off64_t>) {
    if mode == crate::gzguts_h::GZ_APPEND {
        (crate::gzguts_h::GZ_WRITE, None)
    } else if mode == crate::gzguts_h::GZ_READ {
        (
            mode,
            Some(if current_offset == -1 {
                0
            } else {
                current_offset
            }),
        )
    } else {
        (mode, None)
    }
}

fn gz_apply_post_open_metadata(
    state: &mut crate::gzguts_h::gz_state,
    current_offset: crate::stdlib::off64_t,
) {
    let (mode, start) = gz_post_open_metadata(state.mode, current_offset);
    state.mode = mode;
    if let Some(start) = start {
        state.start = start;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzOpenOffsetPlan {
    whence: ::core::ffi::c_int,
    record_offset: bool,
}

fn gz_open_offset_plan(mode: ::core::ffi::c_int) -> Option<GzOpenOffsetPlan> {
    if mode == crate::gzguts_h::GZ_APPEND {
        Some(GzOpenOffsetPlan {
            whence: crate::stdlib::SEEK_END,
            record_offset: false,
        })
    } else if mode == crate::gzguts_h::GZ_READ {
        Some(GzOpenOffsetPlan {
            whence: crate::stdlib::SEEK_CUR,
            record_offset: true,
        })
    } else {
        None
    }
}

fn gz_open_recorded_offset(
    record_offset: bool,
    current_offset: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    if record_offset {
        current_offset
    } else {
        0
    }
}

fn gz_finish_open(state: &mut crate::gzguts_h::gz_state, current_offset: crate::stdlib::off64_t) {
    gz_apply_post_open_metadata(state, current_offset);
    gz_reset_state(state);
}

unsafe extern "C" fn gz_open(
    mut path: *const ::core::ffi::c_void,
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut len: crate::stdlib::z_size_t = 0;
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    state = crate::stdlib::malloc(
        ::core::mem::size_of::<crate::gzguts_h::gz_state>() as crate::__stddef_size_t_h::size_t
    ) as crate::gzguts_h::gz_statep;
    if state.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    gz_open_defaults(&mut *state);
    let plan = match gz_parse_open_mode(::core::ffi::CStr::from_ptr(mode).to_bytes())
        .and_then(gz_prepare_open)
    {
        Some(plan) => plan,
        None => {
            crate::stdlib::free(state as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
        }
    };
    gz_apply_open_plan(&mut *state, plan);
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
    if fd == -1 as ::core::ffi::c_int {
        (*state).fd = crate::stdlib::open(
            path as *const ::core::ffi::c_char,
            plan.oflag,
            0o666 as ::core::ffi::c_int,
        );
    } else {
        if plan.oflag & crate::stdlib::O_NONBLOCK != 0 {
            crate::stdlib::fcntl(
                fd,
                crate::stdlib::F_SETFL,
                crate::stdlib::fcntl(fd, crate::stdlib::F_GETFL) | crate::stdlib::O_NONBLOCK,
            );
        }
        if plan.oflag & crate::stdlib::O_CLOEXEC != 0 {
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
    let current_offset = match gz_open_offset_plan((*state).mode) {
        Some(plan) => {
            let offset =
                crate::stdlib::lseek64((*state).fd, 0 as crate::stdlib::__off64_t, plan.whence)
                    as crate::stdlib::off64_t;
            gz_open_recorded_offset(plan.record_offset, offset)
        }
        None => 0,
    };
    gz_finish_open(&mut *state, current_offset);
    gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
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
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()
                        as crate::__stddef_size_t_h::size_t),
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
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()
                    as crate::__stddef_size_t_h::size_t),
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
fn gzbuffer_normalized_want(size: ::core::ffi::c_uint) -> Option<::core::ffi::c_uint> {
    if (size << 1 as ::core::ffi::c_int) < size {
        None
    } else {
        Some(size.max(8 as ::core::ffi::c_uint))
    }
}

fn gzbuffer_core(
    state: &mut crate::gzguts_h::gz_state,
    size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if !gz_is_read_or_write_mode(state.mode) {
        return -1 as ::core::ffi::c_int;
    }
    if state.size != 0 as ::core::ffi::c_uint {
        return -1 as ::core::ffi::c_int;
    }
    let Some(size) = gzbuffer_normalized_want(size) else {
        return -1 as ::core::ffi::c_int;
    };
    state.want = size;
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    file: crate::zlib_h::gzFile,
    size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }

    gzbuffer_core(&mut *(file as crate::gzguts_h::gz_statep), size)
}
pub unsafe extern "C" fn gzrewind(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    {
        let state_ref = &mut *state;
        if !gzrewind_request_is_valid(state_ref.mode, state_ref.err) {
            return -1 as ::core::ffi::c_int;
        }
        if crate::stdlib::lseek64(
            state_ref.fd,
            state_ref.start as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_SET,
        ) == -1 as ::core::ffi::c_int as crate::stdlib::__off64_t
        {
            return -1 as ::core::ffi::c_int;
        }
        gz_reset_state(state_ref);
    }
    gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
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
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzseek_request_is_valid((*state).mode, (*state).err, whence) {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    offset = gzseek_adjust_offset(offset, whence, (*state).x.pos, (*state).past, (*state).skip);
    if whence == crate::stdlib::SEEK_CUR {
        (*state).skip = 0 as crate::stdlib::off64_t;
    }
    if gzseek_can_fast_forward((*state).mode, (*state).how, (*state).x.pos, offset) {
        ret = crate::stdlib::lseek64(
            (*state).fd,
            gzseek_fast_forward_lseek_offset(offset, (*state).x.have) as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t;
        if ret == -1 as ::core::ffi::c_int as crate::stdlib::off64_t {
            return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
        }
        gzseek_fast_forward_reset(&mut *state);
        gz_error(
            state,
            crate::zlib_h::Z_OK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        (*state).strm.avail_in = 0 as crate::stdlib::uInt;
        (*state).x.pos += offset;
        return (*state).x.pos;
    }
    let seek_plan = match gzseek_plan_remaining_offset((*state).mode, (*state).x.pos, offset) {
        Some(plan) => plan,
        None => return -1 as ::core::ffi::c_int as crate::stdlib::off64_t,
    };
    offset = seek_plan.offset;
    if seek_plan.rewind && gzrewind(file) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    if (*state).mode == crate::gzguts_h::GZ_READ {
        let read_buffer_plan = gzseek_plan_read_buffer_consumption(
            (*state).x.have,
            offset,
            ::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<crate::stdlib::off64_t>(),
            gz_intmax(),
        );
        n = read_buffer_plan.consumed;
        (*state).x.have = (*state).x.have.wrapping_sub(n);
        (*state).x.next = (*state).x.next.offset(n as isize);
        (*state).x.pos += n as crate::stdlib::off64_t;
        offset = read_buffer_plan.remaining_offset;
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
fn gz_legacy_offset_result(ret: crate::stdlib::off64_t) -> crate::stdlib::off_t {
    if ret == ret {
        ret
    } else {
        -1 as ::core::ffi::c_int as crate::stdlib::off_t
    }
}

pub unsafe extern "C" fn gzseek(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gzseek64(file, offset, whence);
    return gz_legacy_offset_result(ret);
}
#[export_name = "gzseek"]

pub unsafe extern "C" fn gzseek_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    gzseek(file, offset, whence)
}

fn gztell64_core(
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    pos + if past != 0 { 0 } else { skip }
}

pub unsafe extern "C" fn gztell64(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_is_read_or_write_mode((*state).mode) {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    gztell64_core((*state).x.pos, (*state).past, (*state).skip)
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    gztell64(file)
}
pub unsafe extern "C" fn gztell(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gztell64(file);
    return gz_legacy_offset_result(ret);
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    gztell(file)
}
fn gzoffset64_adjust_for_buffered_read(
    offset: crate::stdlib::off64_t,
    mode: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
) -> crate::stdlib::off64_t {
    if mode == crate::gzguts_h::GZ_READ {
        offset - avail_in as crate::stdlib::off64_t
    } else {
        offset
    }
}

pub unsafe extern "C" fn gzoffset64(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let mut offset: crate::stdlib::off64_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_is_read_or_write_mode((*state).mode) {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    offset = crate::stdlib::lseek64(
        (*state).fd,
        0 as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_CUR,
    ) as crate::stdlib::off64_t;
    if offset == -1 as ::core::ffi::c_int as crate::stdlib::off64_t {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    gzoffset64_adjust_for_buffered_read(offset, (*state).mode, (*state).strm.avail_in)
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    gzoffset64(file)
}
pub unsafe extern "C" fn gzoffset(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gzoffset64(file);
    return gz_legacy_offset_result(ret);
}
#[export_name = "gzoffset"]

pub unsafe extern "C" fn gzoffset_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    gzoffset(file)
}
fn gzeof_core(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode == crate::gzguts_h::GZ_READ {
        state.past
    } else {
        0 as ::core::ffi::c_int
    }
}

#[export_name = "gzeof"]
pub unsafe extern "C" fn gzeof_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }

    let state = &*(file as crate::gzguts_h::gz_statep);
    if !gz_is_read_or_write_mode(state.mode) {
        return 0 as ::core::ffi::c_int;
    }

    gzeof_core(state)
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzErrorMessage {
    Empty,
    OutOfMemory,
    Stored,
}

fn gzerror_core(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    has_message: bool,
) -> Option<GzErrorMessage> {
    if !gz_is_read_or_write_mode(mode) {
        return None;
    }

    if err == crate::zlib_h::Z_MEM_ERROR {
        Some(GzErrorMessage::OutOfMemory)
    } else if has_message {
        Some(GzErrorMessage::Stored)
    } else {
        Some(GzErrorMessage::Empty)
    }
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    file: crate::zlib_h::gzFile,
    errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if file.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }

    let state = &*(file as crate::gzguts_h::gz_statep);
    let Some(message) = gzerror_core(state.mode, state.err, !state.msg.is_null()) else {
        return ::core::ptr::null::<::core::ffi::c_char>();
    };

    if !errnum.is_null() {
        *errnum = state.err;
    }

    match message {
        GzErrorMessage::OutOfMemory => b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        GzErrorMessage::Empty => b"\0".as_ptr() as *const ::core::ffi::c_char,
        GzErrorMessage::Stored => state.msg as *const ::core::ffi::c_char,
    }
}
pub unsafe extern "C" fn gzclearerr(mut file: crate::zlib_h::gzFile) {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzclearerr_core((*state).mode, &mut (*state).eof, &mut (*state).past) {
        return;
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
pub fn gz_intmax() -> ::core::ffi::c_uint {
    crate::limits_h::INT_MAX as ::core::ffi::c_uint
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}

#[cfg(test)]
mod tests {
    use super::{
        gz_clear_read_flags, gz_is_read_or_write_mode, gz_legacy_offset_result,
        gz_open_offset_plan, gz_open_recorded_offset, gz_parse_open_mode, gz_post_open_metadata,
        gz_prepare_open, gz_reset_core, gzbuffer_normalized_want, gzclearerr_core, gzerror_core,
        gzoffset64_adjust_for_buffered_read, gzrewind_request_is_valid, gzseek_adjust_offset,
        gzseek_can_fast_forward, gzseek_error_allows_positioning, gzseek_fast_forward_lseek_offset,
        gzseek_fast_forward_reset, gzseek_plan_read_buffer_consumption,
        gzseek_plan_remaining_offset, gzseek_read_buffer_consumed, gzseek_request_is_valid,
        gztell64_core, GzErrorMessage, GzOpenOffsetPlan, GzResetFields, GzSeekOffsetPlan,
        GzSeekReadBufferPlan,
    };

    #[test]
    fn clearing_read_flags_resets_both_values() {
        let mut eof = 1;
        let mut past = 1;
        gz_clear_read_flags(&mut eof, &mut past);
        assert_eq!((eof, past), (0, 0));
    }

    #[test]
    fn gzseek_fast_forward_reset_clears_only_fast_forward_reset_fields() {
        let mut state = crate::gzguts_h::gz_state {
            x: crate::zlib_h::gzFile_s {
                have: 7,
                next: ::core::ptr::null_mut(),
                pos: 101,
            },
            mode: crate::gzguts_h::GZ_READ,
            fd: 0,
            path: ::core::ptr::null_mut(),
            size: 0,
            want: 0,
            in_0: ::core::ptr::null_mut(),
            out: ::core::ptr::null_mut(),
            direct: 0,
            junk: 0,
            how: crate::gzguts_h::COPY,
            again: 23,
            start: 0,
            eof: 1,
            past: 1,
            level: 0,
            strategy: 0,
            reset: 0,
            skip: 29,
            err: crate::zlib_h::Z_BUF_ERROR,
            msg: ::core::ptr::null_mut(),
            strm: crate::zlib_h::z_stream_s {
                next_in: ::core::ptr::null_mut(),
                avail_in: 31,
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

        gzseek_fast_forward_reset(&mut state);

        assert_eq!(state.x.have, 0);
        assert_eq!((state.eof, state.past, state.skip), (0, 0, 0));
        assert_eq!(state.x.pos, 101);
        assert_eq!(state.strm.avail_in, 31);
        assert_eq!(state.err, crate::zlib_h::Z_BUF_ERROR);
        assert_eq!(state.again, 23);
    }

    #[test]
    fn gz_reset_core_resets_read_state_without_touching_write_reset() {
        let mut fields = GzResetFields {
            mode: crate::gzguts_h::GZ_READ,
            have: 1,
            eof: 1,
            past: 1,
            how: crate::gzguts_h::COPY,
            junk: 0,
            reset: 1,
            again: 1,
            skip: 1,
            pos: 1,
            avail_in: 1,
        };

        gz_reset_core(&mut fields);

        assert_eq!(fields.have, 0);
        assert_eq!((fields.eof, fields.past), (0, 0));
        assert_eq!((fields.how, fields.junk), (crate::gzguts_h::LOOK, -1));
        assert_eq!(fields.reset, 1);
        assert_eq!((fields.again, fields.skip), (0, 0));
        assert_eq!((fields.pos, fields.avail_in), (0, 0));
    }

    #[test]
    fn gz_reset_core_resets_write_state_without_touching_read_fields() {
        let mut fields = GzResetFields {
            mode: crate::gzguts_h::GZ_WRITE,
            have: 1,
            eof: 1,
            past: 1,
            how: crate::gzguts_h::COPY,
            junk: 0,
            reset: 1,
            again: 1,
            skip: 1,
            pos: 1,
            avail_in: 1,
        };

        gz_reset_core(&mut fields);

        assert_eq!(fields.have, 0);
        assert_eq!((fields.eof, fields.past), (1, 1));
        assert_eq!((fields.how, fields.junk), (crate::gzguts_h::COPY, 0));
        assert_eq!(fields.reset, 0);
        assert_eq!((fields.again, fields.skip), (0, 0));
        assert_eq!((fields.pos, fields.avail_in), (0, 0));
    }

    #[test]
    fn gzclearerr_core_clears_read_flags_for_active_read_mode_only() {
        let mut eof = 1;
        let mut past = 1;
        assert!(gzclearerr_core(
            crate::gzguts_h::GZ_READ,
            &mut eof,
            &mut past
        ));
        assert_eq!((eof, past), (0, 0));

        eof = 1;
        past = 1;
        assert!(gzclearerr_core(
            crate::gzguts_h::GZ_WRITE,
            &mut eof,
            &mut past
        ));
        assert_eq!((eof, past), (1, 1));

        assert!(!gzclearerr_core(
            crate::gzguts_h::GZ_NONE,
            &mut eof,
            &mut past
        ));
    }

    #[test]
    fn read_or_write_mode_validation_accepts_active_modes_only() {
        assert!(gz_is_read_or_write_mode(crate::gzguts_h::GZ_READ));
        assert!(gz_is_read_or_write_mode(crate::gzguts_h::GZ_WRITE));
        assert!(!gz_is_read_or_write_mode(crate::gzguts_h::GZ_NONE));
        assert!(!gz_is_read_or_write_mode(crate::gzguts_h::GZ_APPEND));
    }

    #[test]
    fn gzbuffer_normalizes_small_requested_sizes() {
        assert_eq!(gzbuffer_normalized_want(0), Some(8));
        assert_eq!(gzbuffer_normalized_want(7), Some(8));
    }

    #[test]
    fn gzbuffer_preserves_valid_requested_sizes() {
        assert_eq!(gzbuffer_normalized_want(8), Some(8));
        assert_eq!(gzbuffer_normalized_want(9), Some(9));
    }

    #[test]
    fn gzbuffer_rejects_sizes_that_overflow_when_doubled() {
        assert_eq!(gzbuffer_normalized_want(::core::ffi::c_uint::MAX), None);
    }

    #[test]
    fn gzerror_core_rejects_invalid_modes() {
        assert_eq!(gzerror_core(0, 0, false), None);
    }

    #[test]
    fn gzerror_core_prioritizes_out_of_memory_message() {
        assert_eq!(
            gzerror_core(crate::gzguts_h::GZ_READ, crate::zlib_h::Z_MEM_ERROR, true),
            Some(GzErrorMessage::OutOfMemory)
        );
    }

    #[test]
    fn gzerror_core_selects_empty_or_stored_message() {
        assert_eq!(
            gzerror_core(crate::gzguts_h::GZ_WRITE, crate::zlib_h::Z_OK, false),
            Some(GzErrorMessage::Empty)
        );
        assert_eq!(
            gzerror_core(crate::gzguts_h::GZ_WRITE, crate::zlib_h::Z_OK, true),
            Some(GzErrorMessage::Stored)
        );
    }

    #[test]
    fn gztell64_core_includes_pending_skip_before_eof() {
        assert_eq!(gztell64_core(42, 0, 7), 49);
    }

    #[test]
    fn gztell64_core_ignores_skip_after_eof() {
        assert_eq!(gztell64_core(42, 1, 7), 42);
    }

    #[test]
    fn legacy_offset_result_preserves_signed_offsets() {
        assert_eq!(gz_legacy_offset_result(27), 27 as crate::stdlib::off_t);
        assert_eq!(gz_legacy_offset_result(-1), -1 as crate::stdlib::off_t);
    }

    #[test]
    fn gzoffset64_adjusts_for_unconsumed_read_input() {
        assert_eq!(
            gzoffset64_adjust_for_buffered_read(42, crate::gzguts_h::GZ_READ, 7),
            35
        );
    }

    #[test]
    fn gzoffset64_preserves_offset_outside_read_mode() {
        assert_eq!(
            gzoffset64_adjust_for_buffered_read(42, crate::gzguts_h::GZ_WRITE, 7),
            42
        );
    }

    #[test]
    fn gzseek_consumes_all_buffered_input_with_sufficient_offset() {
        assert_eq!(gzseek_read_buffer_consumed(7, 7, false, 0), 7);
        assert_eq!(gzseek_read_buffer_consumed(7, 9, false, 0), 7);
    }

    #[test]
    fn gzseek_consumes_only_requested_buffered_input() {
        assert_eq!(gzseek_read_buffer_consumed(7, 3, false, 0), 3);
        assert_eq!(gzseek_read_buffer_consumed(7, 0, false, 0), 0);
    }

    #[test]
    fn gzseek_preserves_matching_width_large_buffer_rule() {
        assert_eq!(gzseek_read_buffer_consumed(9, 20, true, 8), 20);
        assert_eq!(gzseek_read_buffer_consumed(8, 20, true, 8), 8);
    }

    #[test]
    fn gzseek_read_buffer_plan_preserves_consumed_bytes_and_remaining_offset() {
        assert_eq!(
            gzseek_plan_read_buffer_consumption(7, 3, false, 0),
            GzSeekReadBufferPlan {
                consumed: 3,
                remaining_offset: 0,
            }
        );
        assert_eq!(
            gzseek_plan_read_buffer_consumption(7, 9, false, 0),
            GzSeekReadBufferPlan {
                consumed: 7,
                remaining_offset: 2,
            }
        );
    }

    #[test]
    fn gzseek_read_buffer_plan_preserves_matching_width_large_buffer_quirk() {
        assert_eq!(
            gzseek_plan_read_buffer_consumption(9, 20, true, 8),
            GzSeekReadBufferPlan {
                consumed: 20,
                remaining_offset: 0,
            }
        );
    }

    #[test]
    fn gzseek_request_validation_requires_active_mode_recoverable_error_and_supported_whence() {
        assert!(gzseek_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_OK,
            crate::stdlib::SEEK_SET
        ));
        assert!(gzseek_request_is_valid(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_BUF_ERROR,
            crate::stdlib::SEEK_CUR
        ));
        assert!(!gzseek_request_is_valid(
            crate::gzguts_h::GZ_NONE,
            crate::zlib_h::Z_OK,
            crate::stdlib::SEEK_SET
        ));
        assert!(!gzseek_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_MEM_ERROR,
            crate::stdlib::SEEK_SET
        ));
        assert!(!gzseek_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_OK,
            crate::stdlib::SEEK_END
        ));
    }

    #[test]
    fn gzseek_positioning_errors_match_rewind_and_seek_requirements() {
        assert!(gzseek_error_allows_positioning(crate::zlib_h::Z_OK));
        assert!(gzseek_error_allows_positioning(crate::zlib_h::Z_BUF_ERROR));
        assert!(!gzseek_error_allows_positioning(crate::zlib_h::Z_MEM_ERROR));
    }

    #[test]
    fn gzrewind_request_validation_requires_read_mode_and_recoverable_error() {
        assert!(gzrewind_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_OK
        ));
        assert!(gzrewind_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_BUF_ERROR
        ));
        assert!(!gzrewind_request_is_valid(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_OK
        ));
        assert!(!gzrewind_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_MEM_ERROR
        ));
    }

    #[test]
    fn gzseek_remaining_offset_plan_keeps_nonnegative_offsets_in_place() {
        assert_eq!(
            gzseek_plan_remaining_offset(crate::gzguts_h::GZ_WRITE, 12, 5),
            Some(GzSeekOffsetPlan {
                offset: 5,
                rewind: false,
            })
        );
    }

    #[test]
    fn gzseek_remaining_offset_plan_rewinds_reads_for_valid_negative_targets() {
        assert_eq!(
            gzseek_plan_remaining_offset(crate::gzguts_h::GZ_READ, 12, -5),
            Some(GzSeekOffsetPlan {
                offset: 7,
                rewind: true,
            })
        );
    }

    #[test]
    fn gzseek_remaining_offset_plan_rejects_invalid_negative_targets() {
        assert_eq!(
            gzseek_plan_remaining_offset(crate::gzguts_h::GZ_WRITE, 12, -1),
            None
        );
        assert_eq!(
            gzseek_plan_remaining_offset(crate::gzguts_h::GZ_READ, 12, -13),
            None
        );
    }

    #[test]
    fn gzseek_adjusts_set_and_current_offsets_using_pending_skip_only_before_eof() {
        assert_eq!(
            gzseek_adjust_offset(30, crate::stdlib::SEEK_SET, 12, 0, 7),
            18
        );
        assert_eq!(
            gzseek_adjust_offset(30, crate::stdlib::SEEK_CUR, 12, 0, 7),
            37
        );
        assert_eq!(
            gzseek_adjust_offset(30, crate::stdlib::SEEK_CUR, 12, 1, 7),
            30
        );
    }

    #[test]
    fn gzseek_fast_forward_requires_read_copy_mode_and_nonnegative_target() {
        assert!(gzseek_can_fast_forward(
            crate::gzguts_h::GZ_READ,
            crate::gzguts_h::COPY,
            12,
            -12
        ));
        assert!(!gzseek_can_fast_forward(
            crate::gzguts_h::GZ_WRITE,
            crate::gzguts_h::COPY,
            12,
            0
        ));
        assert!(!gzseek_can_fast_forward(
            crate::gzguts_h::GZ_READ,
            crate::gzguts_h::LOOK,
            12,
            0
        ));
        assert!(!gzseek_can_fast_forward(
            crate::gzguts_h::GZ_READ,
            crate::gzguts_h::COPY,
            12,
            -13
        ));
    }

    #[test]
    fn gzseek_fast_forward_lseek_offset_accounts_for_buffered_input() {
        assert_eq!(gzseek_fast_forward_lseek_offset(19, 7), 12);
        assert_eq!(gzseek_fast_forward_lseek_offset(-3, 7), -10);
        assert_eq!(gzseek_fast_forward_lseek_offset(0, 0), 0);
    }

    #[test]
    fn parsing_open_mode_collects_mode_flags_and_compression_options() {
        let options = gz_parse_open_mode(b"w9exfNT").unwrap();

        assert_eq!(options.mode, crate::gzguts_h::GZ_WRITE);
        assert_eq!(options.level, 9);
        assert_eq!(options.strategy, crate::zlib_h::Z_FILTERED);
        assert_eq!(options.direct, 1);
        assert_eq!(
            options.oflag,
            crate::stdlib::O_CLOEXEC | crate::stdlib::O_NONBLOCK
        );
        assert_eq!(options.exclusive, 1);
    }

    #[test]
    fn parsing_open_mode_rejects_update_mode() {
        assert_eq!(gz_parse_open_mode(b"rb+"), None);
    }

    #[test]
    fn parsing_open_mode_uses_the_last_conflicting_option() {
        let options = gz_parse_open_mode(b"rawhRFGT2").unwrap();

        assert_eq!(options.mode, crate::gzguts_h::GZ_WRITE);
        assert_eq!(options.level, 2);
        assert_eq!(options.strategy, crate::zlib_h::Z_FIXED);
        assert_eq!(options.direct, 1);
    }

    #[test]
    fn preparing_read_open_normalizes_default_direct_mode_and_flags() {
        let plan = gz_prepare_open(gz_parse_open_mode(b"r").unwrap()).unwrap();

        assert_eq!(plan.mode, crate::gzguts_h::GZ_READ);
        assert_eq!(plan.direct, 1);
        assert_eq!(
            plan.oflag,
            crate::stdlib::O_LARGEFILE | crate::stdlib::O_RDONLY
        );
    }

    #[test]
    fn preparing_write_open_combines_descriptor_flags() {
        let plan = gz_prepare_open(gz_parse_open_mode(b"axNe").unwrap()).unwrap();

        assert_eq!(plan.mode, crate::gzguts_h::GZ_APPEND);
        assert_eq!(plan.direct, 0);
        assert_eq!(
            plan.oflag,
            crate::stdlib::O_LARGEFILE
                | crate::stdlib::O_NONBLOCK
                | crate::stdlib::O_CLOEXEC
                | crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | crate::stdlib::O_EXCL
                | crate::stdlib::O_APPEND
        );
    }

    #[test]
    fn preparing_open_rejects_missing_mode_and_invalid_direct_modes() {
        assert_eq!(gz_prepare_open(gz_parse_open_mode(b"9").unwrap()), None);
        assert_eq!(gz_prepare_open(gz_parse_open_mode(b"rT").unwrap()), None);
        assert_eq!(gz_prepare_open(gz_parse_open_mode(b"wG").unwrap()), None);
    }

    #[test]
    fn post_open_metadata_converts_append_without_setting_read_start() {
        assert_eq!(
            gz_post_open_metadata(crate::gzguts_h::GZ_APPEND, 91),
            (crate::gzguts_h::GZ_WRITE, None)
        );
    }

    #[test]
    fn post_open_metadata_uses_zero_when_read_offset_is_unavailable() {
        assert_eq!(
            gz_post_open_metadata(crate::gzguts_h::GZ_READ, -1),
            (crate::gzguts_h::GZ_READ, Some(0))
        );
        assert_eq!(
            gz_post_open_metadata(crate::gzguts_h::GZ_READ, 19),
            (crate::gzguts_h::GZ_READ, Some(19))
        );
    }
    #[test]
    fn gz_open_offset_plan_seeks_to_end_without_recording_append_offsets() {
        assert_eq!(
            gz_open_offset_plan(crate::gzguts_h::GZ_APPEND),
            Some(GzOpenOffsetPlan {
                whence: crate::stdlib::SEEK_END,
                record_offset: false,
            })
        );
    }

    #[test]
    fn gz_open_offset_plan_records_current_read_offset() {
        assert_eq!(
            gz_open_offset_plan(crate::gzguts_h::GZ_READ),
            Some(GzOpenOffsetPlan {
                whence: crate::stdlib::SEEK_CUR,
                record_offset: true,
            })
        );
    }

    #[test]
    fn gz_open_offset_plan_skips_seeking_for_write_mode() {
        assert_eq!(gz_open_offset_plan(crate::gzguts_h::GZ_WRITE), None);
    }

    #[test]
    fn gz_open_recorded_offset_keeps_only_requested_offsets() {
        assert_eq!(gz_open_recorded_offset(true, 37), 37);
        assert_eq!(gz_open_recorded_offset(false, 37), 0);
    }
}
