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

// Initial deflater fields are ordinary gzip state.  Keep their setup outside
// the allocation and deflater-creation boundary in `gz_init()`.
fn gz_init_prepare_deflater(state: &mut crate::gzguts_h::gz_state) {
    state.strm.zalloc = None;
    state.strm.zfree = None;
    state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
}

// The successful initialization tail only publishes buffers already created
// by `gz_init()` to the bound gzip state.
fn gz_init_finish(state: &mut crate::gzguts_h::gz_state) {
    state.size = state.want;
    if state.direct == 0 {
        state.strm.avail_out = state.size as crate::stdlib::uInt;
        state.strm.next_out = state.out;
        state.x.next = state.strm.next_out;
    }
}

// All callers have already validated and bound the gzip state. Keep this
// coordinator reference-bound; its allocation, cleanup, deflater setup, and
// error bridges are confined to the initialization boundary below.
fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let init = crate::src::gzlib::gz_init_plan(state);
    let input_len = match init {
        crate::src::gzlib::GzInitPlan::Direct { input_len }
        | crate::src::gzlib::GzInitPlan::Deflate { input_len, .. } => input_len,
    };
    let deflate = match init {
        crate::src::gzlib::GzInitPlan::Direct { .. } => None,
        crate::src::gzlib::GzInitPlan::Deflate {
            output_len,
            level,
            strategy,
            ..
        } => Some((output_len, level, strategy)),
    };
    state.in_0 = crate::stdlib::malloc(input_len) as *mut ::core::ffi::c_uchar;
    if state.in_0.is_null() {
        crate::src::gzlib::gz_error(state, crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory\0"));
        return -1 as ::core::ffi::c_int;
    }
    if let Some((output_len, level, strategy)) = deflate {
        state.out = crate::stdlib::malloc(output_len) as *mut ::core::ffi::c_uchar;
        if state.out.is_null() {
            // SAFETY: this failure path releases only the input allocation
            // created above before reporting the initialization error.
            unsafe {
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            }
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory\0"),
            );
            return -1 as ::core::ffi::c_int;
        }
        gz_init_prepare_deflater(state);
        // SAFETY: the validated write state owns both initialized buffers;
        // this call creates the deflater that will use them.
        ret = unsafe {
            crate::src::deflate::deflateInit2_(
                &mut state.strm,
                level,
                8 as ::core::ffi::c_int,
                15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int,
                strategy,
                crate::zlib_h::ZLIB_VERSION.as_ptr(),
                ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
            )
        };
        if ret != crate::zlib_h::Z_OK {
            // SAFETY: failed initialization has not transferred either
            // allocation, so this path releases exactly those two buffers.
            unsafe {
                crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            }
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory\0"),
            );
            return -1 as ::core::ffi::c_int;
        }
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    gz_init_finish(state);
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
    if crate::src::gzlib::gz_write_needs_init(state) && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    match crate::src::gzlib::gz_comp_mode(state, flush) {
        crate::src::gzlib::GzCompMode::Direct => {
            while state.strm.avail_in != 0 {
                // SAFETY: the direct write state exposes `avail_in` bytes at
                // `next_in`; this request is capped by that count. The errno
                // slot and descriptor are used only for this POSIX write.
                let put = crate::src::gzlib::gz_comp_direct_write_request(state);
                crate::src::gzlib::gz_begin_io(state);
                let (written, errno) = unsafe {
                    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                    let written = crate::stdlib::write(
                        state.fd,
                        state.strm.next_in as *const ::core::ffi::c_void,
                        put as crate::__stddef_size_t_h::size_t,
                    ) as ::core::ffi::c_int;
                    (written, *crate::stdlib::__errno_location())
                };
                if let Err(errno) = crate::src::gzlib::gz_io_result(state, written, errno) {
                    unsafe {
                        crate::src::gzlib::gz_error(
                            state,
                            crate::zlib_h::Z_ERRNO,
                            Some(
                                ::core::ffi::CStr::from_ptr(crate::stdlib::strerror(errno))
                                    .to_bytes_with_nul(),
                            ),
                        );
                    }
                    return -1 as ::core::ffi::c_int;
                }
                crate::src::gzlib::gz_direct_write_progress(state, written as ::core::ffi::c_uint);
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
            while crate::src::gzlib::gz_comp_output_pending(state) != 0 {
                // SAFETY: the output plan bounds the pending range from
                // `x.next`, and this scope owns the descriptor/errno bridge
                // for draining that initialized output buffer.
                let put = crate::src::gzlib::gz_comp_output_write_request(state);
                crate::src::gzlib::gz_begin_io(state);
                let (written, errno) = unsafe {
                    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                    let written = crate::stdlib::write(
                        state.fd,
                        state.x.next as *const ::core::ffi::c_void,
                        put as crate::__stddef_size_t_h::size_t,
                    ) as ::core::ffi::c_int;
                    (written, *crate::stdlib::__errno_location())
                };
                if let Err(errno) = crate::src::gzlib::gz_io_result(state, written, errno) {
                    unsafe {
                        crate::src::gzlib::gz_error(
                            state,
                            crate::zlib_h::Z_ERRNO,
                            Some(
                                ::core::ffi::CStr::from_ptr(crate::stdlib::strerror(errno))
                                    .to_bytes_with_nul(),
                            ),
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
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(b"internal error: deflate stream corrupt\0"),
            );
            return -1 as ::core::ffi::c_int;
        }
        have = crate::src::gzlib::gz_produced(have, state.strm.avail_out);
        crate::src::gzlib::gz_comp_output_produced(state, have);
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
            // `write_bytes` does not form references to malloc's still-
            // uninitialized storage.
            unsafe {
                ::core::ptr::write_bytes(state.in_0, 0, n as usize);
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

// Exported entry points bind their caller buffers before calling this helper.
// Keep its input cursor as a Rust slice; only the initialized gzip input
// buffer remains a scoped raw-copy boundary.
fn gz_write(
    state: &mut crate::gzguts_h::gz_state,
    mut source: &[::core::ffi::c_uchar],
) -> crate::stdlib::z_size_t {
    let mut len = source.len() as crate::stdlib::z_size_t;
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
            // SAFETY: the plan limits this range to free bytes in the
            // initialized gzip input allocation, and the FFI caller supplied
            // at least the remaining source bytes. Bind that allocation once;
            // the copy itself can then stay in safe Rust.
            let destination = unsafe {
                ::core::slice::from_raw_parts_mut(
                    state.in_0.wrapping_add(plan.offset as usize),
                    plan.len as usize,
                )
            };
            destination.copy_from_slice(&source[..plan.len as usize]);
            crate::src::gzlib::gz_buffered_copy_progress(state, &mut len, plan.len);
            source = &source[plan.len as usize..];
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
        state.strm.next_in = source.as_ptr() as *mut crate::stdlib::Bytef;
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
    source: &[::core::ffi::c_uchar],
) -> ::core::ffi::c_int {
    if !crate::src::gzlib::gz_begin_write_operation(state) {
        return 0 as ::core::ffi::c_int;
    }
    if !crate::src::gzlib::gz_uint_request_fits_int(source.len() as ::core::ffi::c_uint) {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            Some(b"requested length does not fit in int\0"),
        );
        return 0 as ::core::ffi::c_int;
    }
    gz_write(state, source) as ::core::ffi::c_int
}

// Decide whether the ABI wrapper may bind its caller buffer.  This retains
// `gzwrite()`'s state transition and request error behavior while keeping
// those decisions out of the raw-pointer entry point.
fn gzwrite_preflight(
    state: &mut crate::gzguts_h::gz_state,
    len: ::core::ffi::c_uint,
) -> Result<usize, ()> {
    if !crate::src::gzlib::gz_begin_write_operation(state) {
        return Err(());
    }
    if !crate::src::gzlib::gz_uint_request_fits_int(len) {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            Some(b"requested length does not fit in int\0"),
        );
        return Err(());
    }
    let Some(slice_len) = crate::src::gzlib::gz_rust_slice_len(len as crate::stdlib::z_size_t)
    else {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            Some(b"request does not fit in a Rust slice\0"),
        );
        return Err(());
    };
    Ok(slice_len)
}

// The raw FFI wrapper only binds a range selected by `gzwrite_preflight()`.
// This adapter owns the result and buffer-shape handling after that binding.
fn gzwrite_ffi_dispatch(
    state: &mut crate::gzguts_h::gz_state,
    prepared: Result<usize, ()>,
    source: Option<&[::core::ffi::c_uchar]>,
) -> ::core::ffi::c_int {
    match (prepared, source) {
        (Ok(len), Some(source)) if source.len() == len => {
            gz_write(state, source) as ::core::ffi::c_int
        }
        (Ok(_), _) => {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(b"request does not match caller buffer\0"),
            );
            0
        }
        (Err(()), _) => 0,
    }
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
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    let prepared = gzwrite_preflight(state, len);
    // SAFETY: C's `gzwrite` contract supplies `len` readable bytes when
    // `len` is nonzero. The preflight has already preserved the invalid-state
    // and rejected-request exits before this caller range is bound.
    let source = match prepared {
        Ok(0) => Some(&[] as &[::core::ffi::c_uchar]),
        Ok(len) if !buf.is_null() => Some(::core::slice::from_raw_parts(
            buf as *const ::core::ffi::c_uchar,
            len,
        )),
        _ => None,
    };
    gzwrite_ffi_dispatch(state, prepared, source)
}
pub fn gzfwrite(
    source: &[::core::ffi::c_uchar],
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    state: &mut crate::gzguts_h::gz_state,
) -> crate::stdlib::z_size_t {
    if !crate::src::gzlib::gz_begin_write_operation(state) {
        return 0 as crate::stdlib::z_size_t;
    }
    match crate::src::gzlib::gz_item_request(size, nitems) {
        crate::src::gzlib::GzItemRequest::Empty => 0 as crate::stdlib::z_size_t,
        crate::src::gzlib::GzItemRequest::TooLarge => {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(b"request does not fit in a size_t\0"),
            );
            0 as crate::stdlib::z_size_t
        }
        crate::src::gzlib::GzItemRequest::Bytes(_) => gz_write(state, source).wrapping_div(size),
    }
}

// As for `gzwrite_preflight`, classify the write state and complete item
// request before the FFI adapter turns a caller pointer into a Rust slice.
fn gzfwrite_preflight(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Result<usize, ()> {
    if !crate::src::gzlib::gz_begin_write_operation(state) {
        return Err(());
    }
    match crate::src::gzlib::gz_item_request(size, nitems) {
        crate::src::gzlib::GzItemRequest::Empty => Ok(0),
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
            Ok(slice_len)
        }
    }
}

fn gzfwrite_ffi_dispatch(
    state: &mut crate::gzguts_h::gz_state,
    size: crate::stdlib::z_size_t,
    prepared: Result<usize, ()>,
    source: Option<&[::core::ffi::c_uchar]>,
) -> crate::stdlib::z_size_t {
    match (prepared, source) {
        (Ok(0), _) => 0,
        (Ok(len), Some(source)) if source.len() == len => {
            gz_write(state, source).wrapping_div(size)
        }
        (Ok(_), _) => {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(b"request does not match caller buffer\0"),
            );
            0
        }
        (Err(()), _) => 0,
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
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    let prepared = gzfwrite_preflight(state, size, nitems);
    // SAFETY: a nonempty, representable item request requires C to provide
    // that many readable bytes. The preflight has already preserved the
    // invalid-state and rejected-request exits before binding `buf`.
    let source = match prepared {
        Ok(0) => Some(&[] as &[::core::ffi::c_uchar]),
        Ok(len) if !buf.is_null() => Some(::core::slice::from_raw_parts(
            buf as *const ::core::ffi::c_uchar,
            len,
        )),
        _ => None,
    };
    gzfwrite_ffi_dispatch(state, size, prepared, source)
}
// The one-byte request can use the same buffered/streaming adapter as larger
// writes. Keeping the byte in a local array lets this coordinator remain
// reference-bound and feed the safe source-slice interface directly.
pub fn gzputc(
    state: &mut crate::gzguts_h::gz_state,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let buf: [::core::ffi::c_uchar; 1] = [c as ::core::ffi::c_uchar];
    if !crate::src::gzlib::gz_begin_write_operation(state) {
        return -1 as ::core::ffi::c_int;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if gz_write(state, &buf) != 1 as crate::stdlib::z_size_t {
        return -1 as ::core::ffi::c_int;
    }
    return c & 0xff as ::core::ffi::c_int;
}

// Keep the public null-handle result with the implementation dispatch. The
// ABI adapter only binds a non-null handle before this stateful operation.
fn gzputc_ffi_dispatch(
    state: Option<&mut crate::gzguts_h::gz_state>,
    c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match state {
        Some(state) => gzputc(state, c),
        None => -1,
    }
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let state = if file.is_null() {
        None
    } else {
        // SAFETY: a non-null gzip handle identifies the state bound by this
        // ABI entry. The dispatcher owns the public null-handle result.
        Some(unsafe { &mut *(file as crate::gzguts_h::gz_statep) })
    };
    gzputc_ffi_dispatch(state, c)
}
// Once the dispatcher has accepted the write state and bound the caller's
// string, the write itself needs only a safe C-string view.
fn gzputs_write(
    state: &mut crate::gzguts_h::gz_state,
    s: &::core::ffi::CStr,
) -> ::core::ffi::c_int {
    let source = s.to_bytes();
    let len = source.len() as crate::stdlib::z_size_t;
    if !crate::src::gzlib::gz_string_len_fits_int(len) {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"string length does not fit in int\0"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let put = gz_write(state, source);
    crate::src::gzlib::gz_puts_result(len, put)
}

pub fn gzputs(state: &mut crate::gzguts_h::gz_state, s: &::core::ffi::CStr) -> ::core::ffi::c_int {
    if !crate::src::gzlib::gz_begin_write_operation(state) {
        return -1 as ::core::ffi::c_int;
    }
    gzputs_write(state, s)
}

// Keep gzip-state preflight out of the FFI adapter. It receives the bound
// caller string but decides whether the write operation may proceed.
fn gzputs_ffi_dispatch(
    state: Option<&mut crate::gzguts_h::gz_state>,
    source: Option<&::core::ffi::CStr>,
) -> ::core::ffi::c_int {
    let Some(state) = state else {
        return -1;
    };
    if !crate::src::gzlib::gz_begin_write_operation(state) {
        return -1;
    }
    let Some(source) = source else {
        return -1;
    };
    gzputs_write(state, source)
}
#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let state = if file.is_null() {
        None
    } else {
        // SAFETY: a non-null gzip handle identifies the state bound by this
        // ABI entry. Its mode is checked by the implementation dispatcher.
        Some(unsafe { &mut *(file as crate::gzguts_h::gz_statep) })
    };
    let source = if s.is_null() {
        None
    } else {
        // SAFETY: C's `gzputs` contract supplies a non-null, nul-terminated
        // string. This exported adapter owns that caller-pointer conversion;
        // the dispatcher below owns all gzip-state preflight and operation
        // work.
        Some(unsafe { ::core::ffi::CStr::from_ptr(s) })
    };
    gzputs_ffi_dispatch(state, source)
}
fn gzflush(
    state: &mut crate::gzguts_h::gz_state,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !crate::src::gzlib::gz_begin_write_operation(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
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

// `gzflush` owns write-state validation. Keep only the public null-handle
// result in this implementation dispatcher, after the ABI adapter has bound
// the optional gzip state.
fn gzflush_ffi_dispatch(
    state: Option<&mut crate::gzguts_h::gz_state>,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match state {
        Some(state) => gzflush(state, flush),
        None => crate::zlib_h::Z_STREAM_ERROR,
    }
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let state = if file.is_null() {
        None
    } else {
        // SAFETY: a non-null gzip handle identifies the state bound by this
        // ABI entry. `gzflush` retains write-state validation.
        Some(unsafe { &mut *(file as crate::gzguts_h::gz_statep) })
    };
    gzflush_ffi_dispatch(state, flush)
}
// Parameter selection only needs the already-bound write state. Keep the
// deflater call scoped to its one C boundary so the surrounding validation,
// skip handling, and bookkeeping remain safe Rust.
pub fn gzsetparams(
    state: &mut crate::gzguts_h::gz_state,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let plan = crate::src::gzlib::gz_set_params_plan(state, level, strategy);
    if matches!(plan, crate::src::gzlib::GzSetParamsPlan::Invalid) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gzclearerr(state);
    if matches!(plan, crate::src::gzlib::GzSetParamsPlan::Unchanged) {
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

// The parameter operation owns all write-state and parameter validation.
// This dispatcher only maps an absent bound handle to the public result.
fn gzsetparams_ffi_dispatch(
    state: Option<&mut crate::gzguts_h::gz_state>,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match state {
        Some(state) => gzsetparams(state, level, strategy),
        None => crate::zlib_h::Z_STREAM_ERROR,
    }
}
#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let state = if file.is_null() {
        None
    } else {
        // SAFETY: a non-null gzip handle identifies the state bound by this
        // ABI entry. The safe operation retains parameter validation.
        Some(unsafe { &mut *(file as crate::gzguts_h::gz_statep) })
    };
    gzsetparams_ffi_dispatch(state, level, strategy)
}

// Once a write handle is known to be valid, finishing sparse output and the
// deflater only changes the bound gzip state. Keep that decision separate
// from the raw allocator/descriptor cleanup performed by the exported close
// boundary.
fn gz_close_write_prepare(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret = crate::zlib_h::Z_OK;
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        ret = state.err;
    }
    if gz_comp(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int {
        ret = state.err;
    }
    ret
}

// The close dispatcher has already bound `file` to `state`, so this helper
// can keep the close ordering and result selection in safe Rust. Allocation
// release and descriptor closing remain confined to the raw boundary below.
pub fn gzclose_w(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    if !crate::src::gzlib::gz_has_mode(state, crate::gzguts_h::GZ_WRITE) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    ret = gz_close_write_prepare(state);
    // Determine which resources this state owns before crossing into the raw
    // teardown boundary. This selection only inspects ordinary state flags;
    // the selected deflater and allocation releases remain below.
    let cleanup = crate::src::gzlib::gz_write_close_cleanup(state);
    // SAFETY: this close path owns the initialized gzip allocations and the
    // descriptor. The cleanup plan is derived from that bound state, and no
    // pointer escapes after its selected allocation is released.
    unsafe {
        match cleanup {
            crate::src::gzlib::GzWriteCloseCleanup::DeflaterAndBuffers => {
                crate::src::deflate::deflateEnd(&mut state.strm as *mut crate::zlib_h::z_stream_s);
                crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
            }
            crate::src::gzlib::GzWriteCloseCleanup::None
            | crate::src::gzlib::GzWriteCloseCleanup::Input => {}
        }
        if !matches!(cleanup, crate::src::gzlib::GzWriteCloseCleanup::None) {
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
        }
    }
    crate::src::gzlib::gzclearerr(state);
    let path = state.path;
    let fd = state.fd;
    // SAFETY: `path`, `fd`, and the allocation backing `state` are owned by
    // this closing state. The
    // order matches zlib: close can override an earlier write result, and
    // the state allocation is released only after its fields are no longer
    // needed.
    unsafe {
        crate::stdlib::free(path as *mut ::core::ffi::c_void);
        if crate::stdlib::close(fd) == -1 as ::core::ffi::c_int {
            ret = crate::zlib_h::Z_ERRNO;
        }
        crate::stdlib::free(state as *mut crate::gzguts_h::gz_state as *mut ::core::ffi::c_void);
    }
    return ret;
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    gzclose_w(&mut *(file as crate::gzguts_h::gz_statep))
}
