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

/// Whether a close implementation consumed the heap allocation that backs a
/// gzip handle.  A mismatched `gzclose_[rw]` must leave the handle valid, just
/// as zlib's C entry points do.
pub enum GzCloseResult {
    Closed(::core::ffi::c_int),
    Retained(::core::ffi::c_int),
}

pub fn gzclose(state: &mut crate::gzguts_h::gz_state) -> GzCloseResult {
    if state.mode == crate::gzguts_h::GZ_READ {
        crate::src::gzread::gzclose_r(state)
    } else {
        crate::src::gzwrite::gzclose_w(state)
    }
}
#[export_name = "gzclose"]

pub unsafe extern "C" fn gzclose_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Keep raw ownership conversion at the ABI boundary.  The safe close core
    // retains mismatched handles, so only a completed close recreates and
    // drops the Box.
    let state = Box::leak(Box::from_raw(file as crate::gzguts_h::gz_statep));
    match gzclose(state) {
        GzCloseResult::Closed(result) => {
            drop(Box::from_raw(file as crate::gzguts_h::gz_statep));
            result
        }
        GzCloseResult::Retained(result) => result,
    }
}
