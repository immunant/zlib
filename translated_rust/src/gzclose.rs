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

pub(crate) enum GzCloseTarget {
    Any,
    Read,
    Write,
}

pub(crate) unsafe fn gzclose(
    state: Option<::core::ptr::NonNull<crate::gzguts_h::gz_state>>,
    target: GzCloseTarget,
) -> ::core::ffi::c_int {
    let Some(mut state) = state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let state_ptr = state.as_ptr();
    let state = state.as_mut();
    let close: unsafe fn(&mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int = match target {
        GzCloseTarget::Any => {
            if state.mode == crate::gzguts_h::GZ_READ {
                crate::src::gzread::gzclose_r
            } else {
                crate::src::gzwrite::gzclose_w
            }
        }
        GzCloseTarget::Read if state.mode == crate::gzguts_h::GZ_READ => {
            crate::src::gzread::gzclose_r
        }
        GzCloseTarget::Write if state.mode == crate::gzguts_h::GZ_WRITE => {
            crate::src::gzwrite::gzclose_w
        }
        GzCloseTarget::Read | GzCloseTarget::Write => return crate::zlib_h::Z_STREAM_ERROR,
    };
    let ret = close(state);
    // `gz_open()` allocated this opaque handle as a one-element Vec.  The
    // selected close path has released its owned resources, so reclaim that
    // allocation exactly once after mode-dependent cleanup.
    drop(Vec::from_raw_parts(state_ptr, 1, 1));
    ret
}
#[export_name = "gzclose"]

pub unsafe extern "C" fn gzclose_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose(
        ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep),
        GzCloseTarget::Any,
    )
}
