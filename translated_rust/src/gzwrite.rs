pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_error;
pub use crate::src::gzlib::gz_intmax;
pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::deflate;
pub use crate::src::deflate::deflateEnd;
pub use crate::src::deflate::deflateInit2_;
pub use crate::src::deflate::deflateParams;
pub use crate::src::deflate::deflateReset;
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

// All callers have already validated and bound the gzip state. Keep this
// coordinator reference-bound; its allocation, cleanup, deflater setup, and
// error bridges are confined to the initialization boundary below.
fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    // SAFETY: the validated write state is uninitialized on entry. This
    // boundary allocates its gzip buffers, configures its deflater, and on
    // failure frees only allocations made here before updating that same
    // state's error record.
    unsafe {
        state.in_0 = crate::stdlib::malloc(
            (state.want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
        ) as *mut ::core::ffi::c_uchar;
        if state.in_0.is_null() {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if state.direct == 0 {
            state.out = crate::stdlib::malloc(state.want as crate::__stddef_size_t_h::size_t)
                as *mut ::core::ffi::c_uchar;
            if state.out.is_null() {
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            state.strm.zalloc = None;
            state.strm.zfree = None;
            state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
            ret = crate::src::deflate::deflateInit2_(
                &mut state.strm,
                state.level,
                8 as ::core::ffi::c_int,
                15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int,
                state.strategy,
                crate::zlib_h::ZLIB_VERSION.as_ptr(),
                ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
            );
            if ret != crate::zlib_h::Z_OK {
                crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        }
    }
    state.size = state.want;
    if state.direct == 0 {
        state.strm.avail_out = state.size as crate::stdlib::uInt;
        state.strm.next_out = state.out;
        state.x.next = state.strm.next_out;
    }
    return 0 as ::core::ffi::c_int;
}

// All callers have already validated and bound the gzip state. Descriptor
// writes, errno access, deflater calls, and error-string bridges remain
// documented raw boundaries within this coordinator.
fn gz_comp(
    state: &mut crate::gzguts_h::gz_state,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    if crate::src::gzlib::gz_write_needs_init(state)
        && gz_init(state) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    match crate::src::gzlib::gz_comp_mode(state, flush) {
        crate::src::gzlib::GzCompMode::Direct => {
            while state.strm.avail_in != 0 {
                // SAFETY: the direct write state exposes `avail_in` bytes at
                // `next_in`; this request is capped by that count. The errno
                // slot and descriptor are used only for this POSIX write.
                let (written, errno) = unsafe {
                    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                    crate::src::gzlib::gz_begin_io(state);
                    put = crate::src::gzlib::gz_comp_direct_write_request(state);
                    let written = crate::stdlib::write(
                        state.fd,
                        state.strm.next_in as *const ::core::ffi::c_void,
                        put as crate::__stddef_size_t_h::size_t,
                    ) as ::core::ffi::c_int;
                    (written, *crate::stdlib::__errno_location())
                };
                if let Err(errno) = crate::src::gzlib::gz_io_result(state, written, errno) {
                    // SAFETY: `state` is the validated gzip state and the
                    // errno string is owned by the C runtime for this call.
                    unsafe {
                        crate::src::gzlib::gz_error(
                            state,
                            crate::zlib_h::Z_ERRNO,
                            crate::stdlib::strerror(errno),
                        );
                    }
                    return -1 as ::core::ffi::c_int;
                }
                crate::src::gzlib::gz_direct_write_progress(
                    state,
                    written as ::core::ffi::c_uint,
                );
                state.strm.next_in = state.strm.next_in.wrapping_add(written as usize);
            }
            return 0 as ::core::ffi::c_int;
        }
        crate::src::gzlib::GzCompMode::Idle => {
            return 0 as ::core::ffi::c_int;
        }
        crate::src::gzlib::GzCompMode::Reset => {
            // SAFETY: initialization created the deflater stored in this
            // validated write state before it can reach the reset path.
            unsafe { crate::src::deflate::deflateReset(&mut state.strm) };
            crate::src::gzlib::gz_comp_reset_complete(state);
        }
        crate::src::gzlib::GzCompMode::Deflate => {}
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if let Some(plan) = crate::src::gzlib::gz_comp_output_plan(state, flush, ret) {
            while state.strm.next_out > state.x.next {
                // SAFETY: the output plan bounds the pending range from
                // `x.next`, and this scope owns the descriptor/errno bridge
                // for draining that initialized output buffer.
                let (written, errno) = unsafe {
                    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                    crate::src::gzlib::gz_begin_io(state);
                    put = crate::src::gzlib::gz_comp_output_write_request(state);
                    let written = crate::stdlib::write(
                        state.fd,
                        state.x.next as *const ::core::ffi::c_void,
                        put as crate::__stddef_size_t_h::size_t,
                    ) as ::core::ffi::c_int;
                    (written, *crate::stdlib::__errno_location())
                };
                if let Err(errno) = crate::src::gzlib::gz_io_result(state, written, errno) {
                    // SAFETY: `state` is valid and this reports the string
                    // returned by the C runtime for the failed write.
                    unsafe {
                        crate::src::gzlib::gz_error(
                            state,
                            crate::zlib_h::Z_ERRNO,
                            crate::stdlib::strerror(errno),
                        );
                    }
                    return -1 as ::core::ffi::c_int;
                }
                crate::src::gzlib::gz_comp_output_write_progress(
                    state,
                    written as ::core::ffi::c_uint,
                );
            }
            if plan.reset {
                crate::src::gzlib::gz_comp_reset_output(state);
            }
        }
        have = state.strm.avail_out as ::core::ffi::c_uint;
        // SAFETY: `gz_init` configured this deflater and its input/output
        // fields are maintained by this validated write-state machine.
        ret = unsafe { crate::src::deflate::deflate(&mut state.strm, flush) };
        if ret == crate::zlib_h::Z_STREAM_ERROR {
            // SAFETY: this updates only the validated gzip state's error
            // record with a static message.
            unsafe {
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: deflate stream corrupt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        have = crate::src::gzlib::gz_produced(have, state.strm.avail_out);
        if have == 0 {
            break;
        }
    }
    crate::src::gzlib::gz_comp_finish(state, flush);
    return 0 as ::core::ffi::c_int;
}

// Callers have already validated and bound the gzip state.  Keep this as an
// internal Rust helper so its progress bookkeeping does not need to recover a
// mutable reference from a raw pointer.  Zero-fill and compression remain
// explicit raw-boundary operations below.
fn gz_zero(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    if state.strm.avail_in != 0
        // The validated gzip state owns the initialized stream and buffers
        // required by the compression adapter.
        && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    first = 1 as ::core::ffi::c_int;
    loop {
        n = crate::src::gzlib::gz_skip_chunk(state.size, state.skip);
        if first != 0 {
            // SAFETY: `gz_init` allocated `in_0` with at least `size` bytes,
            // and this first sparse-write chunk is bounded by that size.
            unsafe {
                crate::stdlib::memset(
                    state.in_0 as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    n as crate::__stddef_size_t_h::size_t,
                );
            }
            first = 0 as ::core::ffi::c_int;
        }
        state.strm.avail_in = n as crate::stdlib::uInt;
        state.strm.next_in = state.in_0;
        // The validated gzip state owns the stream and the `in_0` range
        // configured immediately above for this compression request.
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        crate::src::gzlib::gz_zero_progress(state, n);
        if ret == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if state.skip == 0 {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

// Exported entry points validate their handle before calling this helper. Keep
// the internal state reference-bound; only the caller-buffer copy remains a
// scoped raw boundary.
fn gz_write(
    state: &mut crate::gzguts_h::gz_state,
    mut buf: crate::stdlib::voidpc,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut put: crate::stdlib::z_size_t = len;
    let mut ret: ::core::ffi::c_int = 0;
    let buffered = loop {
        match crate::src::gzlib::gz_write_plan(state, len) {
            crate::src::gzlib::GzWritePlan::Empty => return 0 as crate::stdlib::z_size_t,
            crate::src::gzlib::GzWritePlan::Initialize => {
                // This plan is selected only for an uninitialized, validated
                // write state, which `gz_init` configures.
                if gz_init(state) == -1 as ::core::ffi::c_int {
                    return 0 as crate::stdlib::z_size_t;
                }
            }
            crate::src::gzlib::GzWritePlan::Zero => {
                if gz_zero(state) == -1 as ::core::ffi::c_int {
                    return 0 as crate::stdlib::z_size_t;
                }
            }
            crate::src::gzlib::GzWritePlan::Buffered => break true,
            crate::src::gzlib::GzWritePlan::Stream => break false,
        }
    };
    if buffered {
        loop {
            let plan = crate::src::gzlib::gz_buffered_copy_plan(state, len);
            // SAFETY: the plan limits the destination to free bytes in the
            // initialized gzip input buffer; the FFI caller supplied at least
            // the remaining source bytes. These ranges do not overlap.
            unsafe {
                crate::stdlib::memcpy(
                    state.in_0.wrapping_add(plan.offset as usize) as *mut ::core::ffi::c_void,
                    buf as *const ::core::ffi::c_void,
                    plan.len as crate::__stddef_size_t_h::size_t,
                );
            }
            crate::src::gzlib::gz_buffered_copy_progress(state, &mut len, plan.len);
            buf = (buf as *const crate::stdlib::Bytef)
                .wrapping_add(plan.len as usize)
                as crate::stdlib::voidpc;
            if len == 0 as crate::stdlib::z_size_t {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return crate::src::gzlib::gz_write_error_result(state, put, len);
            }
        }
    } else {
        if state.strm.avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
        {
            return 0 as crate::stdlib::z_size_t;
        }
        state.strm.next_in = buf as *mut crate::stdlib::Bytef;
        loop {
            let mut n: ::core::ffi::c_uint = crate::src::gzlib::gz_stream_chunk(len);
            state.strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            n = crate::src::gzlib::gz_stream_write_progress(state, &mut len, n);
            if ret == -1 as ::core::ffi::c_int {
                return crate::src::gzlib::gz_write_error_result(state, put, len);
            }
            if len == 0 {
                break;
            }
        }
    }
    return put;
}
// The exported wrapper owns handle validation and binding.  This coordinator
// operates on that bound state; `gz_write` retains the caller-buffer copy
// boundary used for buffered and streaming writes.
pub fn gzwrite(
    state: &mut crate::gzguts_h::gz_state,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if !crate::src::gzlib::gz_write_state_is_usable(state) {
        return 0 as ::core::ffi::c_int;
    }
    // This is a validated write state, so clearing the shared error record
    // cannot affect read-side EOF bookkeeping.
    crate::src::gzlib::gzclearerr(state);
    if !crate::src::gzlib::gz_uint_request_fits_int(len) {
        // SAFETY: this only updates the bound gzip state's owned error
        // record with a static message.
        unsafe {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_DATA_ERROR,
                b"requested length does not fit in int\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
    return gz_write(state, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_int;
}
#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    gzwrite(&mut *(file as crate::gzguts_h::gz_statep), buf, len)
}
pub fn gzfwrite(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    state: &mut crate::gzguts_h::gz_state,
) -> crate::stdlib::z_size_t {
    if !crate::src::gzlib::gz_write_state_is_usable(state) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gzclearerr(state);
    match crate::src::gzlib::gz_item_request(size, nitems) {
        crate::src::gzlib::GzItemRequest::Empty => 0 as crate::stdlib::z_size_t,
        crate::src::gzlib::GzItemRequest::TooLarge => {
            // SAFETY: this only updates the bound gzip state's owned error
            // record with a static message.
            unsafe {
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"request does not fit in a size_t\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            0 as crate::stdlib::z_size_t
        }
        crate::src::gzlib::GzItemRequest::Bytes(len) => {
            gz_write(state, buf, len).wrapping_div(size)
        }
    }
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    gzfwrite(buf, size, nitems, &mut *(file as crate::gzguts_h::gz_statep))
}
// The one-byte request can use the same buffered/streaming adapter as larger
// writes.  Keeping the byte in a local array lets this coordinator remain
// reference-bound; `gz_write()` retains the sole raw-copy boundary.
pub fn gzputc(
    state: &mut crate::gzguts_h::gz_state,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let buf: [::core::ffi::c_uchar; 1] = [c as ::core::ffi::c_uchar];
    if !crate::src::gzlib::gz_write_state_is_usable(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gzclearerr(state);
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if gz_write(
        state,
        buf.as_ptr() as crate::stdlib::voidpc,
        1 as crate::stdlib::z_size_t,
    ) != 1 as crate::stdlib::z_size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    return c & 0xff as ::core::ffi::c_int;
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzputc(&mut *(file as crate::gzguts_h::gz_statep), c)
}
pub unsafe extern "C" fn gzputs(
    state: &mut crate::gzguts_h::gz_state,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: crate::stdlib::z_size_t = 0;
    let mut put: crate::stdlib::z_size_t = 0;
    if !crate::src::gzlib::gz_write_state_is_usable(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gzclearerr(state);
    len = crate::stdlib::strlen(s) as crate::stdlib::z_size_t;
    if !crate::src::gzlib::gz_string_len_fits_int(len) {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"string length does not fit in int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    put = gz_write(state, s as crate::stdlib::voidpc, len);
    crate::src::gzlib::gz_puts_result(len, put)
}
#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzputs(&mut *(file as crate::gzguts_h::gz_statep), s)
}
fn gzflush(
    state: &mut crate::gzguts_h::gz_state,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !crate::src::gzlib::gz_write_state_is_usable(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // For a validated write state, `gzclearerr` has the same error-reset
    // effect as `gz_error(..., Z_OK, null)` without widening this
    // reference-bound coordinator's unsafe surface.
    crate::src::gzlib::gzclearerr(state);
    match crate::src::gzlib::gz_flush_plan(state, flush) {
        crate::src::gzlib::GzFlushPlan::Invalid => {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        crate::src::gzlib::GzFlushPlan::Zero => {
            if gz_zero(state) == -1 as ::core::ffi::c_int {
                return state.err;
            }
        }
        crate::src::gzlib::GzFlushPlan::Compress => {}
    }
    gz_comp(state, flush);
    return state.err;
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    gzflush(&mut *(file as crate::gzguts_h::gz_statep), flush)
}
// Parameter selection only needs the already-bound write state. Keep the
// deflater call scoped to its one C boundary so the surrounding validation,
// skip handling, and bookkeeping remain safe Rust.
pub fn gzsetparams(
    state: &mut crate::gzguts_h::gz_state,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !crate::src::gzlib::gz_write_state_is_usable(state) || state.direct != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gzclearerr(state);
    if level == state.level && strategy == state.strategy {
        return crate::zlib_h::Z_OK;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    if state.size != 0 {
        if state.strm.avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
        {
            return state.err;
        }
        // SAFETY: `gz_init` created the deflater in this validated write
        // state before a nonzero `size` can reach this branch.
        unsafe {
            crate::src::deflate::deflateParams(&mut state.strm, level, strategy);
        }
    }
    state.level = level;
    state.strategy = strategy;
    return crate::zlib_h::Z_OK;
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
    gzsetparams(
        &mut *(file as crate::gzguts_h::gz_statep),
        level,
        strategy,
    )
}
pub unsafe extern "C" fn gzclose_w(
    state: &mut crate::gzguts_h::gz_state,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    if !crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_WRITE) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        ret = state.err;
    }
    if gz_comp(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int {
        ret = state.err;
    }
    if state.size != 0 {
        if state.direct == 0 {
            crate::src::deflate::deflateEnd(
                &mut state.strm as *mut crate::zlib_h::z_stream_s,
            );
            crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
        }
        crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
    }
    crate::src::gzlib::gzclearerr(state);
    let path = state.path;
    let fd = state.fd;
    crate::stdlib::free(path as *mut ::core::ffi::c_void);
    if crate::stdlib::close(fd) == -1 as ::core::ffi::c_int {
        ret = crate::zlib_h::Z_ERRNO;
    }
    crate::stdlib::free(file as *mut ::core::ffi::c_void);
    return ret;
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    gzclose_w(&mut *(file as crate::gzguts_h::gz_statep), file)
}
