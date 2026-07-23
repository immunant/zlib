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

// A failed POSIX syscall sets the thread-local OS error. The gzip write paths
// inspect it only after a negative result, avoiding a raw errno-pointer read.
fn gz_last_errno() -> ::core::ffi::c_int {
    ::std::io::Error::last_os_error()
        .raw_os_error()
        .unwrap_or(0)
}

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
    let Some(input) = crate::src::gzlib::gz_owned_buffer(input_len) else {
        crate::src::gzlib::gz_error(state, crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory\0"));
        return -1 as ::core::ffi::c_int;
    };
    let output = if let Some((output_len, _, _)) = deflate {
        let Some(output) = crate::src::gzlib::gz_owned_buffer(output_len) else {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory\0"),
            );
            return -1 as ::core::ffi::c_int;
        };
        Some(output)
    } else {
        None
    };
    // The registry owns the backing Vecs for the entire initialized write
    // state. Publish them before installing C-facing cursors, then never
    // borrow one across a deflater call.
    if !crate::src::gzlib::gz_register_owned_write_buffers(state, input, output) {
        crate::src::gzlib::gz_error(state, crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory\0"));
        return -1 as ::core::ffi::c_int;
    }
    if let Some((_, level, strategy)) = deflate {
        if state.out.is_null() {
            crate::src::gzlib::gz_release_owned_write_buffers(state);
            state.in_0 = ::core::ptr::null_mut();
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(b"out of memory\0"),
            );
            return -1 as ::core::ffi::c_int;
        }
        gz_init_prepare_deflater(state);
        ret = crate::src::deflate::deflateInit2_(
            Some(&mut state.strm),
            level,
            8 as ::core::ffi::c_int,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            strategy,
            Some(crate::zlib_h::ZLIB_VERSION[0]),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        );
        if ret != crate::zlib_h::Z_OK {
            crate::src::gzlib::gz_release_owned_write_buffers(state);
            state.in_0 = ::core::ptr::null_mut();
            state.out = ::core::ptr::null_mut();
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
    input: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    if input.len() != state.strm.avail_in as usize {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"internal error: gzip input cursor mismatch\0"),
        );
        return -1;
    }
    if crate::src::gzlib::gz_write_needs_init(state) && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    match crate::src::gzlib::gz_comp_mode(state, flush) {
        crate::src::gzlib::GzCompMode::Direct => {
            while state.strm.avail_in != 0 {
                let put = crate::src::gzlib::gz_comp_direct_write_request(state);
                // Snapshot every state-derived argument before the descriptor
                // call. `write()` cannot call a zlib
                // allocator callback, so the state transition below remains
                // the only use of this bound gzip state after the syscall.
                let fd = state.fd;
                let input = state
                    .strm
                    .next_in
                    .cast_const()
                    .cast::<::core::ffi::c_void>();
                crate::src::gzlib::gz_begin_io(state);
                let written =
                    crate::stdlib::write(fd, input, put as crate::__stddef_size_t_h::size_t)
                        as ::core::ffi::c_int;
                let errno = if written < 0 { gz_last_errno() } else { 0 };
                if let Err(_) = crate::src::gzlib::gz_io_result(state, written, errno) {
                    let message = crate::src::gzlib::gz_errno_message();
                    crate::src::gzlib::gz_error(state, crate::zlib_h::Z_ERRNO, Some(&message));
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
            crate::src::deflate::deflateReset(&mut state.strm);
            crate::src::gzlib::gz_comp_reset_complete(state);
        }
        crate::src::gzlib::GzCompMode::Deflate => {}
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if let Some(plan) = crate::src::gzlib::gz_comp_output_plan(state, flush, ret) {
            while crate::src::gzlib::gz_comp_output_pending(state) != 0 {
                let put = crate::src::gzlib::gz_comp_output_write_request(state);
                // As on the direct path, snapshot the descriptor and bounded
                // byte range before the call. This keeps all gzip-state
                // observation and the subsequent progress update outside the
                // descriptor boundary.
                let fd = state.fd;
                let output = state.x.next.cast_const().cast::<::core::ffi::c_void>();
                crate::src::gzlib::gz_begin_io(state);
                let written =
                    crate::stdlib::write(fd, output, put as crate::__stddef_size_t_h::size_t)
                        as ::core::ffi::c_int;
                let errno = if written < 0 { gz_last_errno() } else { 0 };
                if let Err(_) = crate::src::gzlib::gz_io_result(state, written, errno) {
                    let message = crate::src::gzlib::gz_errno_message();
                    crate::src::gzlib::gz_error(state, crate::zlib_h::Z_ERRNO, Some(&message));
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
        // `gz_init` configured this deflater, and the dispatcher validates
        // its state before advancing the gzip write machine. The write
        // registry owns the output buffer, so lend its current bounded range
        // through the deflate core instead of reconstructing it from the C
        // cursor there.
        let consumed = input.len().wrapping_sub(state.strm.avail_in as usize);
        let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
        ret = crate::src::gzlib::gz_with_owned_write_output_buffer(state_key, |output| {
            let offset = state
                .strm
                .next_out
                .addr()
                .checked_sub(output.as_ptr().addr());
            let output = offset.and_then(|offset| {
                output.get_mut(offset..offset.checked_add(state.strm.avail_out as usize)?)
            });
            let Some(output) = output else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            crate::src::deflate::deflate(&mut state.strm, flush, &input[consumed..], output)
        })
        .unwrap_or(crate::zlib_h::Z_STREAM_ERROR);
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

// Buffered gzip writes retain their input in the write-buffer registry. Take
// a bounded snapshot for one deflater call instead of reconstructing a slice
// from the C-facing cursor in the deflate implementation.
fn gz_comp_with_owned_input(
    state: &mut crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let available = state.strm.avail_in as usize;
    let input = if available == 0 {
        Some(Vec::new())
    } else {
        let offset = state.strm.next_in.addr().checked_sub(state.in_0.addr());
        offset.and_then(|offset| {
            let end = offset.checked_add(available)?;
            crate::src::gzlib::gz_with_owned_write_input_buffer(
                crate::src::gzlib::gz_owned_buffer_key(state),
                |input| input.get(offset..end).map(<[u8]>::to_vec),
            )
            .flatten()
        })
    };
    let Some(input) = input else {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"internal error: gzip write buffer missing\0"),
        );
        return -1;
    };
    gz_comp(state, flush, &input)
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
        && gz_comp_with_owned_input(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    first = 1 as ::core::ffi::c_int;
    loop {
        n = crate::src::gzlib::gz_skip_chunk(state.size, state.skip);
        if first != 0 {
            let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
            let zeroed = crate::src::gzlib::gz_with_owned_write_input_buffer(state_key, |input| {
                for byte in &mut input[..n as usize] {
                    *byte = 0;
                }
            });
            if zeroed.is_none() {
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(b"internal error: gzip write buffer missing\0"),
                );
                return -1 as ::core::ffi::c_int;
            }
            first = 0 as ::core::ffi::c_int;
        }
        state.strm.avail_in = n as crate::stdlib::uInt;
        state.strm.next_in = state.in_0;
        // The validated gzip state owns the stream and the `in_0` range
        // configured immediately above for this compression request.
        ret = gz_comp_with_owned_input(state, crate::zlib_h::Z_NO_FLUSH);
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
            let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
            let copied = crate::src::gzlib::gz_with_owned_write_input_buffer(state_key, |input| {
                let end = plan.offset as usize + plan.len as usize;
                let Some(destination) = input.get_mut(plan.offset as usize..end) else {
                    return false;
                };
                destination.copy_from_slice(&source[..plan.len as usize]);
                true
            });
            if copied != Some(true) {
                crate::src::gzlib::gz_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(b"internal error: gzip write buffer missing\0"),
                );
                return 0 as crate::stdlib::z_size_t;
            }
            crate::src::gzlib::gz_buffered_copy_progress(state, &mut len, plan.len);
            source = &source[plan.len as usize..];
            if len == 0 as crate::stdlib::z_size_t {
                break;
            }
            if gz_comp_with_owned_input(state, crate::zlib_h::Z_NO_FLUSH)
                == -1 as ::core::ffi::c_int
            {
                return crate::src::gzlib::gz_write_error_result(state, put, len);
            }
        }
    } else {
        if state.strm.avail_in != 0
            && gz_comp_with_owned_input(state, crate::zlib_h::Z_NO_FLUSH)
                == -1 as ::core::ffi::c_int
        {
            return 0 as crate::stdlib::z_size_t;
        }
        state.strm.next_in = source.as_ptr() as *mut crate::stdlib::Bytef;
        loop {
            let mut n: ::core::ffi::c_uint = crate::src::gzlib::gz_stream_chunk(len);
            state.strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH, &source[..n as usize]);
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

// The reference-bound dispatcher owns write-state preflight and result
// handling after the ABI adapter has bound the caller buffer.
fn gzwrite_ffi_dispatch(
    state: &mut crate::gzguts_h::gz_state,
    len: ::core::ffi::c_uint,
    source: Option<&[::core::ffi::c_uchar]>,
) -> ::core::ffi::c_int {
    let prepared = gzwrite_preflight(state, len);
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

// Writing neither closes the handle nor invokes a user callback. Resolve its
// opaque address through the owned-state registry, leaving the exported
// adapter responsible only for binding the caller's readable range.
fn gzwrite_handle(
    file_key: usize,
    len: ::core::ffi::c_uint,
    source: Option<&[::core::ffi::c_uchar]>,
) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| {
        gzwrite_ffi_dispatch(state, len, source)
    })
    .unwrap_or(0)
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
    // SAFETY: C's `gzwrite` contract supplies `len` readable bytes when
    // `len` is nonzero. Stateful request validation remains in
    // `gzwrite_handle()` after registry lookup.
    let source = match crate::src::gzlib::gz_uint_request_fits_int(len)
        .then(|| crate::src::gzlib::gz_rust_slice_len(len as crate::stdlib::z_size_t))
        .flatten()
    {
        Some(0) => Some(&[] as &[::core::ffi::c_uchar]),
        Some(slice_len) if !buf.is_null() => Some(::core::slice::from_raw_parts(
            buf as *const ::core::ffi::c_uchar,
            slice_len,
        )),
        _ => None,
    };
    gzwrite_handle(file.addr(), len, source)
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
    nitems: crate::stdlib::z_size_t,
    source: Option<&[::core::ffi::c_uchar]>,
) -> crate::stdlib::z_size_t {
    let prepared = gzfwrite_preflight(state, size, nitems);
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

// As with `gzwrite_handle`, this is a non-closing, non-reentrant operation
// and can retain the registry borrow while it validates and consumes input.
fn gzfwrite_handle(
    file_key: usize,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
    source: Option<&[::core::ffi::c_uchar]>,
) -> crate::stdlib::z_size_t {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| {
        gzfwrite_ffi_dispatch(state, size, nitems, source)
    })
    .unwrap_or(0)
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
    // SAFETY: a nonempty, representable item request requires C to provide
    // that many readable bytes. Stateful request validation remains in
    // `gzfwrite_handle()` after registry lookup.
    let source = match crate::src::gzlib::gz_item_slice_len(size, nitems) {
        Some(slice_len) if slice_len != 0 && !buf.is_null() => Some(::core::slice::from_raw_parts(
            buf as *const ::core::ffi::c_uchar,
            slice_len,
        )),
        _ => None,
    };
    gzfwrite_handle(file.addr(), size, nitems, source)
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

// The opaque handle is keyed by address in gzip's owned-state registry. This
// keeps the public dispatch reference-bound without recreating a mutable
// reference from the foreign handle in the ABI wrapper.
fn gzputc_handle(file_key: usize, c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| gzputc_ffi_dispatch(Some(state), c))
        .unwrap_or(-1)
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzputc_handle(file.addr(), c)
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

// String output is non-closing and cannot re-enter through a user callback,
// so the registry can retain the state borrow for the complete operation.
fn gzputs_handle(file_key: usize, source: Option<&::core::ffi::CStr>) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| {
        gzputs_ffi_dispatch(Some(state), source)
    })
    .unwrap_or(-1)
}
#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let source = if s.is_null() {
        None
    } else {
        // SAFETY: C's `gzputs` contract supplies a non-null, nul-terminated
        // string. This exported adapter owns that caller-pointer conversion;
        // the dispatcher below owns all gzip-state preflight and operation
        // work.
        Some(unsafe { ::core::ffi::CStr::from_ptr(s) })
    };
    gzputs_handle(file.addr(), source)
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
    gz_comp_with_owned_input(state, flush);
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

// Flushing does not close the handle or invoke user callbacks, so it can use
// the same address-keyed state lookup as the other simple write operations.
fn gzflush_handle(file_key: usize, flush: ::core::ffi::c_int) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| {
        gzflush_ffi_dispatch(Some(state), flush)
    })
    .unwrap_or(crate::zlib_h::Z_STREAM_ERROR)
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzflush_handle(file.addr(), flush)
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
            && gz_comp_with_owned_input(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
        {
            return state.err;
        }
        // `deflateParams` validates the already-initialized stream/state
        // pair internally, so this write-state transition stays safe.
        let input = if state.strm.avail_in == 0 {
            Vec::new()
        } else {
            let offset = state.strm.next_in.addr().checked_sub(state.in_0.addr());
            let Some(input) = offset.and_then(|offset| {
                let end = offset.checked_add(state.strm.avail_in as usize)?;
                crate::src::gzlib::gz_with_owned_write_input_buffer(
                    crate::src::gzlib::gz_owned_buffer_key(state),
                    |input| input.get(offset..end).map(<[u8]>::to_vec),
                )
                .flatten()
            }) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            input
        };
        let state_key = crate::src::gzlib::gz_owned_buffer_key(state);
        let _ = crate::src::gzlib::gz_with_owned_write_output_buffer(state_key, |output| {
            let offset = state
                .strm
                .next_out
                .addr()
                .checked_sub(output.as_ptr().addr());
            let output = offset.and_then(|offset| {
                output.get_mut(offset..offset.checked_add(state.strm.avail_out as usize)?)
            });
            output.map_or(crate::zlib_h::Z_STREAM_ERROR, |output| {
                crate::src::deflate::deflateParams(&mut state.strm, level, strategy, &input, output)
            })
        });
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

// Parameter selection is likewise a non-closing write operation with no
// user callback, so keep its state validation behind the registry lookup.
fn gzsetparams_handle(
    file_key: usize,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    crate::src::gzlib::gz_with_owned_state(file_key, |state| {
        gzsetparams_ffi_dispatch(Some(state), level, strategy)
    })
    .unwrap_or(crate::zlib_h::Z_STREAM_ERROR)
}
#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzsetparams_handle(file.addr(), level, strategy)
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
    if gz_comp_with_owned_input(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int {
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
    match cleanup {
        crate::src::gzlib::GzWriteCloseCleanup::DeflaterAndBuffers => {
            crate::src::deflate::deflate_end_default_bound(&mut state.strm);
        }
        crate::src::gzlib::GzWriteCloseCleanup::None
        | crate::src::gzlib::GzWriteCloseCleanup::Input => {}
    }
    if !matches!(cleanup, crate::src::gzlib::GzWriteCloseCleanup::None) {
        crate::src::gzlib::gz_release_owned_write_buffers(state);
    }
    crate::src::gzlib::gzclearerr(state);
    let fd = state.fd;
    // The order matches zlib: close can override an earlier write result,
    // and the state allocation is released only after its fields are no
    // longer needed.
    crate::src::gzlib::gz_release_owned_strings(state);
    if crate::stdlib::close(fd) == -1 as ::core::ffi::c_int {
        ret = crate::zlib_h::Z_ERRNO;
    }
    crate::src::gzlib::gz_release_owned_state(state);
    return ret;
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    // See `gzclose_r_ffi`: close takes the registry-owned box, rather than
    // binding the opaque foreign handle as a mutable reference.
    let Some(mut state) =
        crate::src::gzlib::gz_take_owned_state_with_mode(file.addr(), crate::gzguts_h::GZ_WRITE)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzclose_w(&mut state)
}
