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

#[derive(Debug, Eq, PartialEq)]
enum GzCloseAction {
    Read,
    Write,
}

impl GzCloseAction {
    fn for_mode(mode: ::core::ffi::c_int) -> Self {
        match mode {
            crate::gzguts_h::GZ_READ => Self::Read,
            _ => Self::Write,
        }
    }
}

#[export_name = "gzclose"]
pub unsafe extern "C" fn gzclose_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let state = file as *const crate::gzguts_h::gz_state;
    if state.is_null()
        || state.align_offset(::core::mem::align_of::<crate::gzguts_h::gz_state>()) != 0
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mode = unsafe { (*state).mode };

    match GzCloseAction::for_mode(mode) {
        GzCloseAction::Read => {
            crate::src::gzread::gzclose_r_ffi(file as *mut crate::zlib_h::gzFile_s)
        }
        GzCloseAction::Write => {
            crate::src::gzwrite::gzclose_w_ffi(file as *mut crate::zlib_h::gzFile_s)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{gzclose_ffi, GzCloseAction};

    #[test]
    fn dispatches_read_mode_to_read_close() {
        assert_eq!(
            GzCloseAction::for_mode(crate::gzguts_h::GZ_READ),
            GzCloseAction::Read
        );
    }

    #[test]
    fn dispatches_non_read_modes_to_write_close() {
        assert_eq!(
            GzCloseAction::for_mode(crate::gzguts_h::GZ_WRITE),
            GzCloseAction::Write
        );
        assert_eq!(
            GzCloseAction::for_mode(crate::gzguts_h::GZ_NONE),
            GzCloseAction::Write
        );
    }

    #[test]
    fn dispatches_unrecognized_modes_to_write_close() {
        assert_eq!(GzCloseAction::for_mode(-1), GzCloseAction::Write);
    }

    #[test]
    fn close_rejects_null_handle_without_dispatching() {
        assert_eq!(
            unsafe { gzclose_ffi(core::ptr::null_mut()) },
            crate::zlib_h::Z_STREAM_ERROR
        );
    }

    #[test]
    fn close_rejects_misaligned_handle_without_dispatching() {
        assert!(::core::mem::align_of::<crate::gzguts_h::gz_state>() > 1);
        let mut bytes = [0_u8; ::core::mem::size_of::<crate::gzguts_h::gz_state>() + 1];
        let misaligned = bytes.as_mut_ptr().wrapping_add(1) as crate::zlib_h::gzFile;

        assert_eq!(
            unsafe { gzclose_ffi(misaligned) },
            crate::zlib_h::Z_STREAM_ERROR
        );
    }
}
