pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_intmax;

pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::deflate;
pub use crate::src::deflate::deflateEnd;
pub use crate::src::deflate::deflateInit2_;
pub use crate::src::deflate::internal_state;

pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpc;
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
pub use crate::zlib_h::Z_BLOCK;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_ERRNO;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;

/// Allocate the writer's owned input and optional output storage.  This
/// decision is independent of stream cursors and the deflate state.
pub(crate) fn gz_init_buffers(
    want: ::core::ffi::c_uint,
    direct: ::core::ffi::c_int,
) -> Option<crate::gzguts_h::gz_buffers> {
    let input_len = (want as usize).checked_mul(2)?;
    let output_len = (direct == 0).then_some(want as usize);
    crate::gzguts_h::gz_buffers::new(input_len, output_len)
}

// These macros expand only in exported writer/close boundaries.  They retain
// the legacy deflate calls and ABI cursor updates at those boundaries until
// deflate has a safe stream-call adapter.
macro_rules! gz_init_at_boundary {
    ($state:expr) => {{
        let state_ref = &mut *$state;
        'gz_init_result: {
            let Some(buffers) =
                crate::src::gzwrite::gz_init_buffers(state_ref.want, state_ref.direct)
            else {
                crate::src::gzlib::gz_error_static(
                    state_ref,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0",
                );
                break 'gz_init_result -1;
            };
            state_ref.buffers = Some(buffers);
            if state_ref.direct == 0 {
                state_ref.strm.zalloc = None;
                state_ref.strm.zfree = None;
                state_ref.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if crate::src::deflate::deflateInit2_(
                    &mut state_ref.strm,
                    state_ref.level,
                    8 as ::core::ffi::c_int,
                    15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    state_ref.strategy,
                ) != crate::zlib_h::Z_OK
                {
                    state_ref.buffers = None;
                    crate::src::gzlib::gz_error_static(
                        state_ref,
                        crate::zlib_h::Z_MEM_ERROR,
                        b"out of memory\0",
                    );
                    break 'gz_init_result -1;
                }
                state_ref.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                let Some(output) = state_ref
                    .buffers
                    .as_mut()
                    .and_then(|buffers| buffers.output.as_mut())
                else {
                    break 'gz_init_result -1;
                };
                state_ref.strm.avail_out = state_ref.want as crate::stdlib::uInt;
                state_ref.strm.next_out = output.as_mut_ptr() as *mut crate::stdlib::Bytef;
                state_ref.x.next = state_ref.strm.next_out as *mut ::core::ffi::c_uchar;
            }
            state_ref.size = state_ref.want;
            0
        }
    }};
}
pub(crate) use gz_init_at_boundary;

/// Classify a completed deflate call using only its scalar result and output
/// capacity transition. The codec boundary keeps the stream borrow, error
/// publication, and cursor state; this core makes impossible output growth
/// explicit before the next loop iteration.
pub(crate) enum GzCompDeflateStep {
    StreamError,
    OutputCorrupt,
    NoOutput,
    Continue,
}

pub(crate) fn gz_comp_after_deflate(
    before: crate::stdlib::uInt,
    after: crate::stdlib::uInt,
    ret: ::core::ffi::c_int,
) -> GzCompDeflateStep {
    if ret == crate::zlib_h::Z_STREAM_ERROR {
        return GzCompDeflateStep::StreamError;
    }
    match crate::src::gzread::gz_codec_output_progress(before, after) {
        None => GzCompDeflateStep::OutputCorrupt,
        Some(0) => GzCompDeflateStep::NoOutput,
        Some(_) => GzCompDeflateStep::Continue,
    }
}

macro_rules! gz_comp_at_boundary {
    ($state:expr, $flush:expr) => {{
        let state_ref = &mut *$state;
        let flush = $flush;
        'gz_comp_result: {
            if state_ref.size == 0 && crate::src::gzwrite::gz_init_at_boundary!(state_ref) == -1 {
                break 'gz_comp_result -1;
            }
            if state_ref.direct != 0 {
                break 'gz_comp_result 0;
            }
            if state_ref.reset != 0 {
                if state_ref.strm.avail_in == 0 && flush == crate::zlib_h::Z_NO_FLUSH {
                    break 'gz_comp_result 0;
                }
                crate::src::deflate::deflate_reset_at_boundary!(&raw mut state_ref.strm);
                state_ref.reset = 0;
            }
            let mut ret = crate::zlib_h::Z_OK;
            loop {
                if state_ref.strm.avail_out == 0
                    || flush != crate::zlib_h::Z_NO_FLUSH
                        && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
                {
                    while state_ref.strm.next_out > state_ref.x.next {
                        let (start_index, end_index, result) = {
                            let (Some(file), Some(output)) = (
                                state_ref.file.as_mut(),
                                state_ref
                                    .buffers
                                    .as_ref()
                                    .and_then(|buffers| buffers.output.as_ref()),
                            ) else {
                                break 'gz_comp_result -1;
                            };
                            let Some((start_index, end_index)) =
                                crate::src::gzwrite::gz_comp_pending_indices(
                                    output.as_ptr() as usize,
                                    output.len(),
                                    state_ref.x.next as usize,
                                    state_ref.strm.next_out as usize,
                                )
                            else {
                                break 'gz_comp_result -1;
                            };
                            let Some(pending) = crate::src::gzwrite::gz_comp_pending_output(
                                output,
                                start_index,
                                end_index,
                            ) else {
                                break 'gz_comp_result -1;
                            };
                            (
                                start_index,
                                end_index,
                                crate::src::gzwrite::gz_direct_write_file(file, pending),
                            )
                        };
                        match result {
                            Ok(written) => {
                                if written != end_index.wrapping_sub(start_index) {
                                    break 'gz_comp_result -1;
                                }
                                let Some(output) = state_ref
                                    .buffers
                                    .as_mut()
                                    .and_then(|buffers| buffers.output.as_mut())
                                else {
                                    break 'gz_comp_result -1;
                                };
                                state_ref.again = 0;
                                state_ref.x.next = output.as_mut_ptr().wrapping_add(end_index);
                            }
                            Err((written, code)) => {
                                let Some(next_index) = start_index.checked_add(written) else {
                                    break 'gz_comp_result -1;
                                };
                                let Some(output) = state_ref
                                    .buffers
                                    .as_mut()
                                    .and_then(|buffers| buffers.output.as_mut())
                                else {
                                    break 'gz_comp_result -1;
                                };
                                if next_index > output.len() {
                                    break 'gz_comp_result -1;
                                }
                                state_ref.x.next = output.as_mut_ptr().wrapping_add(next_index);
                                state_ref.again = (code == crate::stdlib::EAGAIN
                                    || code == crate::stdlib::EWOULDBLOCK)
                                    as ::core::ffi::c_int;
                                crate::src::gzlib::gz_error_io(state_ref, code);
                                break 'gz_comp_result -1;
                            }
                        }
                    }
                    if state_ref.strm.avail_out == 0 {
                        let Some(output) = state_ref
                            .buffers
                            .as_mut()
                            .and_then(|buffers| buffers.output.as_mut())
                        else {
                            break 'gz_comp_result -1;
                        };
                        state_ref.strm.avail_out = state_ref.size as crate::stdlib::uInt;
                        state_ref.strm.next_out = output.as_mut_ptr() as *mut crate::stdlib::Bytef;
                        state_ref.x.next = output.as_mut_ptr();
                    }
                }
                let have = state_ref.strm.avail_out;
                let input_len = state_ref.strm.avail_in as usize;
                if input_len != 0 && state_ref.strm.next_in.is_null() {
                    break 'gz_comp_result -1;
                }
                let mut input = if input_len == 0 {
                    &[]
                } else {
                    ::core::slice::from_raw_parts(state_ref.strm.next_in, input_len)
                };
                let output_start = state_ref.strm.next_out as usize;
                let output_len = state_ref.strm.avail_out as usize;
                let (strm, buffers) = (&mut state_ref.strm, &mut state_ref.buffers);
                let Some(backing) = buffers.as_mut().and_then(|buffers| buffers.output.as_mut())
                else {
                    break 'gz_comp_result -1;
                };
                let base = backing.as_mut_ptr() as usize;
                let Some(start) = output_start.checked_sub(base) else {
                    break 'gz_comp_result -1;
                };
                let Some(end) = start.checked_add(output_len) else {
                    break 'gz_comp_result -1;
                };
                let Some(output) = backing.get_mut(start..end) else {
                    break 'gz_comp_result -1;
                };
                let mut output = crate::src::deflate::DeflateOutput::new(output);
                let pending_state = strm.state as *mut crate::src::deflate::deflate_state;
                if pending_state.is_null() {
                    break 'gz_comp_result -1;
                }
                let pending_len = (*pending_state).pending_buf_size as usize;
                if pending_len != 0 && (*pending_state).pending_buf.is_null() {
                    break 'gz_comp_result -1;
                }
                let pending_buf = if pending_len == 0 {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut((*pending_state).pending_buf, pending_len)
                };
                let Some(mut window_hash) =
                    crate::src::deflate::deflate_window_hash_buffers_at_boundary!(&mut *pending_state)
                else {
                    break 'gz_comp_result -1;
                };
                ret = crate::src::deflate::deflate(
                    strm,
                    &mut *pending_state,
                    &mut input,
                    pending_buf,
                    &mut window_hash,
                    &mut output,
                    crate::src::deflate::DeflateGzipPayloads::empty(),
                    flush,
                );
                match crate::src::gzwrite::gz_comp_after_deflate(
                    have,
                    state_ref.strm.avail_out,
                    ret,
                ) {
                    crate::src::gzwrite::GzCompDeflateStep::StreamError => {
                        crate::src::gzlib::gz_error_static(
                            state_ref,
                            crate::zlib_h::Z_STREAM_ERROR,
                            b"internal error: deflate stream corrupt\0",
                        );
                        break 'gz_comp_result -1;
                    }
                    crate::src::gzwrite::GzCompDeflateStep::OutputCorrupt => {
                        crate::src::gzlib::gz_error_static(
                            state_ref,
                            crate::zlib_h::Z_STREAM_ERROR,
                            b"internal error: deflate output corrupt\0",
                        );
                        break 'gz_comp_result -1;
                    }
                    crate::src::gzwrite::GzCompDeflateStep::NoOutput => break,
                    crate::src::gzwrite::GzCompDeflateStep::Continue => {}
                }
            }
            if flush == crate::zlib_h::Z_FINISH {
                state_ref.reset = 1;
            }
            0
        }
    }};
}
pub(crate) use gz_comp_at_boundary;

/// Choose one zero-fill chunk without converting the gzip handle or its
/// buffers.  The narrowing cast deliberately matches zlib's `unsigned`
/// chunk size when a malformed negative skip reaches this private adapter.
pub(crate) fn gz_zero_chunk_plan(
    size: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
) -> ::core::ffi::c_uint {
    if (::core::mem::size_of::<::core::ffi::c_int>()
        == ::core::mem::size_of::<crate::stdlib::off64_t>()
        && size > crate::src::gzlib::gz_intmax())
        || size as crate::stdlib::off64_t > skip
    {
        skip as ::core::ffi::c_uint
    } else {
        size
    }
}

/// Record the number of zero bytes consumed by a completed compression step.
/// The raw adapter owns the input cursor and compressor call; this only keeps
/// the C-style logical-position and pending-seek arithmetic together.
pub(crate) fn gz_zero_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    consumed: ::core::ffi::c_uint,
) {
    state.x.pos = state.x.pos.wrapping_add(consumed as crate::stdlib::off64_t);
    state.skip = state.skip.wrapping_sub(consumed as crate::stdlib::off64_t);
}

/// Commit a transparent sparse-write prefix. `gz_direct_write_commit()` has
/// already advanced the logical position, so this path only consumes seek
/// debt and must not apply the compressed-path position update a second time.
fn gz_zero_commit_direct(state: &mut crate::gzguts_h::gz_state, consumed: ::core::ffi::c_uint) {
    state.skip = state.skip.wrapping_sub(consumed as crate::stdlib::off64_t);
}

/// Write a pending sparse gap for a transparent stream.  This branch never
/// enters the compressor: it receives only the owned zero buffer, descriptor,
/// and scalar state, leaving opaque gzip-state traversal at its boundary.
pub(crate) enum GzZeroDirect {
    Complete(usize),
    IoError {
        consumed: usize,
        code: ::core::ffi::c_int,
    },
    Invalid,
}

/// Commit one completed transparent sparse-write chunk. A successful file
/// write must make progress here: accepting a zero-byte chunk would leave the
/// pending seek unchanged and spin forever. Keep this scalar-only so the
/// descriptor loop never needs to rely on wrapping progress arithmetic.
fn gz_zero_direct_progress(
    consumed: usize,
    skip: crate::stdlib::off64_t,
    written: usize,
) -> Option<(usize, crate::stdlib::off64_t)> {
    if written == 0 {
        return None;
    }
    Some((
        consumed.checked_add(written)?,
        skip.checked_sub(written as crate::stdlib::off64_t)?,
    ))
}

pub(crate) fn gz_zero_direct(
    file: &mut ::std::fs::File,
    input: &mut [u8],
    size: ::core::ffi::c_uint,
    mut skip: crate::stdlib::off64_t,
) -> GzZeroDirect {
    let mut first = true;
    let mut consumed = 0usize;
    loop {
        let mut n = gz_zero_chunk_plan(size, skip);
        if first {
            let Some(bytes) = input.get_mut(..n as usize) else {
                return GzZeroDirect::Invalid;
            };
            bytes.fill(0);
            first = false;
        }
        let Some(bytes) = input.get(..n as usize) else {
            return GzZeroDirect::Invalid;
        };
        let written = match gz_direct_write_file(file, bytes) {
            Ok(written) => written,
            Err((written, code)) => {
                let Some(consumed) = consumed.checked_add(written) else {
                    return GzZeroDirect::Invalid;
                };
                return GzZeroDirect::IoError { consumed, code };
            }
        };
        let Some((next_consumed, next_skip)) = gz_zero_direct_progress(consumed, skip, written)
        else {
            return GzZeroDirect::Invalid;
        };
        consumed = next_consumed;
        skip = next_skip;
        if skip == 0 {
            return GzZeroDirect::Complete(consumed);
        }
    }
}

/// Scalar state commit required by a transparent sparse zero-fill. The
/// exported gzip boundary owns the opaque-state writes and diagnostic; this
/// plan preserves wrapping position/seek accounting without making a safe
/// helper traverse `gz_state`.
pub(crate) enum GzZeroDirectCommit {
    Complete {
        pos: crate::stdlib::off64_t,
        skip: crate::stdlib::off64_t,
    },
    IoError {
        pos: crate::stdlib::off64_t,
        skip: crate::stdlib::off64_t,
        again: ::core::ffi::c_int,
        code: ::core::ffi::c_int,
    },
    Invalid,
}

pub(crate) fn gz_zero_direct_commit(
    pos: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
    result: GzZeroDirect,
) -> GzZeroDirectCommit {
    match result {
        GzZeroDirect::Complete(consumed) => GzZeroDirectCommit::Complete {
            pos: pos.wrapping_add(consumed as crate::stdlib::off64_t),
            skip: skip.wrapping_sub(consumed as crate::stdlib::off64_t),
        },
        GzZeroDirect::IoError { consumed, code } => GzZeroDirectCommit::IoError {
            pos: pos.wrapping_add(consumed as crate::stdlib::off64_t),
            skip: skip.wrapping_sub(consumed as crate::stdlib::off64_t),
            again: (code == crate::stdlib::EAGAIN || code == crate::stdlib::EWOULDBLOCK)
                as ::core::ffi::c_int,
            code,
        },
        GzZeroDirect::Invalid => GzZeroDirectCommit::Invalid,
    }
}

/// Determine how much input fits after the existing buffered compressor
/// input.  The wrapping subtraction deliberately preserves zlib's behavior
/// for a malformed internal cursor while keeping the size conversion local.
fn gz_write_buffered_copy_plan(
    size: ::core::ffi::c_uint,
    buffered_end: ::core::ffi::c_uint,
    remaining: crate::stdlib::z_size_t,
) -> ::core::ffi::c_uint {
    let copy = size.wrapping_sub(buffered_end);
    if copy as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        copy
    }
}

/// Copy as much caller input as fits after the pending compressor input.
/// Both cursors are indices here: the raw gzip adapter reconciles its ABI
/// stream pointer before entering this core.  Keeping the copy bounded by
/// the owned allocation avoids the old pointer arithmetic and libc memcpy.
fn gz_write_buffered_copy(
    input: &mut [u8],
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    next_index: usize,
    source: &[u8],
) -> Option<(::core::ffi::c_uint, crate::stdlib::uInt)> {
    let size = size as usize;
    if size == 0 || size > input.len() || next_index > size {
        return None;
    }
    let end = next_index.checked_add(avail_in as usize)?;
    if end > size {
        return None;
    }
    let copy = gz_write_buffered_copy_plan(
        size as ::core::ffi::c_uint,
        end as ::core::ffi::c_uint,
        source.len(),
    );
    let copy_len = copy as usize;
    input
        .get_mut(end..end.checked_add(copy_len)?)?
        .copy_from_slice(source.get(..copy_len)?);
    Some((copy, avail_in.wrapping_add(copy)))
}

/// Commit the logical-position portion of a bounded pending-input copy.
/// The slice core already returned the updated `avail_in` value, so it is
/// intentionally not adjusted here.
fn gz_write_buffered_copy_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    copied: ::core::ffi::c_uint,
) {
    state.x.pos = state.x.pos.wrapping_add(copied as crate::stdlib::off64_t);
}

/// Limit one direct-write compression input chunk to the `uInt` range used
/// by zlib, without touching the caller-owned input cursor.
fn gz_write_direct_chunk_plan(remaining: crate::stdlib::z_size_t) -> ::core::ffi::c_uint {
    let max = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    if max as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        max
    }
}

/// Advance the temporary caller-input view after a compressor step.  The
/// codec reports its progress through `avail_in`; validate that report before
/// it is used as either Rust slice or scalar progress.  A corrupt/stale codec
/// cursor must fail the gzip write instead of wrapping the count and panicking
/// on a slice range.
fn gz_write_direct_progress<'a>(
    source: &'a [u8],
    remaining: crate::stdlib::z_size_t,
    before: crate::stdlib::uInt,
    after: crate::stdlib::uInt,
) -> Option<(&'a [u8], crate::stdlib::z_size_t, ::core::ffi::c_uint)> {
    if source.len() != remaining {
        return None;
    }
    let consumed = before.checked_sub(after)?;
    let consumed = usize::try_from(consumed).ok()?;
    let next_remaining = remaining.checked_sub(consumed)?;
    Some((
        source.get(consumed..)?,
        next_remaining,
        consumed as ::core::ffi::c_uint,
    ))
}

/// Commit progress reported by a direct-write compression step.  The caller
/// keeps the raw stream cursor at the boundary; this preserves zlib's
/// wrapping logical-position arithmetic and returns the remaining input.
fn gz_write_direct_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    consumed: ::core::ffi::c_uint,
    next_remaining: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    state.x.pos = state.x.pos.wrapping_add(consumed as crate::stdlib::off64_t);
    next_remaining
}

/// Write a direct (transparent) gzip payload through the owned descriptor.
/// The result includes the completed prefix on failure, matching the old
/// `write()` loop's partial-progress behavior without exposing a raw buffer
/// or descriptor to the gzip implementation.
pub(crate) fn gz_direct_write_file(
    file: &mut ::std::fs::File,
    source: &[u8],
) -> Result<usize, (usize, ::core::ffi::c_int)> {
    use std::io::Write;

    let max = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2).wrapping_add(1) as usize;
    let mut written = 0usize;
    while written != source.len() {
        let end = written.saturating_add(max).min(source.len());
        let Some(chunk) = source.get(written..end) else {
            return Err((written, 0));
        };
        match file.write(chunk) {
            Ok(0) => return Err((written, 0)),
            Ok(count) => written += count,
            Err(error) => return Err((written, error.raw_os_error().unwrap_or(0))),
        }
    }
    Ok(written)
}

/// Borrow the pending compressor output after ABI cursors have been
/// reconciled to owned-buffer indices.
pub(crate) fn gz_comp_pending_output(
    output: &[u8],
    start_index: usize,
    end_index: usize,
) -> Option<&[u8]> {
    output.get(start_index..end_index)
}

/// Reconcile the two compatibility output cursors to an owned-buffer range.
/// The export-only compressor adapter supplies address tokens; this core
/// rejects stale, reversed, or out-of-bounds cursors before it lends the
/// pending bytes to descriptor I/O.
pub(crate) fn gz_comp_pending_indices(
    output_start: usize,
    output_len: usize,
    pending_start: usize,
    pending_end: usize,
) -> Option<(usize, usize)> {
    let start_index = pending_start.checked_sub(output_start)?;
    let end_index = pending_end.checked_sub(output_start)?;
    if start_index > end_index || end_index > output_len {
        return None;
    }
    Some((start_index, end_index))
}

/// Commit a direct-write result and map descriptor failure to gzip's stable
/// owned diagnostic.  This keeps error and position updates independent of
/// the particular owned slice used for the write (caller input or zero fill).
fn gz_direct_write_commit(
    state: &mut crate::gzguts_h::gz_state,
    result: Result<usize, (usize, ::core::ffi::c_int)>,
) -> Result<usize, usize> {
    match result {
        Ok(written) => {
            state.again = 0;
            state.x.pos = state.x.pos.wrapping_add(written as crate::stdlib::off64_t);
            Ok(written)
        }
        Err((written, code)) => {
            state.again = (code == crate::stdlib::EAGAIN || code == crate::stdlib::EWOULDBLOCK)
                as ::core::ffi::c_int;
            state.x.pos = state.x.pos.wrapping_add(written as crate::stdlib::off64_t);
            crate::src::gzlib::gz_error_io(state, code);
            Err(written)
        }
    }
}

/// Write a caller-supplied transparent payload through the state's RAII
/// descriptor.  The slice remains call-scoped and no stream cursor needs to
/// point at caller memory.
fn gz_write_direct(state: &mut crate::gzguts_h::gz_state, source: &[u8]) -> Result<usize, usize> {
    let result = match state.file.as_mut() {
        Some(file) => gz_direct_write_file(file, source),
        None => Err((0, 0)),
    };
    gz_direct_write_commit(state, result)
}

/// Convert a failed compression step into zlib's public write progress.  A
/// retryable descriptor error reports the completed prefix; every other
/// error reports no completed write.
fn gz_write_failure_result(
    again: ::core::ffi::c_int,
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if again != 0 {
        requested.wrapping_sub(remaining)
    } else {
        0
    }
}

/// Validate the common public gzip-writer admission state without borrowing
/// the opaque handle.  A retryable descriptor error remains writable, as it
/// does for every write-family entry point.
fn gzwrite_state_is_valid(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
) -> bool {
    mode == crate::gzguts_h::GZ_WRITE && (err == crate::zlib_h::Z_OK || again != 0)
}

/// Compute the byte length requested by `gzfwrite`.  `None` preserves the
/// API's overflow failure, while `Some(0)` remains an ordinary empty request.
fn gzfwrite_request_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    let len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        None
    } else {
        Some(len)
    }
}

/// Convert completed byte count back to completed items after a non-empty
/// `gzfwrite` request.  The caller keeps the raw write and handle access at
/// the boundary.
fn gzfwrite_completed_items(
    written: crate::stdlib::z_size_t,
    size: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    written.wrapping_div(size)
}

/// Accept precisely the flush values supported by the public gzip writer.
/// The opaque handle and compressor remain at the FFI boundary.
fn gzflush_is_valid(flush: ::core::ffi::c_int) -> bool {
    (crate::zlib_h::Z_NO_FLUSH..=crate::zlib_h::Z_FINISH).contains(&flush)
}

/// Validate the scalar gzip-writer state needed by `gzsetparams`, without
/// borrowing the opaque handle or touching the compressor.  A pending retry
/// is permitted to match zlib's existing write-side admission rule.
fn gzsetparams_state_is_valid(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
) -> bool {
    mode == crate::gzguts_h::GZ_WRITE && (err == crate::zlib_h::Z_OK || again != 0) && direct == 0
}

/// Decide whether a parameter update has work to do.  Keeping this separate
/// from the handle adapter avoids observing or changing state before the
/// existing no-op return.
fn gzsetparams_needs_update(
    current_level: ::core::ffi::c_int,
    current_strategy: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> bool {
    level != current_level || strategy != current_strategy
}

/// Apply the descriptor-close outcome to the write-side close result.  The
/// compressor, descriptor, and owned buffers remain at the raw boundary.
pub(crate) fn gzclose_write_result(
    close_result: ::core::ffi::c_int,
    result_before_close: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if close_result == -1 {
        crate::zlib_h::Z_ERRNO
    } else {
        result_before_close
    }
}

/// Insert one byte into the owned pending-compression input.  The ABI stream
/// cursor is represented by `next_index` for this operation, so this core
/// only needs the owned slice and scalar stream state.
fn gzputc_buffered_insert(
    input: &mut [u8],
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    next_index: usize,
    pos: crate::stdlib::off64_t,
    c: ::core::ffi::c_int,
) -> Option<(usize, crate::stdlib::uInt, crate::stdlib::off64_t)> {
    if size == 0 || size as usize > input.len() {
        return None;
    }
    let end = next_index.checked_add(avail_in as usize)?;
    if next_index > size as usize || end >= size as usize {
        return None;
    }
    *input.get_mut(end)? = c as ::core::ffi::c_uchar;
    Some((next_index, avail_in.wrapping_add(1), pos.wrapping_add(1)))
}

// This expansion is deliberately limited to exported writer boundaries.  It
// retains the compressor call and ABI stream cursor updates there while the
// zero-fill and descriptor work continues to use the safe helpers above.
macro_rules! gz_zero_at_boundary {
    ($state:expr) => {{
        'gz_zero_result: {
            let state = &mut *$state;
            let mut ret: ::core::ffi::c_int = 0;
            let mut n: ::core::ffi::c_uint = 0;
            if state.strm.avail_in != 0
                && crate::src::gzwrite::gz_comp_at_boundary!(state, crate::zlib_h::Z_NO_FLUSH)
                    == -1 as ::core::ffi::c_int
            {
                break 'gz_zero_result -1 as ::core::ffi::c_int;
            }
            if state.direct != 0 {
                let result = {
                    let (Some(file), Some(buffers)) = (state.file.as_mut(), state.buffers.as_mut())
                    else {
                        break 'gz_zero_result -1;
                    };
                    crate::src::gzwrite::gz_zero_direct(
                        file,
                        &mut buffers.input,
                        state.size,
                        state.skip,
                    )
                };
                break 'gz_zero_result match crate::src::gzwrite::gz_zero_direct_commit(
                    state.x.pos,
                    state.skip,
                    result,
                ) {
                    crate::src::gzwrite::GzZeroDirectCommit::Complete { pos, skip } => {
                        state.again = 0;
                        state.x.pos = pos;
                        state.skip = skip;
                        0
                    }
                    crate::src::gzwrite::GzZeroDirectCommit::IoError {
                        pos,
                        skip,
                        again,
                        code,
                    } => {
                        state.again = again;
                        state.x.pos = pos;
                        state.skip = skip;
                        crate::src::gzlib::gz_error_io(state, code);
                        -1
                    }
                    crate::src::gzwrite::GzZeroDirectCommit::Invalid => -1,
                };
            }
            let mut first = true;
            loop {
                n = crate::src::gzwrite::gz_zero_chunk_plan(state.size, state.skip);
                if first {
                    // The compressor still receives the old compatibility cursor,
                    // but the sparse gap is initialized through its owned allocation
                    // rather than libc `memset`.
                    let Some(buffers) = state.buffers.as_mut() else {
                        break 'gz_zero_result -1 as ::core::ffi::c_int;
                    };
                    let Some(bytes) = buffers.input.get_mut(..n as usize) else {
                        break 'gz_zero_result -1 as ::core::ffi::c_int;
                    };
                    bytes.fill(0);
                    first = false;
                }
                state.strm.avail_in = n as crate::stdlib::uInt;
                let Some(input) = state.buffers.as_mut().map(|buffers| buffers.input.as_mut())
                else {
                    break 'gz_zero_result -1;
                };
                state.strm.next_in = input.as_mut_ptr() as *mut crate::stdlib::Bytef;
                ret = crate::src::gzwrite::gz_comp_at_boundary!(state, crate::zlib_h::Z_NO_FLUSH);
                n = n.wrapping_sub(state.strm.avail_in as ::core::ffi::c_uint);
                crate::src::gzwrite::gz_zero_commit_state(state, n);
                if ret == -1 as ::core::ffi::c_int {
                    break 'gz_zero_result -1 as ::core::ffi::c_int;
                }
                if state.skip == 0 {
                    break;
                }
            }
            break 'gz_zero_result 0 as ::core::ffi::c_int;
        }
    }};
}
pub(crate) use gz_zero_at_boundary;

// The compressor and ABI stream cursors are deliberately kept in exported
// writer boundaries. This macro is not an implementation function: every
// expansion is in an export wrapper that already owns the validated handle
// and its call-scoped input view.
macro_rules! gz_write_at_boundary {
    ($state:expr, $source:expr) => {{
        'gz_write_result: {
            let state = $state;
            let mut source: &[u8] = $source;
            let mut len = source.len();
            let put: crate::stdlib::z_size_t = len;
            let mut ret: ::core::ffi::c_int = 0;
            if len == 0 as crate::stdlib::z_size_t {
                break 'gz_write_result 0 as crate::stdlib::z_size_t;
            }
            if state.size == 0 as ::core::ffi::c_uint
                && gz_init_at_boundary!(state) == -1 as ::core::ffi::c_int
            {
                break 'gz_write_result 0 as crate::stdlib::z_size_t;
            }
            if state.skip != 0 && gz_zero_at_boundary!(state) == -1 as ::core::ffi::c_int {
                break 'gz_write_result 0 as crate::stdlib::z_size_t;
            }
            if state.direct != 0 {
                break 'gz_write_result match gz_write_direct(state, source) {
                    Ok(written) => written,
                    Err(written) => gz_write_failure_result(state.again, put, put - written),
                };
            }
            if len < state.size as crate::stdlib::z_size_t {
                loop {
                    let Some(input) = state.buffers.as_ref().map(|buffers| &buffers.input) else {
                        break 'gz_write_result 0;
                    };
                    if state.strm.avail_in == 0 as crate::stdlib::uInt {
                        state.strm.next_in = input.as_ptr() as *mut crate::stdlib::Bytef;
                    }
                    let Some(next_index) = (state.strm.next_in as usize)
                        .checked_sub(input.as_ptr() as usize)
                        .filter(|index| *index <= input.len())
                    else {
                        break 'gz_write_result 0;
                    };
                    let copied = {
                        let Some(input) =
                            state.buffers.as_mut().map(|buffers| buffers.input.as_mut())
                        else {
                            break 'gz_write_result 0;
                        };
                        gz_write_buffered_copy(
                            input,
                            state.size,
                            state.strm.avail_in,
                            next_index,
                            source,
                        )
                    };
                    let Some((copy, avail_in)) = copied else {
                        break 'gz_write_result 0;
                    };
                    state.strm.avail_in = avail_in;
                    gz_write_buffered_copy_commit_state(state, copy);
                    source = &source[copy as usize..];
                    len = source.len();
                    if len == 0 as crate::stdlib::z_size_t {
                        break;
                    }
                    if gz_comp_at_boundary!(state, crate::zlib_h::Z_NO_FLUSH)
                        == -1 as ::core::ffi::c_int
                    {
                        break 'gz_write_result gz_write_failure_result(state.again, put, len);
                    }
                }
            } else {
                if state.strm.avail_in != 0
                    && gz_comp_at_boundary!(state, crate::zlib_h::Z_NO_FLUSH)
                        == -1 as ::core::ffi::c_int
                {
                    break 'gz_write_result 0 as crate::stdlib::z_size_t;
                }
                state.strm.next_in = source.as_ptr() as *mut crate::stdlib::Bytef;
                loop {
                    let n = gz_write_direct_chunk_plan(len);
                    state.strm.avail_in = n as crate::stdlib::uInt;
                    ret = gz_comp_at_boundary!(state, crate::zlib_h::Z_NO_FLUSH);
                    let Some((next_source, next_len, consumed)) = gz_write_direct_progress(
                        source,
                        len,
                        n as crate::stdlib::uInt,
                        state.strm.avail_in,
                    ) else {
                        break 'gz_write_result 0;
                    };
                    len = gz_write_direct_commit_state(state, consumed, next_len);
                    source = next_source;
                    if ret == -1 as ::core::ffi::c_int {
                        break 'gz_write_result gz_write_failure_result(state.again, put, len);
                    }
                    if len == 0 {
                        break;
                    }
                }
            }
            break 'gz_write_result put;
        }
    }};
}
#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzwrite_state_is_valid(state.mode, state.err, state.again) {
        return 0;
    }
    crate::src::gzlib::gz_error_clear(state);
    if !crate::src::gzlib::gz_request_len_fits_int(len) {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            b"requested length does not fit in int\0",
        );
        return 0;
    }
    if len != 0 && buf.is_null() {
        return 0;
    }
    let source = if len == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(buf as *const u8, len as usize)
    };
    gz_write_at_boundary!(state, source) as ::core::ffi::c_int
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    if file.is_null() {
        return 0;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzwrite_state_is_valid(state.mode, state.err, state.again) {
        return 0;
    }
    crate::src::gzlib::gz_error_clear(state);
    let len = match gzfwrite_request_len(size, nitems) {
        Some(len) => len,
        None => {
            crate::src::gzlib::gz_error_static(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"request does not fit in a size_t\0",
            );
            return 0;
        }
    };
    if len == 0 {
        return 0;
    }
    if len != 0 && buf.is_null() {
        return 0;
    }
    let source = if len == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(buf as *const u8, len)
    };
    gzfwrite_completed_items(gz_write_at_boundary!(state, source), size)
}
#[export_name = "gzputc"]
pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzwrite_state_is_valid(state.mode, state.err, state.again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state);
    if state.skip != 0 && gz_zero_at_boundary!(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.size != 0 {
        // `strm.next_in` is an ABI cursor into the owned input allocation.
        // Reconcile it at the boundary before passing only an index to the
        // slice-based insertion core.
        let next_index = if state.strm.avail_in == 0 {
            Some(0)
        } else {
            let Some(input) = state.buffers.as_ref().map(|buffers| &buffers.input) else {
                return -1;
            };
            (state.strm.next_in as usize)
                .checked_sub(input.as_ptr() as usize)
                .filter(|index| *index <= input.len())
        };
        let Some(next_index) = next_index else {
            return -1;
        };
        let inserted = {
            let Some(input) = state.buffers.as_mut().map(|buffers| buffers.input.as_mut()) else {
                return -1;
            };
            gzputc_buffered_insert(
                input,
                state.size,
                state.strm.avail_in,
                next_index,
                state.x.pos,
                c,
            )
        };
        if let Some((next_index, avail_in, pos)) = inserted {
            let Some(input) = state.buffers.as_mut().map(|buffers| buffers.input.as_mut()) else {
                return -1;
            };
            state.strm.next_in = input.as_mut_ptr().wrapping_add(next_index);
            state.strm.avail_in = avail_in;
            state.x.pos = pos;
            return c & 0xff as ::core::ffi::c_int;
        }
        // A full pending input buffer follows the existing `gz_write` path
        // below, which flushes it before inserting this byte.
    }
    buf[0 as ::core::ffi::c_int as usize] = c as ::core::ffi::c_uchar;
    if gz_write_at_boundary!(state, &buf) != 1 as crate::stdlib::z_size_t {
        return -1 as ::core::ffi::c_int;
    }
    return c & 0xff as ::core::ffi::c_int;
}
#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if file.is_null() || s.is_null() {
        return -1;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzwrite_state_is_valid(state.mode, state.err, state.again) {
        return -1;
    }
    crate::src::gzlib::gz_error_clear(state);
    let bytes = ::std::ffi::CStr::from_ptr(s).to_bytes();
    let len = bytes.len();
    if !crate::src::gzlib::gz_len_fits_int(len) {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"string length does not fit in int\0",
        );
        return -1;
    }
    let put = gz_write_at_boundary!(state, bytes);
    if len != 0 && put == 0 {
        -1
    } else {
        put as ::core::ffi::c_int
    }
}
#[export_name = "gzflush"]
pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzwrite_state_is_valid(state.mode, state.err, state.again) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error_clear(state);
    if !gzflush_is_valid(flush) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 && gz_zero_at_boundary!(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    gz_comp_at_boundary!(state, flush);
    state.err
}
#[export_name = "gzsetparams"]
pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzsetparams_state_is_valid(state.mode, state.err, state.again, state.direct) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error_clear(state);
    if !gzsetparams_needs_update(state.level, state.strategy, level, strategy) {
        return crate::zlib_h::Z_OK;
    }
    if state.skip != 0 && gz_zero_at_boundary!(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    if state.size != 0 {
        if state.strm.avail_in != 0
            && gz_comp_at_boundary!(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
        {
            return state.err;
        }
        crate::src::deflate::deflate_params_at_boundary!(&raw mut state.strm, level, strategy);
    }
    state.level = level;
    state.strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    crate::src::gzclose::gzclose_write_at_boundary!(file)
}
