pub use crate::__stddef_null_h::NULL;
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
// The FFI wrapper validates and binds `file` before reaching this dispatcher.
// Selecting the read or write close coordinator only uses the bound state, so
// keep that selection safe and leave the raw handle conversion at the ABI edge.
pub fn gzclose(
    state: &mut crate::gzguts_h::gz_state,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    return if state.mode == crate::gzguts_h::GZ_READ {
        crate::src::gzread::gzclose_r(state, file as *mut crate::zlib_h::gzFile_s)
    } else {
        crate::src::gzwrite::gzclose_w(state, file as *mut crate::zlib_h::gzFile_s)
    };
}
#[export_name = "gzclose"]

pub unsafe extern "C" fn gzclose_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    gzclose(&mut *(file as crate::gzguts_h::gz_statep), file)
}
