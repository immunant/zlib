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

fn gz_close_uses_read_close(mode: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_READ
}

fn gz_close_dispatch_mode(mode: Option<::core::ffi::c_int>) -> Result<bool, ::core::ffi::c_int> {
    mode.map(gz_close_uses_read_close)
        .ok_or(crate::zlib_h::Z_STREAM_ERROR)
}

#[export_name = "gzclose"]
pub unsafe extern "C" fn gzclose_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mode = if file.is_null() {
        None
    } else {
        Some(unsafe { (*(file as *const crate::gzguts_h::gz_state)).mode })
    };

    match gz_close_dispatch_mode(mode) {
        Err(status) => status,
        Ok(true) => unsafe { crate::src::gzread::gzclose_r(file as *mut crate::zlib_h::gzFile_s) },
        Ok(false) => unsafe {
            crate::src::gzwrite::gzclose_w(file as *mut crate::zlib_h::gzFile_s)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{gz_close_dispatch_mode, gz_close_uses_read_close};

    #[test]
    fn read_close_is_used_only_for_read_mode() {
        assert!(gz_close_uses_read_close(crate::gzguts_h::GZ_READ));
        assert!(!gz_close_uses_read_close(crate::gzguts_h::GZ_WRITE));
        assert!(!gz_close_uses_read_close(crate::gzguts_h::GZ_NONE));
    }

    #[test]
    fn dispatches_read_mode_to_read_close() {
        assert_eq!(
            gz_close_dispatch_mode(Some(crate::gzguts_h::GZ_READ)),
            Ok(true)
        );
    }

    #[test]
    fn dispatches_non_read_modes_to_write_close() {
        assert_eq!(
            gz_close_dispatch_mode(Some(crate::gzguts_h::GZ_WRITE)),
            Ok(false)
        );
        assert_eq!(
            gz_close_dispatch_mode(Some(crate::gzguts_h::GZ_NONE)),
            Ok(false)
        );
    }

    #[test]
    fn dispatch_rejects_missing_mode_before_dispatch() {
        assert_eq!(
            gz_close_dispatch_mode(None),
            Err(crate::zlib_h::Z_STREAM_ERROR)
        );
    }
}
