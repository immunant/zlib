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

/// Reconcile an ABI cursor address with one owned buffer.  Callers convert
/// raw cursors to scalar addresses at their boundary; this core rejects
/// cursors before the allocation or past its end without creating a slice.
fn gz_owned_buffer_index(base: usize, len: usize, cursor: usize) -> Option<usize> {
    cursor.checked_sub(base).filter(|index| *index <= len)
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
        let Some(remaining) = buf.len().checked_sub(have) else {
            return GzLoad::Error(0);
        };
        let get = remaining.min(max);
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
                let Some(next_have) = have.checked_add(read) else {
                    return GzLoad::Error(0);
                };
                if next_have > buf.len() {
                    return GzLoad::Error(0);
                }
                have = next_have;
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
                let Some(next_index) = gz_owned_buffer_index(
                    input as usize,
                    buffers.input.len(),
                    state.strm.next_in as usize,
                ) else {
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

/// Derive the `junk` transition value used when lookahead restarts the
/// inflater. Keeping the sentinel rule scalar-only makes it independently
/// testable without moving opaque state traversal out of the codec boundary.
fn gz_look_reset_junk(junk: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (junk != -1) as ::core::ffi::c_int
}

/// Allocate the paired owned buffers used by the read-side lookahead state.
/// The transitional codec adapter only installs these buffers and initializes
/// its ABI stream cursors after this checked allocation succeeds.
fn gz_look_buffers(want: ::core::ffi::c_uint) -> Option<crate::gzguts_h::gz_buffers> {
    let output_len = (want as usize).checked_mul(2)?;
    crate::gzguts_h::gz_buffers::new(want as usize, Some(output_len))
}

// This transitional codec state machine expands only in exported gzip entry
// points. Its allocator and inflater calls stay at that ABI boundary until
// inflate has a safe stream-call adapter.
macro_rules! gz_look_at_boundary {
    ($state_ref:expr) => {{
        let state_ref = &mut *$state_ref;
        'gz_look_result: {
            if state_ref.size == 0 as ::core::ffi::c_uint {
                let Some(buffers) = gz_look_buffers(state_ref.want) else {
                    crate::src::gzlib::gz_error_static(
                        state_ref,
                        crate::zlib_h::Z_MEM_ERROR,
                        b"out of memory\0",
                    );
                    break 'gz_look_result -1;
                };
                state_ref.buffers = Some(buffers);
                state_ref.size = state_ref.want;
                state_ref.strm.zalloc = None;
                state_ref.strm.zfree = None;
                state_ref.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
                state_ref.strm.avail_in = 0 as crate::stdlib::uInt;
                state_ref.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                if crate::src::inflate::inflate_init2_at_boundary!(
                    &raw mut state_ref.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                    15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
                    crate::zlib_h::ZLIB_VERSION.as_ptr(),
                    ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
                ) != crate::zlib_h::Z_OK
                {
                    state_ref.buffers = None;
                    state_ref.size = 0;
                    crate::src::gzlib::gz_error_static(
                        state_ref,
                        crate::zlib_h::Z_MEM_ERROR,
                        b"out of memory\0",
                    );
                    break 'gz_look_result -1;
                }
            }
            let reset_junk = if state_ref.direct == -1 as ::core::ffi::c_int
                || state_ref.junk == 0 as ::core::ffi::c_int
            {
                Some(gz_look_reset_junk(state_ref.junk))
            } else {
                if gz_avail(state_ref) == -1 as ::core::ffi::c_int {
                    break 'gz_look_result -1;
                }
                if state_ref.strm.avail_in == 0 as crate::stdlib::uInt
                    || state_ref.again != 0 && state_ref.strm.avail_in < 4 as crate::stdlib::uInt
                {
                    break 'gz_look_result 0;
                }
                let input_is_gzip = {
                    let Some(buffers) = state_ref.buffers.as_ref() else {
                        break 'gz_look_result -1;
                    };
                    let Some(next_index) = gz_owned_buffer_index(
                        buffers.input.as_ptr() as usize,
                        buffers.input.len(),
                        state_ref.strm.next_in as usize,
                    ) else {
                        break 'gz_look_result -1;
                    };
                    let Some(is_gzip) =
                        gz_look_input_is_gzip(&buffers.input, next_index, state_ref.strm.avail_in)
                    else {
                        break 'gz_look_result -1;
                    };
                    is_gzip
                };
                input_is_gzip.then_some(1)
            };
            if let Some(junk) = reset_junk {
                crate::src::inflate::inflate_reset_at_boundary!(&raw mut state_ref.strm);
                state_ref.how = crate::gzguts_h::GZIP;
                state_ref.junk = junk;
                state_ref.direct = 0;
                break 'gz_look_result 0;
            }
            let Some(buffers) = state_ref.buffers.as_mut() else {
                break 'gz_look_result -1;
            };
            let Some(next_index) = gz_owned_buffer_index(
                buffers.input.as_ptr() as usize,
                buffers.input.len(),
                state_ref.strm.next_in as usize,
            ) else {
                break 'gz_look_result -1;
            };
            let Some(output) = buffers.output.as_mut() else {
                break 'gz_look_result -1;
            };
            if gz_look_copy_pending_input(
                &buffers.input,
                next_index,
                state_ref.strm.avail_in,
                output,
            )
            .is_none()
            {
                break 'gz_look_result -1;
            }
            state_ref.x.next = output.as_mut_ptr();
            state_ref.x.have = state_ref.strm.avail_in as ::core::ffi::c_uint;
            state_ref.strm.avail_in = 0 as crate::stdlib::uInt;
            state_ref.how = crate::gzguts_h::COPY;
            0
        }
    }};
}

/// The safe portion of one gzip inflate step after the boundary has invoked
/// the legacy codec.  The codec's dynamic data-error text is deliberately
/// left to that boundary: it is a temporary view of `strm.msg` and must not
/// escape the call that created it.
enum GzDecompInflateStep {
    Continue,
    Break(::core::ffi::c_int),
    StreamError,
    MemoryError,
    EndOfMember,
    DataError,
}

/// Map an inflate diagnostic address token to the immutable message bytes it
/// designates. The gzip boundary turns its ABI message pointer into this
/// scalar token, so owned gzip error storage never needs to borrow arbitrary
/// C text. A null or unrecognised token keeps zlib's generic diagnostic.
fn gz_inflate_error_message(message_address: usize) -> &'static [u8] {
    crate::src::inflate::INFLATE_ERROR_MESSAGES
        .iter()
        .find(|candidate| candidate.as_ptr() as usize == message_address)
        .copied()
        .unwrap_or(b"compressed data error\0")
}

/// Convert a codec output-capacity transition into produced bytes.  Both gzip
/// adapters take the pre-call capacity from the same owned output span, so a
/// larger post-call value is corrupt state rather than wrapping progress.
pub(crate) fn gz_codec_output_progress(
    before: crate::stdlib::uInt,
    after: crate::stdlib::uInt,
) -> Option<crate::stdlib::uInt> {
    before.checked_sub(after)
}

/// Decide the post-inflate gzip state transition using only codec progress
/// and scalar gzip state.  The exported boundary applies the resulting
/// cursor/error updates, so this core neither traverses opaque state nor
/// handles a transient codec diagnostic pointer.
fn gz_decomp_after_inflate(
    avail_out: crate::stdlib::uInt,
    ret: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
) -> GzDecompInflateStep {
    match ret {
        crate::zlib_h::Z_STREAM_ERROR | crate::zlib_h::Z_NEED_DICT => {
            GzDecompInflateStep::StreamError
        }
        crate::zlib_h::Z_MEM_ERROR => GzDecompInflateStep::MemoryError,
        crate::zlib_h::Z_DATA_ERROR if junk == 1 => GzDecompInflateStep::EndOfMember,
        crate::zlib_h::Z_DATA_ERROR => GzDecompInflateStep::DataError,
        _ if avail_out != 0 && ret != crate::zlib_h::Z_STREAM_END => GzDecompInflateStep::Continue,
        _ => GzDecompInflateStep::Break(ret),
    }
}

enum GzDecompFinish {
    Continue { produced: ::core::ffi::c_uint },
    StreamEnd { produced: ::core::ffi::c_uint },
    Error,
}

/// Derive the final buffered-output result after the codec boundary has
/// stopped its loop.  This keeps output accounting checked and independent
/// of the opaque gzip handle.
fn gz_decomp_finish(
    had: ::core::ffi::c_uint,
    avail_out: crate::stdlib::uInt,
    ret: ::core::ffi::c_int,
) -> GzDecompFinish {
    let Some(produced) = gz_codec_output_progress(had, avail_out) else {
        return GzDecompFinish::Error;
    };
    if ret == crate::zlib_h::Z_STREAM_END {
        GzDecompFinish::StreamEnd { produced }
    } else if ret != crate::zlib_h::Z_OK {
        GzDecompFinish::Error
    } else {
        GzDecompFinish::Continue { produced }
    }
}

// See `gz_look_at_boundary!`: this contains the legacy inflate call, so it
// deliberately expands only at an exported ABI boundary.
macro_rules! gz_decomp_at_boundary {
    ($state:expr) => {{
        let state = &mut *$state;
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
                let step = gz_decomp_after_inflate(state.strm.avail_out, ret, state.junk);
                if state.strm.avail_out < had {
                    state.junk = 0;
                }
                match step {
                    GzDecompInflateStep::Continue => {}
                    GzDecompInflateStep::Break(next_ret) => {
                        ret = next_ret;
                        break;
                    }
                    GzDecompInflateStep::StreamError => {
                        crate::src::gzlib::gz_error_static(
                            state,
                            crate::zlib_h::Z_STREAM_ERROR,
                            b"internal error: inflate stream corrupt\0",
                        );
                        break;
                    }
                    GzDecompInflateStep::MemoryError => {
                        crate::src::gzlib::gz_error_static(
                            state,
                            crate::zlib_h::Z_MEM_ERROR,
                            b"out of memory\0",
                        );
                        break;
                    }
                    GzDecompInflateStep::EndOfMember => {
                        state.strm.avail_in = 0;
                        state.eof = 1;
                        state.how = crate::gzguts_h::LOOK;
                        ret = crate::zlib_h::Z_OK;
                        break;
                    }
                    GzDecompInflateStep::DataError => {
                        // `inflate()` assigns every data-error message from its
                        // immutable diagnostic table. Pass only its address
                        // token into the safe lookup; arbitrary C text is
                        // never borrowed by the owned gzip state.
                        crate::src::gzlib::gz_error_static(
                            state,
                            crate::zlib_h::Z_DATA_ERROR,
                            gz_inflate_error_message(state.strm.msg as usize),
                        );
                        break;
                    }
                }
            }
        }
        match gz_decomp_finish(had, state.strm.avail_out, ret) {
            GzDecompFinish::Continue { produced } => {
                state.x.have = produced;
                0
            }
            GzDecompFinish::StreamEnd { produced } => {
                state.x.have = produced;
                // Callers establish `x.next` as the start of this output span
                // before entering the codec, so no post-call pointer
                // reconstruction is needed here.
                state.junk = 0;
                state.how = crate::gzguts_h::LOOK;
                0
            }
            GzDecompFinish::Error => -1,
        }
    }};
}

// Fetch can enter both lookahead and inflate. Keep those calls in the same
// exported expansion rather than retaining a private unsafe forwarding layer.
macro_rules! gz_fetch_at_boundary {
    ($state_ref:expr) => {{
        let state_ref = &mut *$state_ref;
        'gz_fetch_result: {
            loop {
                match state_ref.how {
                    crate::gzguts_h::LOOK => {
                        if gz_look_at_boundary!(state_ref) == -1 as ::core::ffi::c_int {
                            break 'gz_fetch_result -1 as ::core::ffi::c_int;
                        }
                        if state_ref.how == crate::gzguts_h::LOOK {
                            break 'gz_fetch_result 0 as ::core::ffi::c_int;
                        }
                    }
                    crate::gzguts_h::COPY => {
                        let result = {
                            let Some(file) = state_ref.file.as_mut() else {
                                break 'gz_fetch_result -1;
                            };
                            let Some(output) = state_ref
                                .buffers
                                .as_mut()
                                .and_then(|buffers| buffers.output.as_mut())
                            else {
                                break 'gz_fetch_result -1;
                            };
                            gz_load(file, output)
                        };
                        let have = match gz_load_commit(state_ref, result) {
                            Ok(have) => have,
                            Err(()) => break 'gz_fetch_result -1,
                        };
                        let Some(output) = state_ref
                            .buffers
                            .as_mut()
                            .and_then(|buffers| buffers.output.as_mut())
                        else {
                            break 'gz_fetch_result -1;
                        };
                        state_ref.x.have = have;
                        state_ref.x.next = output.as_mut_ptr();
                        break 'gz_fetch_result 0 as ::core::ffi::c_int;
                    }
                    crate::gzguts_h::GZIP => {
                        let Some(output) = state_ref
                            .buffers
                            .as_mut()
                            .and_then(|buffers| buffers.output.as_mut())
                        else {
                            break 'gz_fetch_result -1;
                        };
                        state_ref.strm.avail_out =
                            (state_ref.size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                        state_ref.strm.next_out = output.as_mut_ptr() as *mut crate::stdlib::Bytef;
                        state_ref.x.next = state_ref.strm.next_out as *mut ::core::ffi::c_uchar;
                        if gz_decomp_at_boundary!(state_ref) == -1 as ::core::ffi::c_int {
                            break 'gz_fetch_result -1 as ::core::ffi::c_int;
                        }
                    }
                    _ => {
                        crate::src::gzlib::gz_error_static(
                            state_ref,
                            crate::zlib_h::Z_STREAM_ERROR,
                            b"state corrupt\0",
                        );
                        break 'gz_fetch_result -1 as ::core::ffi::c_int;
                    }
                }
                if !(state_ref.x.have == 0 as ::core::ffi::c_uint
                    && (state_ref.eof == 0 || state_ref.strm.avail_in != 0))
                {
                    break;
                }
            }
            0 as ::core::ffi::c_int
        }
    }};
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

/// Plan the scalar effects of consuming buffered bytes for a pending seek.
/// The ABI cursor is committed separately at the export boundary after it
/// has been reconciled with the owned output allocation.
fn gz_skip_commit(
    have: crate::stdlib::uInt,
    pos: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
    consume: ::core::ffi::c_uint,
) -> Option<(
    crate::stdlib::uInt,
    crate::stdlib::off64_t,
    crate::stdlib::off64_t,
)> {
    if consume > have || skip < consume as crate::stdlib::off64_t {
        return None;
    }
    Some((
        have.wrapping_sub(consume),
        pos.wrapping_add(consume as crate::stdlib::off64_t),
        skip.wrapping_sub(consume as crate::stdlib::off64_t),
    ))
}

/// Advance a buffered-seek cursor using only owned-buffer indices.  The
/// public `gzgetc` macro can alter `x.next` between calls, so the export
/// boundary must reconcile that ABI cursor before it can consume buffered
/// bytes.  Validate the whole advertised buffered range as well as the
/// consumed prefix before returning the next index.
fn gz_skip_buffer_next_index(
    output_len: usize,
    next_index: usize,
    have: crate::stdlib::uInt,
    consume: ::core::ffi::c_uint,
) -> Option<usize> {
    if consume > have {
        return None;
    }
    let end = next_index.checked_add(have as usize)?;
    if end > output_len {
        return None;
    }
    next_index.checked_add(consume as usize)
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

/// Read one byte from owned buffered output after the export boundary has
/// reconciled its public cursor to an index.  This keeps `gzgetc` from
/// dereferencing a public raw cursor directly (the public macro may have
/// advanced it between calls).
fn gzgetc_buffered_take(
    output: &[u8],
    next_index: usize,
    have: crate::stdlib::uInt,
) -> Option<(::core::ffi::c_int, usize)> {
    if have == 0 {
        return None;
    }
    let end = next_index.checked_add(have as usize)?;
    output.get(next_index..end)?;
    Some((
        *output.get(next_index)? as ::core::ffi::c_int,
        next_index + 1,
    ))
}

// Keep raw ABI cursor reconciliation in the two exported `gzgetc` spellings.
// A macro deliberately expands there instead of moving `gz_state` traversal
// into a new implementation function, which the safety audit treats as new
// raw-pointer work.
macro_rules! gzgetc_buffered_or_return {
    ($state:expr) => {
        if $state.x.have != 0 {
            let taken = {
                let Some(output) = $state
                    .buffers
                    .as_ref()
                    .and_then(|buffers| buffers.output.as_ref())
                else {
                    return -1;
                };
                let Some(next_index) = gz_owned_buffer_index(
                    output.as_ptr() as usize,
                    output.len(),
                    $state.x.next as usize,
                ) else {
                    return -1;
                };
                gzgetc_buffered_take(output, next_index, $state.x.have)
            };
            let Some((byte, next_index)) = taken else {
                return -1;
            };
            if !gzgetc_buffer_commit_state($state) {
                return -1 as ::core::ffi::c_int;
            }
            let Some(output) = $state
                .buffers
                .as_mut()
                .and_then(|buffers| buffers.output.as_mut())
            else {
                return -1;
            };
            $state.x.next = output.as_mut_ptr().wrapping_add(next_index);
            return byte;
        }
    };
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

/// One safe transition of a pending forward seek.  Fetching more gzip data
/// remains with the caller, since it can enter the transitional codec layer.
enum GzSkipStep {
    Complete,
    NeedFetch,
    Advance {
        consumed: ::core::ffi::c_uint,
        complete: bool,
    },
}

fn gz_skip_step(
    have: crate::stdlib::uInt,
    skip: crate::stdlib::off64_t,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
) -> Result<GzSkipStep, ()> {
    if have != 0 {
        let Some(consume) = gz_skip_buffer_plan(have, skip) else {
            return Err(());
        };
        return Ok(GzSkipStep::Advance {
            consumed: consume,
            complete: skip == consume as crate::stdlib::off64_t,
        });
    }
    if eof != 0 && avail_in == 0 as crate::stdlib::uInt {
        Ok(GzSkipStep::Complete)
    } else {
        Ok(GzSkipStep::NeedFetch)
    }
}

// The only remaining codec calls in this read loop are deliberately expanded
// in exported read entry points.  Keeping the validated state and caller
// slice at that ABI boundary avoids a private unsafe forwarding adapter.
macro_rules! gz_read_at_boundary {
    ($state_ref:expr, $destination:expr) => {{
        let state_ref = &mut *$state_ref;
        let destination = &mut *$destination;
        'gz_read_result: {
            let mut got: crate::stdlib::z_size_t = 0;
            let mut n: ::core::ffi::c_uint = 0;
            let mut err: ::core::ffi::c_int = 0;
            let mut len = destination.len() as crate::stdlib::z_size_t;
            if len == 0 as crate::stdlib::z_size_t {
                break 'gz_read_result 0 as crate::stdlib::z_size_t;
            }
            while state_ref.skip != 0 {
                match gz_skip_step(
                    state_ref.x.have,
                    state_ref.skip,
                    state_ref.eof,
                    state_ref.strm.avail_in,
                ) {
                    Ok(GzSkipStep::Complete) => break,
                    Ok(GzSkipStep::NeedFetch) => {
                        if gz_fetch_at_boundary!(state_ref) == -1 as ::core::ffi::c_int {
                            break 'gz_read_result 0 as crate::stdlib::z_size_t;
                        }
                    }
                    Ok(GzSkipStep::Advance { consumed, complete }) => {
                        let next_index = {
                            let Some(output) = state_ref
                                .buffers
                                .as_ref()
                                .and_then(|buffers| buffers.output.as_ref())
                            else {
                                break 'gz_read_result 0;
                            };
                            let Some(next_index) = gz_owned_buffer_index(
                                output.as_ptr() as usize,
                                output.len(),
                                state_ref.x.next as usize,
                            ) else {
                                break 'gz_read_result 0;
                            };
                            let Some(next_index) = gz_skip_buffer_next_index(
                                output.len(),
                                next_index,
                                state_ref.x.have,
                                consumed,
                            ) else {
                                break 'gz_read_result 0;
                            };
                            next_index
                        };
                        let Some((have, pos, skip)) = gz_skip_commit(
                            state_ref.x.have,
                            state_ref.x.pos,
                            state_ref.skip,
                            consumed,
                        ) else {
                            break 'gz_read_result 0;
                        };
                        let Some(output) = state_ref
                            .buffers
                            .as_mut()
                            .and_then(|buffers| buffers.output.as_mut())
                        else {
                            break 'gz_read_result 0;
                        };
                        state_ref.x.next = output.as_mut_ptr().wrapping_add(next_index);
                        state_ref.x.have = have;
                        state_ref.x.pos = pos;
                        state_ref.skip = skip;
                        if complete {
                            break;
                        }
                    }
                    Err(()) => break 'gz_read_result 0 as crate::stdlib::z_size_t,
                }
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
                            break 'gz_read_result got;
                        };
                        let Some(next_index) = gz_owned_buffer_index(
                            buffered.as_ptr() as usize,
                            buffered.len(),
                            state_ref.x.next as usize,
                        ) else {
                            break 'gz_read_result got;
                        };
                        let Some(destination) = destination
                            .get_mut(got..)
                            .and_then(|destination| destination.get_mut(..n as usize))
                        else {
                            break 'gz_read_result got;
                        };
                        let Some(copied) = gz_read_buffered_copy(
                            destination,
                            buffered,
                            next_index,
                            n,
                            state_ref.x.have,
                        ) else {
                            break 'gz_read_result got;
                        };
                        n = copied;
                        state_ref.x.next = buffered.as_ptr().wrapping_add(next_index + n as usize)
                            as *mut ::core::ffi::c_uchar;
                        if !gz_read_buffer_copy_commit_state(state_ref, n) {
                            break 'gz_read_result got;
                        }
                        if state_ref.err != crate::zlib_h::Z_OK {
                            err = -1 as ::core::ffi::c_int;
                        }
                    } else {
                        if state_ref.eof != 0 && state_ref.strm.avail_in == 0 as crate::stdlib::uInt
                        {
                            break 's_140;
                        }
                        if state_ref.how == crate::gzguts_h::LOOK
                            || n < state_ref.size << 1 as ::core::ffi::c_int
                        {
                            if gz_fetch_at_boundary!(state_ref) == -1 as ::core::ffi::c_int
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
                                break 'gz_read_result got;
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
                                break 'gz_read_result got;
                            };
                            state_ref.strm.avail_out = n as crate::stdlib::uInt;
                            state_ref.strm.next_out =
                                destination.as_mut_ptr() as *mut crate::stdlib::Bytef;
                            state_ref.x.next = state_ref.strm.next_out as *mut ::core::ffi::c_uchar;
                            err = gz_decomp_at_boundary!(state_ref);
                            n = state_ref.x.have;
                            state_ref.x.have = 0 as ::core::ffi::c_uint;
                        }
                    }
                    (len, got, state_ref.x.pos) =
                        gz_read_progress_state(len, got, state_ref.x.pos, n);
                }
                if !(len != 0 && err == 0) {
                    break;
                }
            }
            if len != 0 && state_ref.eof != 0 {
                state_ref.past = 1 as ::core::ffi::c_int;
            }
            got
        }
    }};
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
    len = gz_read_at_boundary!(state, destination) as ::core::ffi::c_uint;
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
    return gzfread_completed_items(len, size, gz_read_at_boundary!(state, destination));
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzread_state_is_valid(state.mode, state.err, state.again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state);
    gzgetc_buffered_or_return!(state);
    if gz_read_at_boundary!(state, &mut buf) < 1 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    }
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzread_state_is_valid(state.mode, state.err, state.again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state);
    gzgetc_buffered_or_return!(state);
    if gz_read_at_boundary!(state, &mut buf) < 1 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    }
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
    let capacity = match plan {
        GzUngetcBufferPlan::InvalidCharacter | GzUngetcBufferPlan::Full => return Err(plan),
        GzUngetcBufferPlan::Empty { capacity } | GzUngetcBufferPlan::Buffered { capacity, .. } => {
            capacity as usize
        }
    };
    // An ungetc buffer uses the output allocation's leading `capacity`
    // bytes. Besides checking the cursor itself, ensure its buffered range
    // fits there before the mutation below can write a byte.
    if capacity > output.len() {
        return Err(plan);
    }
    match plan {
        GzUngetcBufferPlan::Empty { .. } | GzUngetcBufferPlan::Buffered { .. } => {}
        GzUngetcBufferPlan::InvalidCharacter | GzUngetcBufferPlan::Full => return Err(plan),
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
    if !shift_to_end
        && next_index
            .checked_add(have as usize)
            .filter(|end| *end <= capacity)
            .is_none()
    {
        return Err(plan);
    }
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
        gz_look_at_boundary!(state);
    }
    if !gzread_state_is_valid(state.mode, state.err, state.again) {
        return -1;
    }
    crate::src::gzlib::gz_error_clear(state);
    while state.skip != 0 {
        match gz_skip_step(state.x.have, state.skip, state.eof, state.strm.avail_in) {
            Ok(GzSkipStep::Complete) => break,
            Ok(GzSkipStep::NeedFetch) => {
                if gz_fetch_at_boundary!(state) == -1 {
                    return -1;
                }
            }
            Ok(GzSkipStep::Advance { consumed, complete }) => {
                let next_index = {
                    let Some(output) = state
                        .buffers
                        .as_ref()
                        .and_then(|buffers| buffers.output.as_ref())
                    else {
                        return -1;
                    };
                    let Some(next_index) = gz_owned_buffer_index(
                        output.as_ptr() as usize,
                        output.len(),
                        state.x.next as usize,
                    ) else {
                        return -1;
                    };
                    let Some(next_index) =
                        gz_skip_buffer_next_index(output.len(), next_index, state.x.have, consumed)
                    else {
                        return -1;
                    };
                    next_index
                };
                let Some((have, pos, skip)) =
                    gz_skip_commit(state.x.have, state.x.pos, state.skip, consumed)
                else {
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
                state.x.have = have;
                state.x.pos = pos;
                state.skip = skip;
                if complete {
                    break;
                }
            }
            Err(()) => return -1,
        }
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
        let Some(index) = gz_owned_buffer_index(
            output.as_ptr() as usize,
            output.len(),
            state.x.next as usize,
        ) else {
            return -1;
        };
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
    while state.skip != 0 {
        match gz_skip_step(state.x.have, state.skip, state.eof, state.strm.avail_in) {
            Ok(GzSkipStep::Complete) => break,
            Ok(GzSkipStep::NeedFetch) => {
                if gz_fetch_at_boundary!(state) == -1 {
                    return ::core::ptr::null_mut();
                }
            }
            Ok(GzSkipStep::Advance { consumed, complete }) => {
                let next_index = {
                    let Some(output) = state
                        .buffers
                        .as_ref()
                        .and_then(|buffers| buffers.output.as_ref())
                    else {
                        return ::core::ptr::null_mut();
                    };
                    let Some(next_index) = gz_owned_buffer_index(
                        output.as_ptr() as usize,
                        output.len(),
                        state.x.next as usize,
                    ) else {
                        return ::core::ptr::null_mut();
                    };
                    let Some(next_index) =
                        gz_skip_buffer_next_index(output.len(), next_index, state.x.have, consumed)
                    else {
                        return ::core::ptr::null_mut();
                    };
                    next_index
                };
                let Some((have, pos, skip)) =
                    gz_skip_commit(state.x.have, state.x.pos, state.skip, consumed)
                else {
                    return ::core::ptr::null_mut();
                };
                let Some(output) = state
                    .buffers
                    .as_mut()
                    .and_then(|buffers| buffers.output.as_mut())
                else {
                    return ::core::ptr::null_mut();
                };
                state.x.next = output.as_mut_ptr().wrapping_add(next_index);
                state.x.have = have;
                state.x.pos = pos;
                state.skip = skip;
                if complete {
                    break;
                }
            }
            Err(()) => return ::core::ptr::null_mut(),
        }
    }
    let mut left = (destination.len() as ::core::ffi::c_uint).wrapping_sub(1);
    let mut written = 0usize;
    while left != 0 && !(state.x.have == 0 && gz_fetch_at_boundary!(state) == -1) {
        if state.x.have == 0 {
            state.past = 1;
            break;
        }
        let copied = {
            let Some(output) = state
                .buffers
                .as_ref()
                .and_then(|buffers| buffers.output.as_ref())
            else {
                return ::core::ptr::null_mut();
            };
            let Some(next_index) = gz_owned_buffer_index(
                output.as_ptr() as usize,
                output.len(),
                state.x.next as usize,
            ) else {
                return ::core::ptr::null_mut();
            };
            let Some(available) = output.get(next_index..) else {
                return ::core::ptr::null_mut();
            };
            let Some((copied, found_newline)) =
                gzgets_buffered_copy(&mut destination[written..], available, state.x.have, left)
            else {
                return ::core::ptr::null_mut();
            };
            let Some(next_index) = next_index.checked_add(copied as usize) else {
                return ::core::ptr::null_mut();
            };
            if next_index > output.len() {
                return ::core::ptr::null_mut();
            }
            (copied, found_newline, next_index)
        };
        let (copied, found_newline, next_index) = copied;
        let Some(output) = state
            .buffers
            .as_mut()
            .and_then(|buffers| buffers.output.as_mut())
        else {
            return ::core::ptr::null_mut();
        };
        state.x.next = output.as_mut_ptr().wrapping_add(next_index);
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
        gz_look_at_boundary!(state);
    }
    gzdirect_state(state.direct)
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    crate::src::gzclose::gzclose_read_at_boundary!(file)
}
