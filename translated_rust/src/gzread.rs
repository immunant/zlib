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
                crate::stdlib::strerror(errno),
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

// All callers have already validated and bound the gzip state.  Allocation
// and input-buffer access remain raw within this internal adapter.
unsafe fn gz_look(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size == 0 as ::core::ffi::c_uint {
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
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        state.size = state.want;
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        state.strm.avail_in = 0 as crate::stdlib::uInt;
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit2_(
            &mut state.strm,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            state.size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if state.direct == -1 as ::core::ffi::c_int || state.junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflateReset(&mut state.strm);
        crate::src::gzlib::gz_set_gzip_input(state, state.junk != -1 as ::core::ffi::c_int);
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let available = state.strm.avail_in;
    let gzip_header = if available > 3 {
        // Only the raw input adapter touches the untrusted input pointer.
        let input = state.strm.next_in;
        crate::src::gzlib::gz_is_gzip_header([
            *input,
            *input.wrapping_add(1),
            *input.wrapping_add(2),
            *input.wrapping_add(3),
        ])
    } else {
        false
    };
    match crate::src::gzlib::gz_look_plan(available, state.again != 0, gzip_header) {
        crate::src::gzlib::GzLookPlan::NeedMore => return 0 as ::core::ffi::c_int,
        crate::src::gzlib::GzLookPlan::Gzip => {
            crate::src::inflate::inflateReset(&mut state.strm);
            crate::src::gzlib::gz_set_gzip_input(state, true);
            return 0 as ::core::ffi::c_int;
        }
        crate::src::gzlib::GzLookPlan::Copy { copied } => {
            crate::stdlib::memcpy(
                state.out as *mut ::core::ffi::c_void,
                state.strm.next_in as *const ::core::ffi::c_void,
                copied as crate::__stddef_size_t_h::size_t,
            );
            crate::src::gzlib::gz_set_copy_input(state, copied);
            return 0 as ::core::ffi::c_int;
        }
    }
}

// The decompressor is likewise internal to the read state machine.  Its
// inflater and output-buffer adapters remain raw, but the state itself is
// reference-bound by every caller.
unsafe fn gz_decomp(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
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
                    b"unexpected end of file\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            break;
        } else {
            ret = crate::src::inflate::inflate(
                &mut state.strm,
                crate::zlib_h::Z_NO_FLUSH,
            );
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
                        b"internal error: inflate stream corrupt\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    break;
                }
                crate::src::gzlib::GzDecompStep::MemError => {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_MEM_ERROR,
                        b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    break;
                }
                crate::src::gzlib::GzDecompStep::DataError => {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_DATA_ERROR,
                        if state.strm.msg.is_null() {
                            b"compressed data error\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            state.strm.msg as *const ::core::ffi::c_char
                        },
                    );
                    break;
                }
            }
        }
    }
    state.x.have =
        (had as crate::stdlib::uInt).wrapping_sub(state.strm.avail_out) as ::core::ffi::c_uint;
    state.x.next = state.strm.next_out.wrapping_sub(state.x.have as usize);
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
                // SAFETY: `state` is the validated read-state reference
                // passed to this coordinator; `gz_look` owns its raw buffer
                // and allocation boundary.
                if unsafe { gz_look(state) } == -1 as ::core::ffi::c_int {
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
                state.strm.avail_out = output as crate::stdlib::uInt;
                state.strm.next_out = state.out;
                // SAFETY: `state` remains the validated read-state reference;
                // `gz_decomp` owns the inflater and output-buffer boundary.
                if unsafe { gz_decomp(state) } == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            crate::src::gzlib::GzFetchPlan::Corrupt => {
                // SAFETY: the validated state reference is also the state
                // whose error ownership `gz_error` updates.
                unsafe {
                    crate::src::gzlib::gz_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"state corrupt\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
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
        if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        if gz_fetch(state) == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
    }
    0
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

// Public entry points have already checked and bound the gzip state.  Keep
// this internal reader reference-bound; the caller buffer remains its only
// raw input boundary.
unsafe fn gz_read(
    state: &mut crate::gzguts_h::gz_state,
    mut buf: crate::stdlib::voidp,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
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
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    state.x.next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                n = gz_consume(state, n as crate::stdlib::off64_t);
                consumed_buffered = true;
                if state.err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
            }
            crate::src::gzlib::GzReadPlan::End => break 's_140,
            crate::src::gzlib::GzReadPlan::Fetch => {
                if gz_fetch(state) == -1 as ::core::ffi::c_int
                    && state.x.have == 0 as ::core::ffi::c_uint
                {
                    err = -1 as ::core::ffi::c_int;
                }
                // Fetch only fills gzip's internal output buffer.  It has not
                // yet copied a byte to the caller, so retry before accounting.
                if err == 0 {
                    continue 's_140;
                }
                break 's_140;
            }
            crate::src::gzlib::GzReadPlan::Copy(chunk) => {
                n = chunk;
                let result = gz_load(state, buf as *mut ::core::ffi::c_uchar, n);
                n = result.received;
                err = result.status;
            }
            crate::src::gzlib::GzReadPlan::Decompress(chunk) => {
                n = chunk;
                state.strm.avail_out = n as crate::stdlib::uInt;
                state.strm.next_out = buf as *mut crate::stdlib::Bytef;
                err = gz_decomp(state);
                n = state.x.have;
                state.x.have = 0 as ::core::ffi::c_uint;
            }
        }
        crate::src::gzlib::gz_read_progress(state, &mut len, &mut got, n, consumed_buffered);
        buf = (buf as *mut crate::stdlib::Bytef).wrapping_add(n as usize) as crate::stdlib::voidp;
        if !(len != 0 && err == 0) {
            break;
        }
    }
    crate::src::gzlib::gz_read_mark_past(state, len);
    return got;
}
pub unsafe extern "C" fn gzread(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state = &mut *state;
    if !crate::src::gzlib::gz_read_state_is_usable(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in an int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    len = gz_read(state, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if state.again != 0 {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_ERRNO,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    return len as ::core::ffi::c_int;
}
#[export_name = "gzread"]

pub unsafe extern "C" fn gzread_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzread(file, buf, len)
}
pub unsafe extern "C" fn gzfread(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let mut len: crate::stdlib::z_size_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state = &mut *state;
    if !crate::src::gzlib::gz_read_state_is_usable(state) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as crate::stdlib::z_size_t;
    }
    return if len != 0 {
        gz_read(state, buf, len).wrapping_div(size)
    } else {
        0 as crate::stdlib::z_size_t
    };
}
#[export_name = "gzfread"]

pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    gzfread(buf, size, nitems, file)
}
pub unsafe extern "C" fn gzgetc(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state = &mut *state;
    if !crate::src::gzlib::gz_read_state_is_usable(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if state.x.have != 0 {
        let c2rust_fresh2 = state.x.next;
        gz_consume(state, 1);
        return *c2rust_fresh2 as ::core::ffi::c_int;
    }
    return if gz_read(
        state,
        &raw mut buf as *mut ::core::ffi::c_uchar as crate::stdlib::voidp,
        1 as crate::stdlib::z_size_t,
    ) < 1 as crate::stdlib::z_size_t
    {
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
pub unsafe extern "C" fn gzungetc(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_READ) {
        return -1 as ::core::ffi::c_int;
    }
    if state.how == crate::gzguts_h::LOOK && state.x.have == 0 as ::core::ffi::c_uint {
        gz_look(state);
    }
    if !crate::src::gzlib::gz_read_state_is_usable(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if gz_finish_skip(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    match crate::src::gzlib::gz_ungetc_plan(state) {
        crate::src::gzlib::GzUngetcPlan::First { buffer_end } => {
            state.x.next = state
                .out
                .wrapping_add(buffer_end as usize)
                .wrapping_sub(1);
            *state.x.next = c as ::core::ffi::c_uchar;
            crate::src::gzlib::gz_ungetc_progress(state, true);
        }
        crate::src::gzlib::GzUngetcPlan::Full => {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_DATA_ERROR,
                b"out of room to push characters\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        crate::src::gzlib::GzUngetcPlan::Prepend { move_to_end } => {
            if move_to_end {
                let mut src: *mut ::core::ffi::c_uchar =
                    state.out.wrapping_add(state.x.have as usize);
                let mut dest: *mut ::core::ffi::c_uchar = state
                    .out
                    .wrapping_add((state.size << 1 as ::core::ffi::c_int) as usize);
                while src > state.out {
                    src = src.wrapping_sub(1);
                    dest = dest.wrapping_sub(1);
                    *dest = *src;
                }
                state.x.next = dest;
            }
            state.x.next = state.x.next.wrapping_sub(1);
            *state.x.next = c as ::core::ffi::c_uchar;
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
    gzungetc(c, file)
}
pub unsafe extern "C" fn gzgets(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut left: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut eol: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if file.is_null() || buf.is_null() || len < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !crate::src::gzlib::gz_read_state_is_usable(state) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if gz_finish_skip(state) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    str = buf;
    left = (len as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
    if left != 0 {
        while !(state.x.have == 0 as ::core::ffi::c_uint
            && gz_fetch(state) == -1 as ::core::ffi::c_int)
        {
            if state.x.have == 0 as ::core::ffi::c_uint {
                state.past = 1 as ::core::ffi::c_int;
                break;
            } else {
                n = if state.x.have > left {
                    left
                } else {
                    state.x.have
                };
                eol = crate::stdlib::memchr(
                    state.x.next as *const ::core::ffi::c_void,
                    '\n' as ::core::ffi::c_int,
                    n as crate::__stddef_size_t_h::size_t,
                ) as *mut ::core::ffi::c_uchar;
                if !eol.is_null() {
                    n = (eol.offset_from(state.x.next) as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint);
                }
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    state.x.next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                gz_consume(state, n as crate::stdlib::off64_t);
                left = left.wrapping_sub(n);
                buf = buf.wrapping_add(n as usize);
                if !(left != 0 && eol.is_null()) {
                    break;
                }
            }
        }
    }
    if buf == str {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *buf = 0 as ::core::ffi::c_char;
    return str;
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    gzgets(file, buf, len)
}
pub unsafe extern "C" fn gzdirect(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state = &mut *state;
    if crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_READ)
        && state.how == crate::gzguts_h::LOOK
        && state.x.have == 0 as ::core::ffi::c_uint
    {
        gz_look(state);
    }
    return (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzdirect(file)
}
pub unsafe extern "C" fn gzclose_r(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state = &mut *state;
    if !crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_READ) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.size != 0 {
        crate::src::inflate::inflateEnd(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
        crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
    }
    err = if state.err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let path = state.path;
    let fd = state.fd;
    crate::stdlib::free(path as *mut ::core::ffi::c_void);
    ret = crate::stdlib::close(fd);
    crate::stdlib::free(file as *mut ::core::ffi::c_void);
    return if ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    };
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_r(file)
}
