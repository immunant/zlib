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

struct GzLoadResult {
    received: ::core::ffi::c_uint,
    status: ::core::ffi::c_int,
}

// This is the raw descriptor-read boundary. Its private callers either pass a
// range in gzip's initialized buffers or an FFI caller buffer whose validity
// was checked by their unsafe entry point. Keep the byte count in a typed
// result instead of passing a raw out-pointer through each caller, including
// the partial-read error case that gzip must still account for.
fn gz_load(
    state: &mut crate::gzguts_h::gz_state,
    mut buf: *mut ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
) -> GzLoadResult {
    let mut ret: ::core::ffi::c_int = 0;
    let mut get: ::core::ffi::c_uint = 0;
    // Keep the byte count local while crossing the raw read boundary.  The
    // caller only observes it after the descriptor result has been classified.
    let mut loaded: ::core::ffi::c_uint = 0;
    // SAFETY: the callers maintain the buffer validity described above for
    // the whole requested range. This is the only place the read boundary
    // dereferences errno or passes that range to the descriptor API.
    unsafe {
        crate::src::gzlib::gz_begin_io(state);
        *crate::stdlib::__errno_location() = 0;
        loop {
            get = crate::src::gzlib::gz_load_request(len, loaded);
            ret = crate::stdlib::read(
                state.fd,
                buf.wrapping_add(loaded as usize) as *mut ::core::ffi::c_void,
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
        if let Err(errno) = crate::src::gzlib::gz_load_result(
            state,
            ret,
            loaded,
            *crate::stdlib::__errno_location(),
        ) {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_ERRNO,
                Some(::core::ffi::CStr::from_ptr(crate::stdlib::strerror(errno)).to_bytes_with_nul()),
            );
            return GzLoadResult {
                received: loaded,
                status: -1,
            };
        }
    }
    GzLoadResult {
        received: loaded,
        status: 0,
    }
}

// This helper is internal and all of its callers have already bound the
// validated gzip state. Only input-buffer compaction needs a raw operation;
// descriptor I/O remains confined to `gz_load`.
fn gz_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut got: ::core::ffi::c_uint = 0;
    let plan = match crate::src::gzlib::gz_avail_plan(state) {
        Ok(plan) => plan,
        Err(()) => return -1,
    };
    if let crate::src::gzlib::GzAvailPlan::Load {
        buffered,
        requested,
    } = plan
    {
        if buffered != 0 {
            let input = state.strm.next_in;
            if crate::src::gzlib::gz_avail_needs_compaction(
                buffered,
                input == state.in_0,
            ) {
                // `next_in` points into the input buffer, so the source and
                // destination may overlap.  `copy` preserves the translated
                // forward-copy behavior for that compaction.
                // SAFETY: `next_in` and `in_0` identify ranges within the
                // initialized gzip input buffer. They may overlap, which is
                // why this preserves the C implementation's `memmove`-like
                // compaction operation.
                unsafe { ::core::ptr::copy(input, state.in_0, buffered as usize) };
            }
        }
        let result = gz_load(state, state.in_0.wrapping_add(buffered as usize), requested);
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
        // SAFETY: this state has not allocated its gzip buffers yet. These
        // allocations, cleanup calls, and inflater initialization all use
        // the fields configured here; `gz_error` updates this same state.
        unsafe {
            state.in_0 = crate::stdlib::malloc(state.want as crate::__stddef_size_t_h::size_t)
                as *mut ::core::ffi::c_uchar;
            state.out = crate::stdlib::malloc(
                (state.want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
            ) as *mut ::core::ffi::c_uchar;
            if state.in_0.is_null() || state.out.is_null() {
                crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_MEM_ERROR,
                    Some(b"out of memory\0"),
                );
                return -1 as ::core::ffi::c_int;
            }
            gz_look_prepare_stream(state);
            if crate::src::inflate::inflateInit2_(
                &mut state.strm,
                15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
                crate::zlib_h::ZLIB_VERSION.as_ptr(),
                ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
            ) != crate::zlib_h::Z_OK
            {
                crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
                gz_look_init_failed(state);
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_MEM_ERROR,
                    Some(b"out of memory\0"),
                );
                return -1 as ::core::ffi::c_int;
            }
        }
    }
    if state.direct == -1 as ::core::ffi::c_int || state.junk == 0 as ::core::ffi::c_int {
        // SAFETY: initialization above, or the existing read state, provides
        // the live inflater stream required by this reset.
        unsafe { crate::src::inflate::inflateReset(&mut state.strm) };
        crate::src::gzlib::gz_set_gzip_input(state, state.junk != -1 as ::core::ffi::c_int);
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let available = state.strm.avail_in;
    let gzip_header = if available > 3 {
        // SAFETY: `gz_avail` maintains `next_in` within the initialized input
        // buffer and `available > 3` makes these four header bytes readable.
        unsafe {
            let input = state.strm.next_in;
            crate::src::gzlib::gz_is_gzip_header([
                *input,
                *input.wrapping_add(1),
                *input.wrapping_add(2),
                *input.wrapping_add(3),
            ])
        }
    } else {
        false
    };
    match crate::src::gzlib::gz_look_plan(available, state.again != 0, gzip_header) {
        crate::src::gzlib::GzLookPlan::NeedMore => return 0 as ::core::ffi::c_int,
        crate::src::gzlib::GzLookPlan::Gzip => {
            // SAFETY: `gz_look` has initialized the stream before classifying
            // a gzip member, so resetting it is valid here.
            unsafe { crate::src::inflate::inflateReset(&mut state.strm) };
            crate::src::gzlib::gz_set_gzip_input(state, true);
            return 0 as ::core::ffi::c_int;
        }
        crate::src::gzlib::GzLookPlan::Copy { copied } => {
            // SAFETY: `gz_avail` has made `copied` input bytes available, and
            // `out` was allocated with twice the input-buffer capacity. The
            // ranges are distinct gzip buffers.
            unsafe {
                crate::stdlib::memcpy(
                    state.out as *mut ::core::ffi::c_void,
                    state.strm.next_in as *const ::core::ffi::c_void,
                    copied as crate::__stddef_size_t_h::size_t,
                );
            }
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
            // SAFETY: `gz_look` initialized this stream and its input/output
            // ranges are owned by the validated gzip state for this call.
            ret = unsafe {
                crate::src::inflate::inflate(&mut state.strm, crate::zlib_h::Z_NO_FLUSH)
            };
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
                    unsafe {
                        crate::src::gzlib::gz_error(
                            state,
                            crate::zlib_h::Z_DATA_ERROR,
                            if state.strm.msg.is_null() {
                                Some(b"compressed data error\0" as &[u8])
                            } else {
                                Some(::core::ffi::CStr::from_ptr(state.strm.msg).to_bytes_with_nul())
                            },
                        );
                    }
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
                let result = gz_load(state, state.out, requested);
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
                // SAFETY: the public reader entry point supplied a writable
                // caller buffer of the requested length, and `x.next` plus
                // `x.have` identifies the initialized internal output range.
                // `gz_read_plan` bounds this copy by both ranges.
                unsafe {
                    crate::stdlib::memcpy(
                        buf[got as usize..].as_mut_ptr() as *mut ::core::ffi::c_void,
                        state.x.next as *const ::core::ffi::c_void,
                        n as crate::__stddef_size_t_h::size_t,
                    );
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
                let result = gz_load(state, buf[got as usize..].as_mut_ptr(), n);
                n = result.received;
                err = result.status;
            }
            crate::src::gzlib::GzReadPlan::Decompress(chunk) => {
                n = chunk;
                state.strm.avail_out = n as crate::stdlib::uInt;
                state.strm.next_out = buf[got as usize..].as_mut_ptr();
                err = gz_decomp(state);
                n = crate::src::gzlib::gz_read_take_decompressed(state);
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
// Read request validation must precede binding an FFI caller range: an
// unusable state, or a request that Rust cannot represent, is not allowed to
// inspect that range.  Both the Rust-facing and ABI-facing adapters share
// this coordinator.
pub fn gzread<'a, F>(
    state: &mut crate::gzguts_h::gz_state,
    len: ::core::ffi::c_uint,
    bind: F,
) -> ::core::ffi::c_int
where
    F: FnOnce(usize) -> Option<&'a mut [::core::ffi::c_uchar]>,
{
    let slice_len = match gzread_request_len(state, len) {
        Ok(slice_len) => slice_len,
        Err(()) => return -1 as ::core::ffi::c_int,
    };
    let buf = bind(slice_len);
    let Some(buf) = buf else {
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
            // SAFETY: the errno slot and strerror result are used only to
            // record this would-block error immediately in the bound state.
            unsafe {
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_ERRNO,
                    Some(::core::ffi::CStr::from_ptr(crate::stdlib::strerror(*crate::stdlib::__errno_location())).to_bytes_with_nul()),
                );
            }
            return -1 as ::core::ffi::c_int;
        }
    }
    read as ::core::ffi::c_int
}

// State validation and request classification must happen before any caller
// range is bound. Both the Rust-facing operation and FFI preflight use this
// same safe coordinator, leaving their adapters to handle only the buffer.
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
    let Some(slice_len) = crate::src::gzlib::gz_rust_slice_len(len as crate::stdlib::z_size_t) else {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in a Rust slice\0"),
        );
        return Err(());
    };
    Ok(slice_len)
}

// The FFI wrapper asks this safe adapter whether it may bind caller memory.
// It preserves the public error result while the request coordinator owns all
// state validation and rejected-request error recording.
fn gzread_preflight(
    state: &mut crate::gzguts_h::gz_state,
    len: ::core::ffi::c_uint,
) -> Result<usize, ::core::ffi::c_int> {
    gzread_request_len(state, len).map_err(|()| -1)
}

// Keep the public result mapping in safe code as well. The FFI entry point
// only converts the prepared byte count to a slice and dispatches here.
fn gzread_ffi_dispatch(
    state: &mut crate::gzguts_h::gz_state,
    len: ::core::ffi::c_uint,
    prepared: Result<usize, ::core::ffi::c_int>,
    buffer: Option<&mut [::core::ffi::c_uchar]>,
) -> ::core::ffi::c_int {
    match prepared {
        Ok(_) => gzread(state, len, |_| buffer),
        Err(result) => result,
    }
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
    let prepared = gzread_preflight(state, len);
    let buffer = match prepared {
        Ok(0) => Some(&mut [] as &mut [::core::ffi::c_uchar]),
        Ok(len) if !buf.is_null() => Some(::core::slice::from_raw_parts_mut(
            buf as *mut ::core::ffi::c_uchar,
            len,
        )),
        _ => None,
    };
    gzread_ffi_dispatch(state, len, prepared, buffer)
}

// As with `gzread`, decide whether an item request is usable
// before asking an FFI caller to provide a slice. This keeps all state and
// request semantics out of `gzfread_ffi`.
pub fn gzfread<'a, F>(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
    bind: F,
) -> crate::stdlib::z_size_t
where
    F: FnOnce(usize) -> Option<&'a mut [::core::ffi::c_uchar]>,
{
    let slice_len = match gzfread_request_len(state, size, nitems) {
        Ok(Some(slice_len)) => slice_len,
        Ok(None) | Err(()) => return 0 as crate::stdlib::z_size_t,
    };
    match bind(slice_len) {
        Some(buf) if buf.len() == slice_len => gz_read(state, buf).wrapping_div(size),
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

// Like `gzread_preflight`, this adapter exposes only a concrete slice length
// to the FFI wrapper. Empty requests remain zero-length and need no binding.
fn gzfread_preflight(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Result<usize, ()> {
    gzfread_request_len(state, size, nitems).map(|slice_len| slice_len.unwrap_or(0))
}

fn gzfread_ffi_dispatch(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
    prepared: Result<usize, ()>,
    buffer: Option<&mut [::core::ffi::c_uchar]>,
) -> crate::stdlib::z_size_t {
    match prepared {
        Ok(_) => gzfread(state, size, nitems, |_| buffer),
        Err(()) => 0,
    }
}
#[export_name = "gzfread"]

pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    let prepared = gzfread_preflight(state, size, nitems);
    let buffer = match prepared {
        Ok(0) => Some(&mut [] as &mut [::core::ffi::c_uchar]),
        Ok(len) if !buf.is_null() => Some(::core::slice::from_raw_parts_mut(
            buf as *mut ::core::ffi::c_uchar,
            len,
        )),
        _ => None,
    };
    gzfread_ffi_dispatch(state, size, nitems, prepared, buffer)
}
// Reading one byte through `gz_read` preserves the buffered and unbuffered
// paths' cursor and EOF bookkeeping while keeping the internal buffer access
// in that reader's existing raw-copy boundary.
pub fn gzgetc(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    if !gz_begin_read_operation(state) {
        return -1 as ::core::ffi::c_int;
    }
    return if gz_read(
        state,
        &mut buf,
    ) < 1 as crate::stdlib::z_size_t
    {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    };
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzgetc(&mut *(file as crate::gzguts_h::gz_statep))
}
pub fn gzgetc_(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    gzgetc(state)
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzgetc_(&mut *(file as crate::gzguts_h::gz_statep))
}
pub fn gzungetc(
    mut c: ::core::ffi::c_int,
    state: &mut crate::gzguts_h::gz_state,
) -> ::core::ffi::c_int {
    if !crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_READ) {
        return -1 as ::core::ffi::c_int;
    }
    if crate::src::gzlib::gz_ungetc_needs_look(state) {
        gz_look(state);
    }
    if !gz_prepare_read_operation(state) {
        return -1 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    match crate::src::gzlib::gz_ungetc_plan(state) {
        crate::src::gzlib::GzUngetcPlan::First { buffer_end } => {
            // SAFETY: the ungetc plan reserves the final byte of the
            // initialized output buffer for this first pushed-back byte.
            state.x.next = state
                .out
                .wrapping_add(buffer_end as usize)
                .wrapping_sub(1);
            unsafe {
                *state.x.next = c as ::core::ffi::c_uchar;
            }
            crate::src::gzlib::gz_ungetc_progress(state, true);
        }
        crate::src::gzlib::GzUngetcPlan::Full => {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_DATA_ERROR,
                Some(b"out of room to push characters\0"),
            );
            return -1 as ::core::ffi::c_int;
        }
        crate::src::gzlib::GzUngetcPlan::Prepend { move_to_end } => {
            // SAFETY: this plan is derived from the initialized output
            // buffer's available capacity. The backwards copy stays within
            // that buffer and preserves the translated overlapping move.
            if move_to_end {
                let mut src: *mut ::core::ffi::c_uchar =
                    state.out.wrapping_add(state.x.have as usize);
                let mut dest: *mut ::core::ffi::c_uchar = state
                    .out
                    .wrapping_add((state.size << 1 as ::core::ffi::c_int) as usize);
                unsafe {
                    while src > state.out {
                        src = src.wrapping_sub(1);
                        dest = dest.wrapping_sub(1);
                        *dest = *src;
                    }
                }
                state.x.next = dest;
            }
            state.x.next = state.x.next.wrapping_sub(1);
            unsafe {
                *state.x.next = c as ::core::ffi::c_uchar;
            }
            crate::src::gzlib::gz_ungetc_progress(state, false);
        }
    }
    c
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzungetc(c, &mut *(file as crate::gzguts_h::gz_statep))
}
// The ABI wrapper binds the caller's writable string once.  The read loop can
// then use a Rust slice for its cursor and terminator, leaving only the
// already-owned gzip output buffer as a raw boundary here.
fn gzgets(state: &mut crate::gzguts_h::gz_state, buf: &mut [::core::ffi::c_char]) -> bool {
    let mut left: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut written = 0usize;
    if !gz_prepare_read_operation(state) {
        return false;
    }
    left = crate::src::gzlib::gz_gets_remaining(buf.len() as ::core::ffi::c_int);
    if left != 0 {
        while !(state.x.have == 0 as ::core::ffi::c_uint
            && gz_fetch(state) == -1 as ::core::ffi::c_int)
        {
            match crate::src::gzlib::gz_gets_plan(state.x.have, left) {
                crate::src::gzlib::GzGetsPlan::Empty => {
                    crate::src::gzlib::gz_gets_mark_past(state);
                    break;
                }
                crate::src::gzlib::GzGetsPlan::Copy(chunk) => {
                    n = chunk;
                    // SAFETY: `gz_gets_plan()` bounds this view by `x.have`,
                    // whose initialized bytes begin at `x.next`.
                    let source = unsafe {
                        ::core::slice::from_raw_parts(state.x.next, n as usize)
                    };
                    let found_eol = if let Some(eol) = source.iter().position(|byte| *byte == b'\n') {
                        n = (eol as ::core::ffi::c_uint).wrapping_add(1);
                        true
                    } else {
                        false
                    };
                    for (destination, source) in buf[written..written + n as usize]
                        .iter_mut()
                        .zip(&source[..n as usize])
                    {
                        *destination = *source as ::core::ffi::c_char;
                    }
                    gz_consume(state, n as crate::stdlib::off64_t);
                    crate::src::gzlib::gz_gets_after_copy(&mut left, n);
                    written += n as usize;
                    if !crate::src::gzlib::gz_gets_should_continue(left, found_eol) {
                        break;
                    }
                }
            }
        }
    }
    if written == 0 {
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
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let state = if file.is_null() {
        None
    } else {
        // SAFETY: a non-null gzip handle identifies the state bound by this
        // ABI entry. Its read-mode preflight remains in the dispatcher.
        Some(unsafe { &mut *(file as crate::gzguts_h::gz_statep) })
    };
    let destination = if file.is_null() || buf.is_null() || len < 1 {
        None
    } else {
        // SAFETY: C's `gzgets` contract supplies a writable `len`-byte
        // buffer. This ABI adapter performs the raw conversion; the
        // dispatcher owns gzip-state validation and the read operation.
        Some(unsafe { ::core::slice::from_raw_parts_mut(buf, len as usize) })
    };
    if gzgets_ffi_dispatch(state, destination) {
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
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    gzdirect(&mut *(file as crate::gzguts_h::gz_statep))
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
// Keep its mode/error decisions safe; inflater teardown, allocation release,
// and descriptor closing remain at the narrow raw cleanup boundary below.
pub fn gzclose_r(
    state: &mut crate::gzguts_h::gz_state,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    if !crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_READ) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    match crate::src::gzlib::gz_read_close_cleanup(state) {
        crate::src::gzlib::GzReadCloseCleanup::None => {}
        crate::src::gzlib::GzReadCloseCleanup::Inflater => unsafe {
            // SAFETY: this close path owns the initialized inflater and gzip
            // buffers. The dispatcher bound `file` to this state, and no
            // allocation escapes after it is released.
            crate::src::inflate::inflateEnd(
                &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            );
            crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
        },
    }
    err = gz_close_read_finish(state);
    let path = state.path;
    let fd = state.fd;
    // SAFETY: `path`, `fd`, and `file` are owned by this closing state. The
    // result of closing the descriptor intentionally overrides the earlier
    // buffered-error result, matching zlib's cleanup order.
    unsafe {
        crate::stdlib::free(path as *mut ::core::ffi::c_void);
        ret = crate::stdlib::close(fd);
        crate::stdlib::free(file as *mut ::core::ffi::c_void);
    }
    return if ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    };
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    gzclose_r(&mut *(file as crate::gzguts_h::gz_statep), file)
}
