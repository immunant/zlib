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
                if (*state).size != 0 {
                    crate::src::inflate::inflateEnd(
                        &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                    );
                    crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
                    crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
                }
                let state_err = (*state).err;
                let message = (*state).msg;
                let release_message = crate::src::gzlib::gz_clear_error_should_release_message(
                    !message.is_null(),
                    (*state).err,
                );
                (*state).msg = ::core::ptr::null_mut();
                (*state).err = crate::zlib_h::Z_OK;
                if release_message {
                    crate::stdlib::free(message as *mut ::core::ffi::c_void);
                }
                crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
                let close_result = crate::stdlib::close((*state).fd);
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
                let mut ret = crate::zlib_h::Z_OK;
                if (*state).skip != 0
                    && crate::src::gzwrite::gz_zero(state) == -1 as ::core::ffi::c_int
                {
                    ret = (*state).err;
                }
                if crate::src::gzwrite::gz_comp(state, crate::zlib_h::Z_FINISH)
                    == -1 as ::core::ffi::c_int
                {
                    ret = (*state).err;
                }
                if (*state).size != 0 {
                    if (*state).direct == 0 {
                        crate::src::deflate::deflateEnd(
                            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                        );
                        crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
                    }
                    crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
                }
                let message = (*state).msg;
                let release_message = crate::src::gzlib::gz_clear_error_should_release_message(
                    !message.is_null(),
                    (*state).err,
                );
                (*state).msg = ::core::ptr::null_mut();
                (*state).err = crate::zlib_h::Z_OK;
                if release_message {
                    crate::stdlib::free(message as *mut ::core::ffi::c_void);
                }
                crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
                let close_result = crate::stdlib::close((*state).fd);
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
