pub use crate::__stddef_null_h::NULL;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_READ;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::gzread::gzclose_r_ffi;
pub use crate::src::gzwrite::gzclose_w_ffi;
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

enum GzCloseRoute {
    Read,
    Write,
}

fn gzclose_route(mode: ::core::ffi::c_int) -> GzCloseRoute {
    if mode == crate::gzguts_h::GZ_READ {
        GzCloseRoute::Read
    } else {
        GzCloseRoute::Write
    }
}

#[export_name = "gzclose"]

pub unsafe extern "C" fn gzclose_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let route = {
        let state = &*(file as crate::gzguts_h::gz_statep);
        gzclose_route(state.mode)
    };
    match route {
        GzCloseRoute::Read => {
            return crate::src::gzread::gzclose_r_ffi(file as *mut crate::zlib_h::gzFile_s);
        }
        GzCloseRoute::Write => {
            return crate::src::gzwrite::gzclose_w_ffi(file as *mut crate::zlib_h::gzFile_s);
        }
    }
}
