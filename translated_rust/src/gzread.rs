pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZIP;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::LOOK;
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

/// Decide whether a public read operation may proceed after its boundary
/// adapter has sampled the opaque gzip state.  A retryable descriptor error
/// remains admissible, matching zlib's nonblocking I/O behavior.
fn gzread_state_is_valid(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
) -> bool {
    mode == crate::gzguts_h::GZ_READ
        && (err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR || again != 0)
}

/// Move unconsumed compressed input back to the beginning of its owned
/// buffer before a refill.  The gzip adapter converts its ABI cursor to the
/// `next_index` boundary value; this core therefore needs no raw pointers or
/// overlapping libc copy.
fn gz_avail_retain_input(
    input: &mut [u8],
    next_index: usize,
    avail_in: crate::stdlib::uInt,
) -> Option<()> {
    let end = next_index.checked_add(avail_in as usize)?;
    if end > input.len() || avail_in as usize > input.len() {
        return None;
    }
    input.copy_within(next_index..end, 0);
    Some(())
}

/// Inspect the pending compressed-input prefix without dereferencing the ABI
/// stream cursor.  `start` is the boundary-reconciled index of that cursor.
fn gz_look_input_is_gzip(
    input: &[u8],
    start: usize,
    avail_in: crate::stdlib::uInt,
) -> Option<bool> {
    let end = start.checked_add(avail_in as usize)?;
    let pending = input.get(start..end)?;
    Some(
        pending.len() > 3
            && pending[0] == 31
            && pending[1] == 139
            && pending[2] == 8
            && pending[3] < 32,
    )
}

/// Copy pending compressed bytes into the owned direct-read output buffer.
/// The buffers cannot overlap, but using slices keeps that ownership fact
/// explicit and rejects a stale ABI cursor before copying.
fn gz_look_copy_pending_input(
    input: &[u8],
    start: usize,
    avail_in: crate::stdlib::uInt,
    output: &mut [u8],
) -> Option<()> {
    let end = start.checked_add(avail_in as usize)?;
    let pending = input.get(start..end)?;
    output.get_mut(..pending.len())?.copy_from_slice(pending);
    Some(())
}

/// Result of safely filling one caller-owned read buffer from the descriptor.
enum GzLoad {
    Loaded {
        have: ::core::ffi::c_uint,
        eof: bool,
        again: bool,
    },
    Error(::core::ffi::c_int),
}

/// Read through the gzip state's RAII descriptor into an ordinary slice.
/// This preserves bounded refill and nonblocking partial-progress behavior
/// without raw buffers, libc I/O, or errno access.
fn gz_load(file: &mut ::std::fs::File, buf: &mut [u8]) -> GzLoad {
    use std::io::Read;

    let max = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2).wrapping_add(1) as usize;
    let mut have = 0usize;
    loop {
        let get = (buf.len() - have).min(max);
        if get == 0 {
            return GzLoad::Loaded {
                have: have as ::core::ffi::c_uint,
                eof: false,
                again: false,
            };
        }
        let Some(end) = have.checked_add(get) else {
            return GzLoad::Error(0);
        };
        let Some(chunk) = buf.get_mut(have..end) else {
            return GzLoad::Error(0);
        };
        match file.read(chunk) {
            Ok(0) => {
                return GzLoad::Loaded {
                    have: have as ::core::ffi::c_uint,
                    eof: true,
                    again: false,
                };
            }
            Ok(read) => {
                have += read;
                if have == buf.len() {
                    return GzLoad::Loaded {
                        have: have as ::core::ffi::c_uint,
                        eof: false,
                        again: false,
                    };
                }
            }
            Err(error) => {
                let code = error.raw_os_error().unwrap_or(0);
                let again = code == crate::stdlib::EAGAIN || code == crate::stdlib::EWOULDBLOCK;
                if again && have != 0 {
                    return GzLoad::Loaded {
                        have: have as ::core::ffi::c_uint,
                        eof: false,
                        again: true,
                    };
                }
                return GzLoad::Error(code);
            }
        }
    }
}

/// Apply descriptor-read scalar progress to owned gzip state. The safe loader
/// never exposes raw state or C diagnostics.
fn gz_load_commit(
    state: &mut crate::gzguts_h::gz_state,
    result: GzLoad,
) -> Result<::core::ffi::c_uint, ()> {
    match result {
        GzLoad::Loaded { have, eof, again } => {
            state.again = again as ::core::ffi::c_int;
            if eof {
                state.eof = 1;
            }
            Ok(have)
        }
        GzLoad::Error(code) => {
            state.again = 0;
            crate::src::gzlib::gz_error_io(state, code);
            Err(())
        }
    }
}

fn gz_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if state.eof == 0 as ::core::ffi::c_int {
        {
            let Some(buffers) = state.buffers.as_mut() else {
                return -1;
            };
            if state.strm.avail_in != 0 {
                let input = buffers.input.as_mut_ptr();
                let Some(next_index) = (state.strm.next_in as usize)
                    .checked_sub(input as usize)
                    .filter(|index| *index <= buffers.input.len())
                else {
                    return -1;
                };
                if gz_avail_retain_input(&mut buffers.input, next_index, state.strm.avail_in)
                    .is_none()
                {
                    return -1;
                }
            }
        }
        let avail_in = state.strm.avail_in as usize;
        let result = {
            let (Some(file), Some(buffers)) = (state.file.as_mut(), state.buffers.as_mut()) else {
                return -1;
            };
            let Some(input) = buffers.input.get_mut(avail_in..state.size as usize) else {
                return -1;
            };
            gz_load(file, input)
        };
        let got = match gz_load_commit(state, result) {
            Ok(got) => got,
            Err(()) => return -1,
        };
        let input = match state.buffers.as_mut() {
            Some(buffers) => buffers.input.as_mut_ptr(),
            None => return -1,
        };
        state.strm.avail_in = state.strm.avail_in.wrapping_add(got);
        state.strm.next_in = input as *mut crate::stdlib::Bytef;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_look(state_ref: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state_ref.size == 0 as ::core::ffi::c_uint {
        let output_len = match (state_ref.want as usize).checked_mul(2) {
            Some(len) => len,
            None => {
                crate::src::gzlib::gz_error_static(
                    state_ref,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0",
                );
                return -1;
            }
        };
        let Some(buffers) =
            crate::gzguts_h::gz_buffers::new(state_ref.want as usize, Some(output_len))
        else {
            crate::src::gzlib::gz_error_static(
                state_ref,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1;
        };
        state_ref.buffers = Some(buffers);
        state_ref.size = state_ref.want;
        state_ref.strm.zalloc = None;
        state_ref.strm.zfree = None;
        state_ref.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        state_ref.strm.avail_in = 0 as crate::stdlib::uInt;
        state_ref.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit2_(
            &raw mut state_ref.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            state_ref.buffers = None;
            state_ref.size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_error_static(
                state_ref,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if state_ref.direct == -1 as ::core::ffi::c_int || state_ref.junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflateReset(&raw mut state_ref.strm);
        state_ref.how = crate::gzguts_h::GZIP;
        state_ref.junk = (state_ref.junk != -1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        state_ref.direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state_ref) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state_ref.strm.avail_in == 0 as crate::stdlib::uInt
        || state_ref.again != 0 && state_ref.strm.avail_in < 4 as crate::stdlib::uInt
    {
        return 0 as ::core::ffi::c_int;
    }
    let input_is_gzip = {
        let Some(buffers) = state_ref.buffers.as_ref() else {
            return -1;
        };
        let Some(next_index) = (state_ref.strm.next_in as usize)
            .checked_sub(buffers.input.as_ptr() as usize)
            .filter(|index| *index <= buffers.input.len())
        else {
            return -1;
        };
        let Some(is_gzip) =
            gz_look_input_is_gzip(&buffers.input, next_index, state_ref.strm.avail_in)
        else {
            return -1;
        };
        is_gzip
    };
    if input_is_gzip {
        crate::src::inflate::inflateReset(&raw mut state_ref.strm);
        state_ref.how = crate::gzguts_h::GZIP;
        state_ref.junk = 1 as ::core::ffi::c_int;
        state_ref.direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    let Some(buffers) = state_ref.buffers.as_mut() else {
        return -1;
    };
    let Some(next_index) = (state_ref.strm.next_in as usize)
        .checked_sub(buffers.input.as_ptr() as usize)
        .filter(|index| *index <= buffers.input.len())
    else {
        return -1;
    };
    let Some(output) = buffers.output.as_mut() else {
        return -1;
    };
    if gz_look_copy_pending_input(&buffers.input, next_index, state_ref.strm.avail_in, output)
        .is_none()
    {
        return -1;
    }
    state_ref.x.next = output.as_mut_ptr();
    state_ref.x.have = state_ref.strm.avail_in as ::core::ffi::c_uint;
    state_ref.strm.avail_in = 0 as crate::stdlib::uInt;
    state_ref.how = crate::gzguts_h::COPY;
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_decomp(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let had = state.strm.avail_out as ::core::ffi::c_uint;
    loop {
        if state.strm.avail_in == 0 as crate::stdlib::uInt
            && gz_avail(state) == -1 as ::core::ffi::c_int
        {
            ret = state.err;
            break;
        } else if state.strm.avail_in == 0 as crate::stdlib::uInt {
            if state.again == 0 {
                crate::src::gzlib::gz_error_static(
                    state,
                    crate::zlib_h::Z_BUF_ERROR,
                    b"unexpected end of file\0",
                );
            }
            break;
        } else {
            ret = crate::src::inflate::inflate(
                &mut state.strm as *mut crate::zlib_h::z_stream_s,
                crate::zlib_h::Z_NO_FLUSH,
            );
            if state.strm.avail_out < had {
                state.junk = 0 as ::core::ffi::c_int;
            }
            if ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT {
                crate::src::gzlib::gz_error_static(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt\0",
                );
                break;
            } else if ret == crate::zlib_h::Z_MEM_ERROR {
                crate::src::gzlib::gz_error_static(
                    state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0",
                );
                break;
            } else if ret == crate::zlib_h::Z_DATA_ERROR {
                if state.junk == 1 as ::core::ffi::c_int {
                    state.strm.avail_in = 0 as crate::stdlib::uInt;
                    state.eof = 1 as ::core::ffi::c_int;
                    state.how = crate::gzguts_h::LOOK;
                    ret = crate::zlib_h::Z_OK;
                    break;
                } else {
                    let message = if state.strm.msg.is_null() {
                        ::std::ffi::CStr::from_bytes_with_nul(b"compressed data error\0").ok()
                    } else {
                        Some(::std::ffi::CStr::from_ptr(
                            state.strm.msg as *const ::core::ffi::c_char,
                        ))
                    };
                    crate::src::gzlib::gz_error_update_state(
                        state,
                        crate::zlib_h::Z_DATA_ERROR,
                        message,
                    );
                    break;
                }
            } else if !(state.strm.avail_out != 0 && ret != crate::zlib_h::Z_STREAM_END) {
                break;
            }
        }
    }
    state.x.have =
        (had as crate::stdlib::uInt).wrapping_sub(state.strm.avail_out) as ::core::ffi::c_uint;
    // Callers establish `x.next` as the start of this output span before
    // entering the codec.  Keeping that origin avoids reconstructing it by
    // subtracting from the raw post-inflate cursor.
    if ret == crate::zlib_h::Z_STREAM_END {
        state.junk = 0 as ::core::ffi::c_int;
        state.how = crate::gzguts_h::LOOK;
        return 0 as ::core::ffi::c_int;
    }
    return if ret != crate::zlib_h::Z_OK {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}

unsafe fn gz_fetch(state_ref: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    loop {
        match state_ref.how {
            crate::gzguts_h::LOOK => {
                if gz_look(state_ref) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                if state_ref.how == crate::gzguts_h::LOOK {
                    return 0 as ::core::ffi::c_int;
                }
            }
            crate::gzguts_h::COPY => {
                let result = {
                    let Some(file) = state_ref.file.as_mut() else {
                        return -1;
                    };
                    let Some(output) = state_ref
                        .buffers
                        .as_mut()
                        .and_then(|buffers| buffers.output.as_mut())
                    else {
                        return -1;
                    };
                    gz_load(file, output)
                };
                let have = match gz_load_commit(state_ref, result) {
                    Ok(have) => have,
                    Err(()) => return -1,
                };
                let Some(output) = state_ref
                    .buffers
                    .as_mut()
                    .and_then(|buffers| buffers.output.as_mut())
                else {
                    return -1;
                };
                state_ref.x.have = have;
                state_ref.x.next = output.as_mut_ptr();
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                let Some(output) = state_ref
                    .buffers
                    .as_mut()
                    .and_then(|buffers| buffers.output.as_mut())
                else {
                    return -1;
                };
                state_ref.strm.avail_out =
                    (state_ref.size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                state_ref.strm.next_out = output.as_mut_ptr() as *mut crate::stdlib::Bytef;
                state_ref.x.next = state_ref.strm.next_out as *mut ::core::ffi::c_uchar;
                if gz_decomp(state_ref) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            _ => {
                crate::src::gzlib::gz_error_static(
                    state_ref,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"state corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if !(state_ref.x.have == 0 as ::core::ffi::c_uint
            && (state_ref.eof == 0 || state_ref.strm.avail_in != 0))
        {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

/// Decide how much already-buffered gzip output a pending forward seek can
/// consume.  Cursor reconstruction remains at the raw boundary, but this
/// keeps the size conversion and scalar arithmetic out of that adapter.
fn gz_skip_buffer_plan(
    have: crate::stdlib::uInt,
    skip: crate::stdlib::off64_t,
) -> Option<::core::ffi::c_uint> {
    if skip < 0 {
        return None;
    }
    Some(
        if (::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && have > crate::src::gzlib::gz_intmax())
            || have as crate::stdlib::off64_t > skip
        {
            skip as ::core::ffi::c_uint
        } else {
            have
        },
    )
}

/// Commit a preflighted buffered-seek consumption after the boundary has
/// advanced the raw output cursor.
fn gz_skip_buffer_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    consume: ::core::ffi::c_uint,
) -> bool {
    if consume > state.x.have || state.skip < consume as crate::stdlib::off64_t {
        return false;
    }
    state.x.have = state.x.have.wrapping_sub(consume);
    state.x.pos = state.x.pos.wrapping_add(consume as crate::stdlib::off64_t);
    state.skip = state.skip.wrapping_sub(consume as crate::stdlib::off64_t);
    true
}

/// Limit a direct copy from already-buffered gzip output to the amount that
/// is both requested and available.  The raw cursor is advanced separately at
/// the boundary only after this preflight succeeds.
fn gz_read_buffer_copy_plan(
    requested: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    requested.min(have)
}

/// Copy one buffered output prefix into the caller's current destination.
/// The adapter supplies both cursor positions as indices, so this core can
/// validate its ranges and use an ordinary slice copy instead of libc
/// `memcpy`.
fn gz_read_buffered_copy(
    destination: &mut [u8],
    buffered: &[u8],
    next_index: usize,
    requested: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    let copied = gz_read_buffer_copy_plan(requested, have) as usize;
    let end = next_index.checked_add(copied)?;
    destination
        .get_mut(..copied)?
        .copy_from_slice(buffered.get(next_index..end)?);
    Some(copied as ::core::ffi::c_uint)
}

/// Commit a preflighted direct buffered read after its raw cursor has moved.
/// The shared read loop records the logical position for every source path.
fn gz_read_buffer_copy_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    copied: ::core::ffi::c_uint,
) -> bool {
    if copied > state.x.have {
        return false;
    }
    state.x.have = state.x.have.wrapping_sub(copied);
    true
}

/// Commit the common read-loop byte accounting after the boundary has copied
/// or produced `copied` bytes and advanced its raw caller cursor.
fn gz_read_progress_state(
    remaining: crate::stdlib::z_size_t,
    got: crate::stdlib::z_size_t,
    pos: crate::stdlib::off64_t,
    copied: ::core::ffi::c_uint,
) -> (
    crate::stdlib::z_size_t,
    crate::stdlib::z_size_t,
    crate::stdlib::off64_t,
) {
    let copied = copied as crate::stdlib::z_size_t;
    (
        remaining.wrapping_sub(copied),
        got.wrapping_add(copied),
        pos.wrapping_add(copied as crate::stdlib::off64_t),
    )
}

/// Limit one `gzgets` copy to the caller's remaining space and, when one was
/// found in that range, include the newline byte.  The boundary still locates
/// that byte in its raw buffered output.
fn gzgets_buffer_copy_plan(
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    newline_offset: Option<::core::ffi::c_uint>,
) -> ::core::ffi::c_uint {
    let copy = have.min(left);
    match newline_offset {
        Some(offset) if offset < copy => offset + 1,
        _ => copy,
    }
}

/// Copy a bounded prefix of the owned read buffer into a caller-provided
/// line buffer.  Both cursor positions are indices supplied by the export
/// boundary, so this core can find a newline and copy bytes without C memory
/// routines or pointer arithmetic.
fn gzgets_buffered_copy(
    destination: &mut [u8],
    available: &[u8],
    have: crate::stdlib::uInt,
    left: ::core::ffi::c_uint,
) -> Option<(::core::ffi::c_uint, bool)> {
    let available = available.get(..have as usize)?;
    let limit = gzgets_buffer_copy_plan(have, left, None) as usize;
    let candidate = available.get(..limit)?;
    let copied = match candidate.iter().position(|byte| *byte == b'\n') {
        Some(offset) => offset.checked_add(1)?,
        None => candidate.len(),
    };
    destination
        .get_mut(..copied)?
        .copy_from_slice(candidate.get(..copied)?);
    Some((
        copied as ::core::ffi::c_uint,
        copied != 0 && candidate[copied - 1] == b'\n',
    ))
}

/// Commit a preflighted `gzgets` buffered copy after the boundary has copied
/// bytes and advanced its raw cursors.
fn gzgets_buffer_copy_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    left: ::core::ffi::c_uint,
    copied: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    if copied > state.x.have || copied > left {
        return None;
    }
    state.x.have = state.x.have.wrapping_sub(copied);
    state.x.pos = state.x.pos.wrapping_add(copied as crate::stdlib::off64_t);
    Some(left.wrapping_sub(copied))
}

/// Commit consumption of one buffered byte after the boundary has read it and
/// advanced the raw cursor.  This keeps `gzgetc`'s visible prefix accounting
/// checked and independent of the raw output pointer.
fn gzgetc_buffer_commit_state(state: &mut crate::gzguts_h::gz_state) -> bool {
    if state.x.have == 0 {
        return false;
    }
    state.x.have = state.x.have.wrapping_sub(1);
    state.x.pos = state.x.pos.wrapping_add(1);
    true
}

/// Calculate a `gzfread` byte request with the same wrapping multiplication
/// and overflow rejection as the C API.  This deliberately reports a zero
/// request separately from an invalid overflowing request.
fn gzfread_request_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    let len = nitems.wrapping_mul(size);
    if size == 0 || len.wrapping_div(size) == nitems {
        Some(len)
    } else {
        None
    }
}

/// Convert a completed `gz_read` byte count into complete `gzfread` items.
/// A zero byte request is a successful zero-item operation and never divides
/// by a zero item size.
fn gzfread_completed_items(
    request_len: crate::stdlib::z_size_t,
    size: crate::stdlib::z_size_t,
    read_len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if request_len == 0 {
        0
    } else {
        read_len.wrapping_div(size)
    }
}

unsafe fn gz_skip(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_uint = 0;
    loop {
        if state.x.have != 0 {
            let Some(consume) = gz_skip_buffer_plan(state.x.have, state.skip) else {
                return -1 as ::core::ffi::c_int;
            };
            n = consume;
            state.x.next = state.x.next.wrapping_add(n as usize);
            if !gz_skip_buffer_commit_state(state, n) {
                return -1 as ::core::ffi::c_int;
            }
        } else {
            if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_fetch(state) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if state.skip == 0 {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_read(
    state_ref: &mut crate::gzguts_h::gz_state,
    destination: &mut [u8],
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut len = destination.len() as crate::stdlib::z_size_t;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if state_ref.skip != 0 && gz_skip(state_ref) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    's_140: loop {
        n = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
        if n as crate::stdlib::z_size_t > len {
            n = len as ::core::ffi::c_uint;
        }
        's_28: {
            if state_ref.x.have != 0 {
                let Some(buffered) = state_ref
                    .buffers
                    .as_ref()
                    .and_then(|buffers| buffers.output.as_ref())
                else {
                    return got;
                };
                let Some(next_index) = (state_ref.x.next as usize)
                    .checked_sub(buffered.as_ptr() as usize)
                    .filter(|index| *index <= buffered.len())
                else {
                    return got;
                };
                let Some(destination) = destination
                    .get_mut(got..)
                    .and_then(|destination| destination.get_mut(..n as usize))
                else {
                    return got;
                };
                let Some(copied) =
                    gz_read_buffered_copy(destination, buffered, next_index, n, state_ref.x.have)
                else {
                    return got;
                };
                n = copied;
                state_ref.x.next = buffered.as_ptr().wrapping_add(next_index + n as usize)
                    as *mut ::core::ffi::c_uchar;
                if !gz_read_buffer_copy_commit_state(state_ref, n) {
                    return got;
                }
                if state_ref.err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
            } else {
                if state_ref.eof != 0 && state_ref.strm.avail_in == 0 as crate::stdlib::uInt {
                    break 's_140;
                }
                if state_ref.how == crate::gzguts_h::LOOK
                    || n < state_ref.size << 1 as ::core::ffi::c_int
                {
                    if gz_fetch(state_ref) == -1 as ::core::ffi::c_int
                        && state_ref.x.have == 0 as ::core::ffi::c_uint
                    {
                        err = -1 as ::core::ffi::c_int;
                    }
                    break 's_28;
                } else if state_ref.how == crate::gzguts_h::COPY {
                    let Some(destination) = destination
                        .get_mut(got..)
                        .and_then(|destination| destination.get_mut(..n as usize))
                    else {
                        return got;
                    };
                    let result = match state_ref.file.as_mut() {
                        Some(file) => gz_load(file, destination),
                        None => GzLoad::Error(0),
                    };
                    n = match gz_load_commit(state_ref, result) {
                        Ok(have) => have,
                        Err(()) => 0,
                    };
                    if state_ref.err != crate::zlib_h::Z_OK {
                        err = -1;
                    }
                } else {
                    let Some(destination) = destination
                        .get_mut(got..)
                        .and_then(|destination| destination.get_mut(..n as usize))
                    else {
                        return got;
                    };
                    state_ref.strm.avail_out = n as crate::stdlib::uInt;
                    state_ref.strm.next_out = destination.as_mut_ptr() as *mut crate::stdlib::Bytef;
                    state_ref.x.next = state_ref.strm.next_out as *mut ::core::ffi::c_uchar;
                    err = gz_decomp(state_ref);
                    n = state_ref.x.have;
                    state_ref.x.have = 0 as ::core::ffi::c_uint;
                }
            }
            (len, got, state_ref.x.pos) = gz_read_progress_state(len, got, state_ref.x.pos, n);
        }
        if !(len != 0 && err == 0) {
            break;
        }
    }
    if len != 0 && state_ref.eof != 0 {
        state_ref.past = 1 as ::core::ffi::c_int;
    }
    return got;
}
#[export_name = "gzread"]
pub unsafe extern "C" fn gzread_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzread_state_is_valid(state.mode, state.err, state.again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state);
    if !crate::src::gzlib::gz_request_len_fits_int(len) {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in an int\0",
        );
        return -1 as ::core::ffi::c_int;
    }
    if len != 0 && buf.is_null() {
        return -1;
    }
    let destination = if len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(buf as *mut u8, len as usize)
    };
    len = gz_read(state, destination) as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if state.again != 0 {
            let code = ::std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(0);
            crate::src::gzlib::gz_error_io(state, code);
            return -1 as ::core::ffi::c_int;
        }
    }
    return len as ::core::ffi::c_int;
}

#[export_name = "gzfread"]
pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let mut len: crate::stdlib::z_size_t = 0;
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzread_state_is_valid(state.mode, state.err, state.again) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error_clear(state);
    let Some(request_len) = gzfread_request_len(size, nitems) else {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0",
        );
        return 0 as crate::stdlib::z_size_t;
    };
    len = request_len;
    if len != 0 && buf.is_null() {
        return 0;
    }
    let destination = if len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(buf as *mut u8, len)
    };
    return gzfread_completed_items(len, size, gz_read(state, destination));
}
pub unsafe extern "C" fn gzgetc(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzread_state_is_valid(state.mode, state.err, state.again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state);
    if state.x.have != 0 {
        let c2rust_fresh2 = state.x.next;
        state.x.next = state.x.next.offset(1);
        if !gzgetc_buffer_commit_state(state) {
            return -1 as ::core::ffi::c_int;
        }
        return *c2rust_fresh2 as ::core::ffi::c_int;
    }
    return if gz_read(state, &mut buf) < 1 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    };
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc(file)
}
pub unsafe extern "C" fn gzgetc_(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    return gzgetc(file);
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc_(file)
}

/// Describe where an ungot byte belongs in the existing read buffer without
/// touching its ABI cursor.  The boundary converts that cursor to an index
/// before this safe core runs.
#[derive(Clone, Copy)]
enum GzUngetcBufferPlan {
    InvalidCharacter,
    Full,
    Empty {
        capacity: crate::stdlib::uInt,
    },
    Buffered {
        shift_to_end: bool,
        capacity: crate::stdlib::uInt,
    },
}

fn gzungetc_buffer_plan(
    c: ::core::ffi::c_int,
    have: crate::stdlib::uInt,
    size: crate::stdlib::uInt,
    next_is_output_start: bool,
) -> GzUngetcBufferPlan {
    if c < 0 {
        return GzUngetcBufferPlan::InvalidCharacter;
    }
    let capacity = size.wrapping_shl(1);
    if have == 0 {
        return GzUngetcBufferPlan::Empty { capacity };
    }
    if have >= capacity {
        return GzUngetcBufferPlan::Full;
    }
    GzUngetcBufferPlan::Buffered {
        shift_to_end: next_is_output_start,
        capacity,
    }
}

/// Commit only the scalar consequences of a successful ungetc buffer write.
/// `x.next` is deliberately left to the boundary, where it remains a raw ABI
/// cursor.
fn gzungetc_buffer_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    plan: GzUngetcBufferPlan,
) -> bool {
    let capacity = match plan {
        GzUngetcBufferPlan::InvalidCharacter | GzUngetcBufferPlan::Full => return false,
        GzUngetcBufferPlan::Empty { capacity } => {
            if state.x.have != 0 {
                return false;
            }
            capacity
        }
        GzUngetcBufferPlan::Buffered { capacity, .. } => {
            if state.x.have == 0 || state.x.have >= capacity {
                return false;
            }
            capacity
        }
    };
    if state.size.wrapping_shl(1) != capacity {
        return false;
    }
    state.x.have = state.x.have.wrapping_add(1);
    state.x.pos = state.x.pos.wrapping_sub(1);
    state.past = 0;
    true
}

/// Move the currently buffered output to the end of the output buffer before
/// inserting an ungot byte.  `copy_within` deliberately preserves the
/// overlap-safe backward copy performed by the original pointer loop.
fn gzungetc_shift_to_end(buffer: &mut [u8], have: crate::stdlib::uInt) -> Option<usize> {
    let have = have as usize;
    if have > buffer.len() {
        return None;
    }
    let start = buffer.len().checked_sub(have)?;
    buffer.copy_within(0..have, start);
    Some(start)
}

/// Insert an ungot byte into the owned output buffer.  The public prefix
/// cursor is represented by `next_index` for this operation, so this core
/// never needs to inspect or construct a raw pointer or opaque state.
fn gzungetc_buffer_insert(
    output: &mut [u8],
    c: ::core::ffi::c_int,
    next_index: usize,
    have: crate::stdlib::uInt,
    size: crate::stdlib::uInt,
) -> Result<(usize, GzUngetcBufferPlan), GzUngetcBufferPlan> {
    let plan = gzungetc_buffer_plan(c, have, size, next_index == 0);
    match plan {
        GzUngetcBufferPlan::InvalidCharacter | GzUngetcBufferPlan::Full => return Err(plan),
        GzUngetcBufferPlan::Empty { .. } | GzUngetcBufferPlan::Buffered { .. } => {}
    };
    if let GzUngetcBufferPlan::Empty { capacity } = plan {
        let Some(index) = (capacity as usize).checked_sub(1) else {
            return Err(plan);
        };
        let Some(slot) = output.get_mut(index..=index) else {
            return Err(plan);
        };
        slot.copy_from_slice(&[c as ::core::ffi::c_uchar]);
        return Ok((index, plan));
    }
    let GzUngetcBufferPlan::Buffered { shift_to_end, .. } = plan else {
        return Err(plan);
    };
    let cursor = if shift_to_end {
        let Some(start) = gzungetc_shift_to_end(output, have) else {
            return Err(plan);
        };
        start
    } else {
        next_index
    };
    let Some(index) = cursor.checked_sub(1) else {
        return Err(plan);
    };
    let Some(slot) = output.get_mut(index..=index) else {
        return Err(plan);
    };
    slot.copy_from_slice(&[c as ::core::ffi::c_uchar]);
    Ok((index, plan))
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1;
    }
    if state.how == crate::gzguts_h::LOOK && state.x.have == 0 {
        gz_look(state);
    }
    if !gzread_state_is_valid(state.mode, state.err, state.again) {
        return -1;
    }
    crate::src::gzlib::gz_error_clear(state);
    if state.skip != 0 && gz_skip(state) == -1 {
        return -1;
    }

    // `gzgetc` is allowed to advance the public prefix cursor directly, so
    // reconcile that ABI cursor with the owned output allocation here.
    let next_index = {
        let Some(output) = state
            .buffers
            .as_ref()
            .and_then(|buffers| buffers.output.as_ref())
        else {
            return -1;
        };
        let start = output.as_ptr() as usize;
        let Some(index) = (state.x.next as usize).checked_sub(start) else {
            return -1;
        };
        if index > output.len() {
            return -1;
        }
        index
    };
    let have = state.x.have;
    let size = state.size;
    let inserted = {
        let Some(output) = state
            .buffers
            .as_mut()
            .and_then(|buffers| buffers.output.as_mut())
        else {
            return -1;
        };
        gzungetc_buffer_insert(output, c, next_index, have, size)
    };
    let (next_index, plan) = match inserted {
        Ok(inserted) => inserted,
        Err(GzUngetcBufferPlan::Full) => {
            crate::src::gzlib::gz_error_static(
                state,
                crate::zlib_h::Z_DATA_ERROR,
                b"out of room to push characters\0",
            );
            return -1;
        }
        Err(_) => return -1,
    };
    if !gzungetc_buffer_commit_state(state, plan) {
        return -1;
    };
    let Some(output) = state
        .buffers
        .as_mut()
        .and_then(|buffers| buffers.output.as_mut())
    else {
        return -1;
    };
    state.x.next = output.as_mut_ptr().wrapping_add(next_index);
    c
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if file.is_null() || buf.is_null() || len < 1 {
        return ::core::ptr::null_mut();
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzread_state_is_valid(state.mode, state.err, state.again) {
        return ::core::ptr::null_mut();
    }
    let destination = ::core::slice::from_raw_parts_mut(buf as *mut u8, len as usize);
    crate::src::gzlib::gz_error_clear(state);
    if state.skip != 0 && gz_skip(state) == -1 {
        return ::core::ptr::null_mut();
    }
    let mut left = (destination.len() as ::core::ffi::c_uint).wrapping_sub(1);
    let mut written = 0usize;
    while left != 0 && !(state.x.have == 0 && gz_fetch(state) == -1) {
        if state.x.have == 0 {
            state.past = 1;
            break;
        }
        let available = ::core::slice::from_raw_parts(state.x.next, state.x.have as usize);
        let Some((copied, found_newline)) =
            gzgets_buffered_copy(&mut destination[written..], available, state.x.have, left)
        else {
            return ::core::ptr::null_mut();
        };
        state.x.next = state.x.next.offset(copied as isize);
        let Some(remaining) = gzgets_buffer_copy_commit_state(state, left, copied) else {
            return ::core::ptr::null_mut();
        };
        left = remaining;
        written = match written.checked_add(copied as usize) {
            Some(written) => written,
            None => return ::core::ptr::null_mut(),
        };
        if found_newline {
            break;
        }
    }
    if written == 0 {
        ::core::ptr::null_mut()
    } else {
        destination[written] = 0;
        buf
    }
}
/// Convert the direct-stream flag into the public `gzdirect` result after
/// the boundary has performed any required lazy lookahead.
fn gzdirect_state(direct: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (direct == 1) as ::core::ffi::c_int
}

/// Map the read-side saved stream status and descriptor-close result to the
/// public close status.  Resource release remains at the raw boundary.
pub(crate) fn gzclose_read_result(
    state_err: ::core::ffi::c_int,
    close_result: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if close_result != 0 {
        crate::zlib_h::Z_ERRNO
    } else if state_err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    }
}

#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::LOOK
        && state.x.have == 0
    {
        gz_look(state);
    }
    gzdirect_state(state.direct)
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    crate::src::gzclose::gzclose_read_at_boundary!(file)
}
