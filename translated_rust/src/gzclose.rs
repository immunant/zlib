pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_READ;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::gzread::gzclose_r;
pub use crate::src::gzwrite::gzclose_w;
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
pub fn gzclose(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode == crate::gzguts_h::GZ_READ {
        // The state reference is valid for this synchronous close operation.
        unsafe {
            crate::src::gzread::gzclose_r(
                state as *mut crate::gzguts_h::gz_state as crate::zlib_h::gzFile,
            )
        }
    } else {
        // The state reference is valid for this synchronous close operation.
        unsafe {
            crate::src::gzwrite::gzclose_w(
                state as *mut crate::gzguts_h::gz_state as crate::zlib_h::gzFile,
            )
        }
    }
}
#[export_name = "gzclose"]

pub unsafe extern "C" fn gzclose_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzclose(state)
}
