pub use crate::__stddef_null_h::NULL;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_READ;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::gzread::gzclose_r;
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

fn gz_mode(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    state.mode
}

pub unsafe fn gzclose(owned: Box<crate::gzguts_h::gz_state>) -> ::core::ffi::c_int {
    if gz_mode(owned.as_ref()) == crate::gzguts_h::GZ_READ {
        crate::src::gzread::gzclose_r(owned)
    } else {
        crate::src::gzwrite::gzclose_w_impl(owned)
    }
}
#[export_name = "gzclose"]

pub unsafe extern "C" fn gzclose_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let owned = Box::from_raw(file.cast::<crate::gzguts_h::gz_state>());
    gzclose(owned)
}
