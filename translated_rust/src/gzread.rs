pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZIP;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::LOOK;
pub use crate::src::gzlib::gz_error;
pub use crate::src::gzlib::gz_intmax;
pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate;
pub use crate::src::inflate::inflateEnd;
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

// Publishing newly allocated read buffers and clearing the inflater's input
// fields is an ordinary state transition. Keep it separate from allocation
// and inflater construction in `gz_look()`.
fn gz_look_prepare_stream(state: &mut crate::gzguts_h::gz_state) {
    state.size = state.want;
    state.strm.zalloc = None;
    state.strm.zfree = None;
    state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    state.strm.avail_in = 0;
    state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
}

// A failed inflater construction leaves the allocated buffers released and
// returns the state to its uninitialized buffer size.
fn gz_look_init_failed(state: &mut crate::gzguts_h::gz_state) {
    state.size = 0;
}

// Once the gzip input and output allocations have been bounded at the state
// boundary, recognizing a transparent (non-gzip) stream only needs an
// ordinary slice copy. Keeping the transfer here avoids a raw C `memcpy` in
// the lookahead state machine.
fn gz_copy_lookahead_output(output: &mut [::core::ffi::c_uchar], input: &[::core::ffi::c_uchar]) {
    output[..input.len()].copy_from_slice(input);
}

struct GzLoadResult {
    received: ::core::ffi::c_uint,
    status: ::core::ffi::c_int,
}

// A POSIX syscall sets the thread-local OS error only on failure. The gzip
// state machine only consults it for a negative result, so reading it through
// Rust's OS-error API preserves the C ordering without a raw errno pointer.
fn gz_last_errno() -> ::core::ffi::c_int {
    ::std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

// This is the descriptor-read boundary. Its private callers retain a bounded
// Rust buffer through the syscall, so the partial-read cursor can stay a
// checked slice rather than a raw pointer plus manual address arithmetic.
// Keep the byte count in a typed result instead of passing a raw out-pointer
// through each caller, including the partial-read error case that gzip must
// still account for.
fn gz_load(
    state: &mut crate::gzguts_h::gz_state,
    buf: &mut [::core::ffi::c_uchar],
) -> GzLoadResult {
    let mut ret: ::core::ffi::c_int = 0;
    let mut get: ::core::ffi::c_uint = 0;
    // Keep the byte count local while crossing the raw read boundary.  The
    // caller only observes it after the descriptor result has been classified.
    let mut loaded: ::core::ffi::c_uint = 0;
    let len = buf.len() as ::core::ffi::c_uint;
    crate::src::gzlib::gz_begin_io(state);
    loop {
        get = crate::src::gzlib::gz_load_request(len, loaded);
        ret = crate::stdlib::read(
            state.fd,
            buf[loaded as usize..].as_mut_ptr() as *mut ::core::ffi::c_void,
            get as crate::__stddef_size_t_h::size_t,
        ) as ::core::ffi::c_int;
        if ret <= 0 {
            break;
        }
        loaded = crate::src::gzlib::gz_add_received(loaded, ret as ::core::ffi::c_uint);
        if loaded >= len {
            break;
        }
    }
    let errno = if ret < 0 { gz_last_errno() } else { 0 };
    if let Err(_) = crate::src::gzlib::gz_load_result(state, ret, loaded, errno) {
        let message = crate::src::gzlib::gz_errno_message();
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_ERRNO,
            Some(&message),
        );
        return GzLoadResult {
            received: loaded,
            status: -1,
        };
    }
    GzLoadResult {
        received: loaded,
        status: 0,
    }
}

// Compute the still-buffered input range without forming a slice from the
// cursor itself. The two non-null cursors are part of gzip's initialized
// input allocation; checking their address-derived range keeps the later
// `copy_within()` and refill slice within the one allocation bound here.
fn gz_buffered_input_range(
    input_addr: usize,
    cursor_addr: usize,
    buffered: ::core::ffi::c_uint,
    capacity: usize,
) -> Option<::core::ops::Range<usize>> {
    let start = cursor_addr.checked_sub(input_addr)?;
    let end = start.checked_add(buffered as usize)?;
    (end <= capacity).then_some(start..end)
}

// This helper is internal and all of its callers have already bound the
// validated gzip state. Descriptor I/O remains confined to `gz_load`.
fn gz_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut got: ::core::ffi::c_uint = 0;
    let plan = match crate::src::gzlib::gz_avail_plan(state) {
        Ok(plan) => plan,
        Err(()) => return -1,
    };
    if let crate::src::gzlib::GzAvailPlan::Load {
        buffered,
        requested: _,
    } = plan
    {
        // `gz_look()` publishes this fixed-size allocation in the owned
        // buffer registry before `gz_avail()` can refill it. Borrow the Vec
        // there instead of rebuilding a slice from `state.in_0`.
        let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
        let refill = crate::src::gzlib::gz_with_owned_input_buffer(state_key, |buffer| {
            if state.in_0 != buffer.as_mut_ptr() || state.size as usize != buffer.len() {
                return None;
            }
            if buffered != 0 {
                let cursor_at_start = state.strm.next_in == state.in_0;
                if crate::src::gzlib::gz_avail_needs_compaction(buffered, cursor_at_start) {
                    let Some(range) = gz_buffered_input_range(
                        buffer.as_ptr().addr(),
                        state.strm.next_in.addr(),
                        buffered,
                        buffer.len(),
                    )
                    else {
                        return None;
                    };
                    buffer.copy_within(range, 0);
                }
            }
            Some(gz_load(state, &mut buffer[buffered as usize..]))
        });
        let Some(Some(result)) = refill else {
            return -1;
        };
        got = result.received;
        if result.status == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        crate::src::gzlib::gz_avail_after_load(state, got);
    }
    0
}

// All callers have already validated and bound the gzip state. Allocation,
// inflater setup, and input-buffer operations stay in documented, scoped raw
// boundaries within this adapter.
fn gz_look(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size == 0 as ::core::ffi::c_uint {
        // Keep both zero-filled buffers local until the inflater and their
        // registry ownership are ready. This preserves C's allocation order
        // while avoiding a partially-published state across setup.
        let Some(input) = crate::src::gzlib::gz_owned_buffer(state.want as usize) else {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory\0"),
            );
            return -1 as ::core::ffi::c_int;
        };
        let Some(output) = crate::src::gzlib::gz_owned_buffer(state.want.wrapping_shl(1) as usize) else {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory\0"),
            );
            return -1 as ::core::ffi::c_int;
        };
        gz_look_prepare_stream(state);
        let init_ret = crate::src::inflate::inflateInit2_(
            Some(&mut state.strm),
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            Some(crate::zlib_h::ZLIB_VERSION[0] as ::core::ffi::c_char),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        );
        if init_ret != crate::zlib_h::Z_OK {
            gz_look_init_failed(state);
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory\0"),
            );
            return -1 as ::core::ffi::c_int;
        }
        if !crate::src::gzlib::gz_register_owned_buffers(state, input, output) {
            crate::src::inflate::inflateEnd(&mut state.strm);
            gz_look_init_failed(state);
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory\0"),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if state.direct == -1 as ::core::ffi::c_int || state.junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflate_reset_stream_bound(&mut state.strm);
        crate::src::gzlib::gz_set_gzip_input(state, state.junk != -1 as ::core::ffi::c_int);
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let available = state.strm.avail_in;
    // Both allocations were published together by `gz_look()` initialization.
    // Keep the cursor range within the owned input Vec and perform a possible
    // transparent-stream copy while both Rust slices are borrowed from that
    // single registry entry.  This avoids rebuilding slices from `next_in`
    // and `out`, and does not nest the registry mutex.
    let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
    let look = crate::src::gzlib::gz_with_owned_read_buffers(state_key, |input, output| {
        if state.in_0 != input.as_mut_ptr() || state.out != output.as_mut_ptr() {
            return None;
        }
        let input_range = gz_buffered_input_range(
            input.as_ptr().addr(),
            state.strm.next_in.addr(),
            available,
            input.len(),
        )?;
        let gzip_header = if available > 3 {
            let header = &input[input_range.start..input_range.start + 4];
            crate::src::gzlib::gz_is_gzip_header([header[0], header[1], header[2], header[3]])
        } else {
            false
        };
        let plan = crate::src::gzlib::gz_look_plan(available, state.again != 0, gzip_header);
        if let crate::src::gzlib::GzLookPlan::Copy { copied } = plan {
            let copied = copied as usize;
            if copied > output.len() {
                return None;
            }
            gz_copy_lookahead_output(&mut output[..copied], &input[input_range]);
        }
        Some(plan)
    });
    let Some(Some(look)) = look else {
        return -1;
    };
    match look {
        crate::src::gzlib::GzLookPlan::NeedMore => return 0 as ::core::ffi::c_int,
        crate::src::gzlib::GzLookPlan::Gzip => {
            crate::src::inflate::inflate_reset_stream_bound(&mut state.strm);
            crate::src::gzlib::gz_set_gzip_input(state, true);
            return 0 as ::core::ffi::c_int;
        }
        crate::src::gzlib::GzLookPlan::Copy { copied } => {
            crate::src::gzlib::gz_set_copy_input(state, copied);
            return 0 as ::core::ffi::c_int;
        }
    }
}

// The decompressor is likewise internal to the read state machine. Its state
// transition is reference-bound by every caller; keep its inflater and error
// bridges tightly scoped to the operations that cross those raw boundaries.
fn gz_decomp(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut had: ::core::ffi::c_uint = 0;
    had = state.strm.avail_out as ::core::ffi::c_uint;
    loop {
        if state.strm.avail_in == 0 as crate::stdlib::uInt
            && gz_avail(state) == -1 as ::core::ffi::c_int
        {
            ret = state.err;
            break;
        } else if state.strm.avail_in == 0 as crate::stdlib::uInt {
            if state.again == 0 {
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_BUF_ERROR,
                    Some(b"unexpected end of file\0"),
                );
            }
            break;
        } else {
            // `gz_look` initialized this stream and its input/output ranges
            // are owned by the validated gzip state for this call.
            ret = crate::src::inflate::inflate(&mut state.strm, crate::zlib_h::Z_NO_FLUSH);
            match crate::src::gzlib::gz_decomp_after_inflate(state, had, ret) {
                crate::src::gzlib::GzDecompStep::Continue => {}
                crate::src::gzlib::GzDecompStep::Stop(result) => {
                    ret = result;
                    break;
                }
                crate::src::gzlib::GzDecompStep::StreamError => {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(b"internal error: inflate stream corrupt\0"),
                    );
                    break;
                }
                crate::src::gzlib::GzDecompStep::MemError => {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_MEM_ERROR,
                        Some(b"out of memory\0"),
                    );
                    break;
                }
                crate::src::gzlib::GzDecompStep::DataError => {
                    let message = crate::src::inflate::inflate_error_message(&state.strm)
                        .unwrap_or(b"compressed data error\0");
                    crate::src::gzlib::gz_error(state, crate::zlib_h::Z_DATA_ERROR, Some(message));
                    break;
                }
            }
        }
    }
    crate::src::gzlib::gz_decomp_publish_output(state, had);
    return if crate::src::gzlib::gz_decomp_finish(state, ret) {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}

// The fetch state machine only coordinates an already-bound gzip state. Raw
// I/O and buffer access remain confined to `gz_look`, `gz_load`, and
// `gz_decomp`.
fn gz_fetch(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    loop {
        match crate::src::gzlib::gz_fetch_plan(state) {
            crate::src::gzlib::GzFetchPlan::Look => {
                if gz_look(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                if state.how == crate::gzguts_h::LOOK {
                    return 0 as ::core::ffi::c_int;
                }
            }
            crate::src::gzlib::GzFetchPlan::Copy { requested } => {
                // COPY mode is reached after `gz_look()` has allocated the
                // gzip output buffer. Borrow that owned allocation rather
                // than passing its C-facing cursor through this coordinator.
                let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
                let result = crate::src::gzlib::gz_with_owned_output_buffer(state_key, |output| {
                    if state.out != output.as_mut_ptr() || requested as usize > output.len() {
                        return None;
                    }
                    Some(gz_load(state, &mut output[..requested as usize]))
                });
                let Some(Some(result)) = result else {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(b"state corrupt\0"),
                    );
                    return -1;
                };
                crate::src::gzlib::gz_fetch_copy_loaded(state, result.received);
                if result.status == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                return 0 as ::core::ffi::c_int;
            }
            crate::src::gzlib::GzFetchPlan::Gzip { output } => {
                crate::src::gzlib::gz_fetch_prepare_decompression(state, output);
                if gz_decomp(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            crate::src::gzlib::GzFetchPlan::Corrupt => {
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(b"state corrupt\0"),
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if !crate::src::gzlib::gz_fetch_needs_more(state) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

// Consume an already-buffered portion of a seek.  The callers retain the raw
// fetch operation when more input must be produced.
fn gz_skip_buffered(state: &mut crate::gzguts_h::gz_state) -> bool {
    if state.x.have == 0 {
        return false;
    }
    let n = gz_consume(state, state.skip);
    state.skip -= n as crate::stdlib::off64_t;
    true
}

// Complete a deferred forward seek before a read-facing operation.  This is
// only a state-machine coordinator: `gz_fetch` retains the raw I/O and buffer
// work needed when the seek outgrows the bytes already buffered.
fn gz_finish_skip(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    while state.skip != 0 {
        if gz_skip_buffered(state) {
            continue;
        }
        if !crate::src::gzlib::gz_skip_needs_fetch(state) {
            break;
        }
        if gz_fetch(state) == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
    }
    0
}

// Read-side entry points that operate on an already-bound state share this
// state-only validation and ordinary-error reset. Caller-buffer access and
// buffer-pointer movement remain in their respective adapters.
fn gz_begin_read_operation(state: &mut crate::gzguts_h::gz_state) -> bool {
    if !crate::src::gzlib::gz_read_state_is_usable(state) {
        return false;
    }
    crate::src::gzlib::gz_clear_read_error(state);
    true
}

// Operations that must complete a deferred seek use the common entry setup
// above before advancing the read state machine.
fn gz_prepare_read_operation(state: &mut crate::gzguts_h::gz_state) -> bool {
    if !gz_begin_read_operation(state) {
        return false;
    }
    gz_finish_skip(state) != -1 as ::core::ffi::c_int
}

pub(crate) fn gz_consume(
    state: &mut crate::gzguts_h::gz_state,
    limit: crate::stdlib::off64_t,
) -> ::core::ffi::c_uint {
    let n = crate::src::gzlib::gz_skip_chunk(state.x.have, limit);
    state.x.have = state.x.have.wrapping_sub(n);
    state.x.next = state.x.next.wrapping_add(n as usize);
    crate::src::gzlib::gz_advance_pos(state, n);
    n
}

// Public entry points have already checked and bound the gzip state and
// caller buffer. Keep this internal reader reference-bound; its only raw
// boundary is copying from gzip's owned output buffer.
fn gz_read(
    state: &mut crate::gzguts_h::gz_state,
    buf: &mut [::core::ffi::c_uchar],
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut len = buf.len() as crate::stdlib::z_size_t;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if gz_finish_skip(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    's_140: loop {
        let mut consumed_buffered = false;
        match crate::src::gzlib::gz_read_plan(state, len) {
            crate::src::gzlib::GzReadPlan::Buffered(chunk) => {
                n = chunk;
                // The fetched bytes live in the registry-owned output Vec.
                // Keep its cursor-range check and the caller copy within that
                // one bounded borrow instead of rebuilding a raw source slice.
                let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
                let copied = crate::src::gzlib::gz_with_owned_output_buffer(state_key, |output| {
                    if state.out != output.as_mut_ptr() {
                        return false;
                    }
                    let Some(source) = gz_buffered_input_range(
                        output.as_ptr().addr(),
                        state.x.next.addr(),
                        n,
                        output.len(),
                    ) else {
                        return false;
                    };
                    let end = match (got as usize).checked_add(n as usize) {
                        Some(end) if end <= buf.len() => end,
                        _ => return false,
                    };
                    buf[got as usize..end].copy_from_slice(&output[source]);
                    true
                });
                if copied != Some(true) {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(b"state corrupt\0"),
                    );
                    err = -1;
                    break 's_140;
                }
                n = gz_consume(state, n as crate::stdlib::off64_t);
                consumed_buffered = true;
                if state.err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
            }
            crate::src::gzlib::GzReadPlan::End => break 's_140,
            crate::src::gzlib::GzReadPlan::Fetch => {
                // Fetch only fills gzip's internal output buffer.  It has not
                // yet copied a byte to the caller, so retry before accounting.
                match crate::src::gzlib::gz_read_after_fetch(gz_fetch(state), state.x.have) {
                    crate::src::gzlib::GzReadFetchResult::Retry => continue 's_140,
                    crate::src::gzlib::GzReadFetchResult::Error => {
                        err = -1 as ::core::ffi::c_int;
                        break 's_140;
                    }
                }
            }
            crate::src::gzlib::GzReadPlan::Copy(chunk) => {
                n = chunk;
                let result = gz_load(
                    state,
                    &mut buf[got as usize..got.wrapping_add(n as crate::stdlib::z_size_t) as usize],
                );
                n = result.received;
                err = result.status;
            }
            crate::src::gzlib::GzReadPlan::Decompress(chunk) => {
                // Bulk reads used to direct the inflater at `buf`.  Keep the
                // decoder's output in gzip's registry-owned allocation
                // instead, then copy the produced prefix to the already
                // bound caller slice.  This leaves the inflater with only
                // its own stable output cursor and makes the caller buffer a
                // normal slice-only concern of this read coordinator.
                let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
                let requested =
                    crate::src::gzlib::gz_with_owned_output_buffer(state_key, |output| {
                        (state.out == output.as_mut_ptr()).then(|| output.len().min(chunk as usize))
                    })
                    .flatten();
                let Some(requested) =
                    requested.and_then(|len| ::core::ffi::c_uint::try_from(len).ok())
                else {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(b"state corrupt\0"),
                    );
                    err = -1;
                    break 's_140;
                };
                if requested == 0 {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(b"state corrupt\0"),
                    );
                    err = -1;
                    break 's_140;
                }
                state.strm.avail_out = requested as crate::stdlib::uInt;
                state.strm.next_out = state.out;
                err = gz_decomp(state);
                n = crate::src::gzlib::gz_read_take_decompressed(state);
                let copied = crate::src::gzlib::gz_with_owned_output_buffer(state_key, |output| {
                    if state.out != output.as_mut_ptr() || n > requested {
                        return false;
                    }
                    let Some(source) = gz_buffered_input_range(
                        output.as_ptr().addr(),
                        state.x.next.addr(),
                        n,
                        output.len(),
                    ) else {
                        return false;
                    };
                    let end = match (got as usize).checked_add(n as usize) {
                        Some(end) if end <= buf.len() => end,
                        _ => return false,
                    };
                    buf[got as usize..end].copy_from_slice(&output[source]);
                    true
                });
                if copied != Some(true) {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(b"state corrupt\0"),
                    );
                    err = -1;
                    break 's_140;
                }
            }
        }
        crate::src::gzlib::gz_read_progress(state, &mut len, &mut got, n, consumed_buffered);
        if !crate::src::gzlib::gz_read_should_continue(len, err) {
            break;
        }
    }
    crate::src::gzlib::gz_read_mark_past(state, len);
    return got;
}
// This token is created only by the safe request dispatcher. Its concrete
// length is therefore suitable for the ABI adapter to bind, while rejected
// requests never carry a caller range to bind.
enum GzReadRequest {
    Rejected,
    Bytes(usize),
}

// Read request validation must precede binding an FFI caller range: an
// unusable state, or a request that Rust cannot represent, is not allowed to
// inspect that range. The dispatcher records rejected-request errors before
// either the Rust-facing or ABI-facing adapter considers caller memory.
fn gzread_dispatch(
    state: &mut crate::gzguts_h::gz_state,
    len: ::core::ffi::c_uint,
) -> GzReadRequest {
    match gzread_request_len(state, len) {
        Ok(len) => GzReadRequest::Bytes(len),
        Err(()) => GzReadRequest::Rejected,
    }
}

// The public coordinator owns buffer-length validation after the safe
// dispatcher has classified the request.
fn gzread(
    state: &mut crate::gzguts_h::gz_state,
    request: GzReadRequest,
    buffer: Option<&mut [::core::ffi::c_uchar]>,
) -> ::core::ffi::c_int {
    let slice_len = match request {
        GzReadRequest::Bytes(slice_len) => slice_len,
        GzReadRequest::Rejected => return -1,
    };
    let Some(buf) = buffer else {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in a Rust slice\0"),
        );
        return -1 as ::core::ffi::c_int;
    };
    if buf.len() != slice_len {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not match caller buffer\0"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let read = gz_read(state, buf) as ::core::ffi::c_uint;
    match crate::src::gzlib::gz_read_result(read, state.err, state.again) {
        crate::src::gzlib::GzReadResult::Count => {}
        crate::src::gzlib::GzReadResult::Error => return -1 as ::core::ffi::c_int,
        crate::src::gzlib::GzReadResult::WouldBlock => {
            let message = crate::src::gzlib::gz_errno_message();
            crate::src::gzlib::gz_error(state, crate::zlib_h::Z_ERRNO, Some(&message));
            return -1 as ::core::ffi::c_int;
        }
    }
    read as ::core::ffi::c_int
}

// State validation and request classification happen in `gzread_dispatch()`
// before the coordinator consumes any caller range.
fn gzread_request_len(
    state: &mut crate::gzguts_h::gz_state,
    len: ::core::ffi::c_uint,
) -> Result<usize, ()> {
    if !gz_begin_read_operation(state) {
        return Err(());
    }
    if !crate::src::gzlib::gz_uint_request_fits_int(len) {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in an int\0"),
        );
        return Err(());
    }
    let Some(slice_len) = crate::src::gzlib::gz_rust_slice_len(len as crate::stdlib::z_size_t)
    else {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in a Rust slice\0"),
        );
        return Err(());
    };
    Ok(slice_len)
}

// Byte reads neither close the handle nor invoke a user callback.  Resolve
// the opaque address through the owned-state registry, keeping the stateful
// request classification and result handling reference-bound.
fn gzread_handle(
    file_key: usize,
    len: ::core::ffi::c_uint,
    buffer: Option<&mut [::core::ffi::c_uchar]>,
) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| {
        let request = gzread_dispatch(state, len);
        gzread(state, request, buffer)
    })
    .unwrap_or(-1)
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
    // SAFETY: C's `gzread` contract supplies a writable `len`-byte range
    // for a non-null buffer.  Stateful validation remains in
    // `gzread_handle()` after registry lookup.
    let buffer = match crate::src::gzlib::gz_uint_request_fits_int(len)
        .then(|| crate::src::gzlib::gz_rust_slice_len(len as crate::stdlib::z_size_t))
        .flatten()
    {
        Some(0) => Some(&mut [] as &mut [::core::ffi::c_uchar]),
        Some(slice_len) if !buf.is_null() => Some(unsafe {
            ::core::slice::from_raw_parts_mut(
                buf as *mut ::core::ffi::c_uchar,
                slice_len,
            )
        }),
        _ => None,
    };
    gzread_handle(file.addr(), len, buffer)
}

// The item-read analogue of `GzReadRequest`; empty requests deliberately do
// not need a caller buffer.
enum GzItemReadRequest {
    Rejected,
    Empty,
    Bytes(usize),
}

// Decide whether an item request is usable before asking an FFI caller to
// provide a slice. This keeps state and overflow classification in the
// implementation dispatcher.
fn gzfread_dispatch(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> GzItemReadRequest {
    match gzfread_request_len(state, size, nitems) {
        Ok(None) => GzItemReadRequest::Empty,
        Ok(Some(len)) => GzItemReadRequest::Bytes(len),
        Err(()) => GzItemReadRequest::Rejected,
    }
}

// This public coordinator retains buffer-length validation after the safe
// dispatcher has classified the request.
fn gzfread(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    request: GzItemReadRequest,
    buffer: Option<&mut [::core::ffi::c_uchar]>,
) -> crate::stdlib::z_size_t {
    let slice_len = match request {
        GzItemReadRequest::Bytes(slice_len) => slice_len,
        GzItemReadRequest::Empty | GzItemReadRequest::Rejected => {
            return 0 as crate::stdlib::z_size_t
        }
    };
    match buffer {
        Some(buf) if buf.len() == slice_len => gzfread_bound(state, size, buf),
        _ => {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(b"request does not fit in a Rust slice\0"),
            );
            0 as crate::stdlib::z_size_t
        }
    }
}

// This is the post-validation item-read path. Item request classification,
// including the empty-request case, stays with the request coordinator and
// is not repeated after an FFI adapter binds memory.
fn gzfread_bound(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    buf: &mut [::core::ffi::c_uchar],
) -> crate::stdlib::z_size_t {
    gz_read(state, buf).wrapping_div(size)
}

// Unlike byte reads, item reads can be empty without binding a caller range.
// Keep that distinction and all rejected-request error recording in this safe
// coordinator before either adapter considers its buffer.
fn gzfread_request_len(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Result<Option<usize>, ()> {
    if !gz_begin_read_operation(state) {
        return Err(());
    }
    match crate::src::gzlib::gz_item_request(size, nitems) {
        crate::src::gzlib::GzItemRequest::Empty => Ok(None),
        crate::src::gzlib::GzItemRequest::TooLarge => {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(b"request does not fit in a size_t\0"),
            );
            Err(())
        }
        crate::src::gzlib::GzItemRequest::Bytes(len) => {
            let Some(slice_len) = crate::src::gzlib::gz_rust_slice_len(len) else {
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(b"request does not fit in a Rust slice\0"),
                );
                return Err(());
            };
            Ok(Some(slice_len))
        }
    }
}

// Item reads do not close the handle or invoke user callbacks. Resolve the
// opaque handle through its owned-state registry so the exported ABI adapter
// only needs to bind the caller's destination range.
fn gzfread_handle(
    file_key: usize,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
    buffer: Option<&mut [::core::ffi::c_uchar]>,
) -> crate::stdlib::z_size_t {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| {
        let request = gzfread_dispatch(state, size, nitems);
        gzfread(state, size, request, buffer)
    })
    .unwrap_or(0)
}

#[export_name = "gzfread"]

pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    if file.is_null() {
        return 0;
    }
    // Classify only the caller-controlled byte count before binding its
    // range. The stateful request preflight, including its error record,
    // remains in `gzfread_handle()` after registry lookup succeeds.
    let buffer = match crate::src::gzlib::gz_item_request(size, nitems) {
        crate::src::gzlib::GzItemRequest::Bytes(len) if !buf.is_null() => {
            let Some(slice_len) = crate::src::gzlib::gz_rust_slice_len(len) else {
                return gzfread_handle(file.addr(), size, nitems, None);
            };
            // SAFETY: C's `gzfread` contract supplies a writable range for
            // its non-null buffer and requested item count.
            Some(unsafe {
                ::core::slice::from_raw_parts_mut(buf as *mut ::core::ffi::c_uchar, slice_len)
            })
        }
        _ => None,
    };
    gzfread_handle(file.addr(), size, nitems, buffer)
}
// Reading one byte through `gz_read` preserves the buffered and unbuffered
// paths' cursor and EOF bookkeeping while keeping the internal buffer access
// in that reader's existing raw-copy boundary.
pub fn gzgetc(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    if !gz_begin_read_operation(state) {
        return -1 as ::core::ffi::c_int;
    }
    return if gz_read(state, &mut buf) < 1 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    };
}

// Keep the null-handle result policy with the reference-based operation, so
// the ABI entries below only bind their opaque C handle.
fn gzgetc_dispatch(state: Option<&mut crate::gzguts_h::gz_state>) -> ::core::ffi::c_int {
    state.map_or(-1, gzgetc)
}

// Reading does not close the handle or invoke user callbacks, so the owned
// state registry can keep the opaque handle valid for the full operation.
fn gzgetc_handle(file_key: usize) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| gzgetc_dispatch(Some(state)))
        .unwrap_or(-1)
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc_handle(file.addr())
}
pub fn gzgetc_(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    gzgetc(state)
}

fn gzgetc__dispatch(state: Option<&mut crate::gzguts_h::gz_state>) -> ::core::ffi::c_int {
    state.map_or(-1, gzgetc_)
}

fn gzgetc__handle(file_key: usize) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| gzgetc__dispatch(Some(state)))
        .unwrap_or(-1)
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc__handle(file.addr())
}
// Classify the push-back request before the ABI adapter binds the output
// allocation. In particular, a failed initial `gz_look()` must not make the
// adapter construct a slice from an unallocated `out` pointer.
fn gzungetc_dispatch(
    c: ::core::ffi::c_int,
    state: &mut crate::gzguts_h::gz_state,
) -> Option<crate::src::gzlib::GzUngetcPlan> {
    if !crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_READ) {
        return None;
    }
    if crate::src::gzlib::gz_ungetc_needs_look(state) {
        gz_look(state);
    }
    if !gz_prepare_read_operation(state) {
        return None;
    }
    if c < 0 as ::core::ffi::c_int {
        return None;
    }
    match crate::src::gzlib::gz_ungetc_plan(state) {
        crate::src::gzlib::GzUngetcPlan::Full => {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_DATA_ERROR,
                Some(b"out of room to push characters\0"),
            );
            None
        }
        plan => Some(plan),
    }
}

// Once `gzungetc_dispatch()` has accepted the request, the implementation
// borrows the initialized owned output allocation. The cursor movement and
// overlap handling below are therefore entirely safe slice operations.
fn gzungetc_with_output(
    c: ::core::ffi::c_int,
    state: &mut crate::gzguts_h::gz_state,
    output: &mut [::core::ffi::c_uchar],
    plan: crate::src::gzlib::GzUngetcPlan,
) -> ::core::ffi::c_int {
    match plan {
        crate::src::gzlib::GzUngetcPlan::First { buffer_end } => {
            let byte = buffer_end as usize - 1;
            output[byte] = c as ::core::ffi::c_uchar;
            state.x.next = state.out.wrapping_add(byte);
            crate::src::gzlib::gz_ungetc_progress(state, true);
        }
        crate::src::gzlib::GzUngetcPlan::Prepend { move_to_end } => {
            if let Some(move_to_end) = move_to_end {
                let source_len = move_to_end.source_len as usize;
                let destination_end = move_to_end.destination_offset as usize;
                let destination_start = destination_end - source_len;
                output.copy_within(0..source_len, destination_start);
                state.x.next = state.out.wrapping_add(destination_start);
            }
            let next = (state.x.next as usize).wrapping_sub(state.out as usize);
            let byte = next - 1;
            output[byte] = c as ::core::ffi::c_uchar;
            state.x.next = state.out.wrapping_add(byte);
            crate::src::gzlib::gz_ungetc_progress(state, false);
        }
        // `gzungetc_dispatch()` rejects this before the ABI binder constructs
        // an output slice.
        crate::src::gzlib::GzUngetcPlan::Full => unreachable!(),
    }
    c
}

// This implementation owns the lookahead, state validation, and owned output
// buffer lookup. The exported wrapper need only bind its opaque C handle.
fn gzungetc(c: ::core::ffi::c_int, state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let Some(plan) = gzungetc_dispatch(c, state) else {
        return -1;
    };
    let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
    crate::src::gzlib::gz_with_owned_output_buffer(state_key, |output| {
        gzungetc_with_output(c, state, output, plan)
    })
    .unwrap_or(-1)
}

// The owned output-buffer lookup remains inside `gzungetc()`.  This outer
// registry borrow only resolves the opaque handle for this non-closing call.
fn gzungetc_handle(file_key: usize, c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| gzungetc(c, state)).unwrap_or(-1)
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    gzungetc_handle(file.addr(), c)
}
// The ABI wrapper binds the caller's writable string once. Reuse `gz_read()`
// for each byte, so this line reader shares the existing bounded handling of
// gzip's owned output buffer instead of binding that raw cursor a second time.
fn gzgets(state: &mut crate::gzguts_h::gz_state, buf: &mut [::core::ffi::c_char]) -> bool {
    let mut written = 0usize;
    if !gz_prepare_read_operation(state) {
        return false;
    }
    let limit = buf.len().saturating_sub(1);
    while written < limit {
        let mut byte = [0u8; 1];
        if gz_read(state, &mut byte) == 0 {
            break;
        }
        buf[written] = byte[0] as ::core::ffi::c_char;
        written += 1;
        if byte[0] == b'\n' {
            break;
        }
    }
    // zlib returns an empty, terminated string for a one-byte destination.
    if written == 0 && limit != 0 {
        return false;
    }
    buf[written] = 0 as ::core::ffi::c_char;
    true
}

// Keep the gzip handle and read-mode preflight out of the FFI adapter. That
// adapter only converts its caller-owned destination, while this dispatcher
// decides whether the operation may proceed.
fn gzgets_ffi_dispatch(
    state: Option<&mut crate::gzguts_h::gz_state>,
    destination: Option<&mut [::core::ffi::c_char]>,
) -> bool {
    let Some(state) = state else {
        return false;
    };
    if !crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_READ) {
        return false;
    }
    let Some(destination) = destination else {
        return false;
    };
    gzgets(state, destination)
}

fn gzgets_handle(
    file_key: usize,
    destination: Option<&mut [::core::ffi::c_char]>,
) -> bool {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| {
        gzgets_ffi_dispatch(Some(state), destination)
    })
    .unwrap_or(false)
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let destination = if file.is_null() || buf.is_null() || len < 1 {
        None
    } else {
        // SAFETY: C's `gzgets` contract supplies a writable `len`-byte
        // buffer. This ABI adapter performs the raw conversion; the
        // dispatcher owns gzip-state validation and the read operation.
        Some(unsafe { ::core::slice::from_raw_parts_mut(buf, len as usize) })
    };
    if gzgets_handle(file.addr(), destination) {
        buf
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }
}
// Direct-mode querying only needs a validated, bound state. `gz_look` keeps
// its allocation and descriptor boundaries scoped inside that coordinator.
fn gzdirect(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if crate::src::gzlib::gz_direct_needs_look(state) {
        gz_look(state);
    }
    return (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}

fn gzdirect_dispatch(state: Option<&mut crate::gzguts_h::gz_state>) -> ::core::ffi::c_int {
    state.map_or(0, gzdirect)
}

fn gzdirect_handle(file_key: usize) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| gzdirect_dispatch(Some(state)))
        .unwrap_or(0)
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzdirect_handle(file.addr())
}

// Closing a read stream has a small state-only tail after its raw inflater,
// allocation, and descriptor cleanup. Keep the error result selection and
// error-record reset reference-bound so the exported cleanup boundary does
// not also own ordinary gzip state transitions.
fn gz_close_read_finish(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let err = if state.err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gzclearerr(state);
    err
}

// The close dispatcher has already validated and bound `file` to `state`.
// Keep its mode/error decisions safe; inflater teardown and descriptor
// closing remain at the narrow raw cleanup boundary below.
pub fn gzclose_r(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    if !crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_READ) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let cleanup = crate::src::gzlib::gz_read_close_cleanup(state);
    let release_buffers = matches!(&cleanup, crate::src::gzlib::GzReadCloseCleanup::Inflater);
    match cleanup {
        crate::src::gzlib::GzReadCloseCleanup::None => {}
        crate::src::gzlib::GzReadCloseCleanup::Inflater => {
            crate::src::inflate::inflate_end_default_bound(&mut state.strm);
        }
    }
    if release_buffers {
        crate::src::gzlib::gz_release_owned_buffers(state);
    }
    err = gz_close_read_finish(state);
    let fd = state.fd;
    // The result of closing the descriptor intentionally overrides the
    // earlier buffered-error result, matching zlib's cleanup order.
    crate::src::gzlib::gz_release_owned_strings(state);
    ret = crate::stdlib::close(fd);
    crate::src::gzlib::gz_release_owned_state(state);
    return if ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    };
}

fn gzclose_r_dispatch(state: Option<&mut crate::gzguts_h::gz_state>) -> ::core::ffi::c_int {
    state.map_or(crate::zlib_h::Z_STREAM_ERROR, gzclose_r)
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    // The address-keyed registry owns every live gzip state. Taking the box
    // makes close reference-bound without dereferencing the foreign handle.
    // `gzclose_r()`'s legacy registry-release step is then a harmless no-op.
    let Some(mut state) = crate::src::gzlib::gz_take_owned_state_with_mode(
        file.addr(),
        crate::gzguts_h::GZ_READ,
    ) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzclose_r(&mut state)
}
