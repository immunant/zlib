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

#[derive(Debug, PartialEq, Eq)]
enum GzCloseMode {
    Read,
    Write,
}

fn gz_close_mode(mode: ::core::ffi::c_int) -> GzCloseMode {
    if mode == crate::gzguts_h::GZ_READ {
        GzCloseMode::Read
    } else {
        GzCloseMode::Write
    }
}

fn gz_close_uses_read_handler(mode: ::core::ffi::c_int) -> bool {
    gz_close_mode(mode) == GzCloseMode::Read
}

fn gz_close_validation_status(file_is_null: bool) -> Result<(), ::core::ffi::c_int> {
    if file_is_null {
        Err(crate::zlib_h::Z_STREAM_ERROR)
    } else {
        Ok(())
    }
}

#[export_name = "gzclose"]
pub unsafe extern "C" fn gzclose_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if let Err(status) = gz_close_validation_status(file.is_null()) {
        return status;
    }

    let state = unsafe { &*(file as *const crate::gzguts_h::gz_state) };
    if gz_close_uses_read_handler(state.mode) {
        unsafe { crate::src::gzread::gzclose_r(file as *mut crate::zlib_h::gzFile_s) }
    } else {
        unsafe { crate::src::gzwrite::gzclose_w(file as *mut crate::zlib_h::gzFile_s) }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        gz_close_mode, gz_close_uses_read_handler, gz_close_validation_status, GzCloseMode,
    };

    #[test]
    fn selects_read_close_for_read_mode() {
        assert_eq!(gz_close_mode(crate::gzguts_h::GZ_READ), GzCloseMode::Read);
    }

    #[test]
    fn selects_write_close_for_non_read_modes() {
        assert_eq!(gz_close_mode(crate::gzguts_h::GZ_WRITE), GzCloseMode::Write);
        assert_eq!(gz_close_mode(crate::gzguts_h::GZ_NONE), GzCloseMode::Write);
    }

    #[test]
    fn uses_read_handler_only_for_read_mode() {
        assert!(gz_close_uses_read_handler(crate::gzguts_h::GZ_READ));
        assert!(!gz_close_uses_read_handler(crate::gzguts_h::GZ_WRITE));
        assert!(!gz_close_uses_read_handler(crate::gzguts_h::GZ_NONE));
    }

    #[test]
    fn rejects_null_file_with_stream_error() {
        assert_eq!(
            gz_close_validation_status(true),
            Err(crate::zlib_h::Z_STREAM_ERROR)
        );
    }

    #[test]
    fn accepts_non_null_file() {
        assert_eq!(gz_close_validation_status(false), Ok(()));
    }
}
