pub use crate::__stddef_null_h::NULL;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_READ;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;

pub use crate::src::deflate::internal_state;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gzFile;
pub use crate::zlib_h::gzFile_s;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::Z_STREAM_ERROR;

/// Clear the read-side resources owned by opaque gzip state after its codec
/// has been stopped at the ABI boundary.  The descriptor remains open here:
/// callers must close it before releasing the ABI allocation.
pub(crate) fn gzclose_read_release_state(
    state: &mut crate::gzguts_h::gz_state,
) -> (::core::ffi::c_int, ::core::ffi::c_int) {
    state.buffers = None;
    let err = state.err;
    state.msg = None;
    state.err = crate::zlib_h::Z_OK;
    let fd = state
        .file
        .take()
        .map(::std::os::fd::IntoRawFd::into_raw_fd)
        .unwrap_or(state.fd);
    state.fd = -1;
    (fd, err)
}

/// Clear the write-side resources owned by opaque gzip state after its codec
/// has been flushed and stopped at the ABI boundary.  This deliberately does
/// not close the descriptor or destroy the ABI allocation.
pub(crate) fn gzclose_write_release_state(
    state: &mut crate::gzguts_h::gz_state,
) -> ::core::ffi::c_int {
    state.buffers = None;
    state.msg = None;
    state.err = crate::zlib_h::Z_OK;
    let fd = state
        .file
        .take()
        .map(::std::os::fd::IntoRawFd::into_raw_fd)
        .unwrap_or(state.fd);
    state.fd = -1;
    fd
}

// These macros are deliberately invoked only by exported close entry points.
// They keep destruction of the malloc-backed ABI state, descriptor close,
// and the legacy compressor/inflater calls at that boundary.
macro_rules! gzclose_read_at_boundary {
    ($file:expr) => {{
        let file = $file;
        if file.is_null() {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let state = file as crate::gzguts_h::gz_statep;
            if (*state).mode != crate::gzguts_h::GZ_READ {
                crate::zlib_h::Z_STREAM_ERROR
            } else {
                let state_ref = &mut *state;
                if state_ref.size != 0 {
                    crate::src::inflate::inflate_end_at_boundary!(
                        &raw mut state_ref.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                    );
                }
                let (fd, state_err) = crate::src::gzclose::gzclose_read_release_state(state_ref);
                let close_result = crate::stdlib::close(fd);
                ::core::ptr::drop_in_place(state);
                crate::stdlib::free(state as *mut ::core::ffi::c_void);
                crate::src::gzread::gzclose_read_result(state_err, close_result)
            }
        }
    }};
}
pub(crate) use gzclose_read_at_boundary;

macro_rules! gzclose_write_at_boundary {
    ($file:expr) => {{
        let file = $file;
        if file.is_null() {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let state = file as crate::gzguts_h::gz_statep;
            if (*state).mode != crate::gzguts_h::GZ_WRITE {
                crate::zlib_h::Z_STREAM_ERROR
            } else {
                let state_ref = &mut *state;
                let mut ret = crate::zlib_h::Z_OK;
                if state_ref.skip != 0
                    && crate::src::gzwrite::gz_zero_at_boundary!(state_ref)
                        == -1 as ::core::ffi::c_int
                {
                    ret = state_ref.err;
                }
                if crate::src::gzwrite::gz_comp_at_boundary!(state_ref, crate::zlib_h::Z_FINISH)
                    == -1 as ::core::ffi::c_int
                {
                    ret = state_ref.err;
                }
                if state_ref.size != 0 {
                    if state_ref.direct == 0 {
                        crate::src::deflate::deflate_end_at_boundary!(&mut state_ref.strm);
                    }
                }
                let fd = crate::src::gzclose::gzclose_write_release_state(state_ref);
                let close_result = crate::stdlib::close(fd);
                ::core::ptr::drop_in_place(state);
                crate::stdlib::free(state as *mut ::core::ffi::c_void);
                crate::src::gzwrite::gzclose_write_result(close_result, ret)
            }
        }
    }};
}
pub(crate) use gzclose_write_at_boundary;

#[export_name = "gzclose"]
pub unsafe extern "C" fn gzclose_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if state.mode == crate::gzguts_h::GZ_READ {
        gzclose_read_at_boundary!(file)
    } else {
        gzclose_write_at_boundary!(file)
    }
}
