pub use crate::__stddef_null_h::NULL;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_READ;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;

pub use crate::src::deflate::internal_state;
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

fn gz_close_action_for_mode(
    mode: Option<::core::ffi::c_int>,
) -> Result<GzCloseAction, ::core::ffi::c_int> {
    mode.map(GzCloseAction::for_mode)
        .ok_or(crate::zlib_h::Z_STREAM_ERROR)
}

#[export_name = "gzclose"]
pub unsafe extern "C" fn gzclose_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mode =
        unsafe { (file as *const crate::gzguts_h::gz_state).as_ref() }.map(|state| state.mode);

    match gz_close_action_for_mode(mode) {
        Err(status) => status,
        Ok(GzCloseAction::Read) => {
            crate::src::gzread::gzclose_r_ffi(file as *mut crate::zlib_h::gzFile_s)
        }
        Ok(GzCloseAction::Write) => {
            crate::src::gzwrite::gzclose_w(file as *mut crate::zlib_h::gzFile_s)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{gz_close_action_for_mode, GzCloseAction};

    #[test]
    fn dispatches_read_mode_to_read_close() {
        assert_eq!(
            gz_close_action_for_mode(Some(crate::gzguts_h::GZ_READ)),
            Ok(GzCloseAction::Read)
        );
    }

    #[test]
    fn dispatches_non_read_modes_to_write_close() {
        assert_eq!(
            gz_close_action_for_mode(Some(crate::gzguts_h::GZ_WRITE)),
            Ok(GzCloseAction::Write)
        );
        assert_eq!(
            gz_close_action_for_mode(Some(crate::gzguts_h::GZ_NONE)),
            Ok(GzCloseAction::Write)
        );
    }

    #[test]
    fn dispatches_unrecognized_modes_to_write_close() {
        assert_eq!(gz_close_action_for_mode(Some(-1)), Ok(GzCloseAction::Write));
    }

    #[test]
    fn dispatch_rejects_missing_mode_before_dispatch() {
        assert_eq!(
            gz_close_action_for_mode(None),
            Err(crate::zlib_h::Z_STREAM_ERROR)
        );
    }
}
