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

#[derive(Debug, PartialEq, Eq)]
enum GzCloseAction {
    Return(::core::ffi::c_int),
    Close(GzCloseMode),
}

fn gz_close_uses_read_close(mode: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_READ
}

fn gz_close_mode(mode: ::core::ffi::c_int) -> GzCloseMode {
    if gz_close_uses_read_close(mode) {
        GzCloseMode::Read
    } else {
        GzCloseMode::Write
    }
}

fn gz_close_action(file_is_null: bool, mode: ::core::ffi::c_int) -> GzCloseAction {
    if file_is_null {
        GzCloseAction::Return(crate::zlib_h::Z_STREAM_ERROR)
    } else {
        GzCloseAction::Close(gz_close_mode(mode))
    }
}

#[export_name = "gzclose"]
pub unsafe extern "C" fn gzclose_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let file_is_null = file.is_null();
    let mode = if file_is_null {
        crate::gzguts_h::GZ_NONE
    } else {
        unsafe { (*(file as *const crate::gzguts_h::gz_state)).mode }
    };

    match gz_close_action(file_is_null, mode) {
        GzCloseAction::Return(status) => status,
        GzCloseAction::Close(GzCloseMode::Read) => unsafe {
            crate::src::gzread::gzclose_r(file as *mut crate::zlib_h::gzFile_s)
        },
        GzCloseAction::Close(GzCloseMode::Write) => unsafe {
            crate::src::gzwrite::gzclose_w(file as *mut crate::zlib_h::gzFile_s)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        gz_close_action, gz_close_mode, gz_close_uses_read_close, GzCloseAction, GzCloseMode,
    };

    #[test]
    fn read_close_is_used_only_for_read_mode() {
        assert!(gz_close_uses_read_close(crate::gzguts_h::GZ_READ));
        assert!(!gz_close_uses_read_close(crate::gzguts_h::GZ_WRITE));
        assert!(!gz_close_uses_read_close(crate::gzguts_h::GZ_NONE));
    }

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
    fn close_action_routes_read_mode_to_read_close() {
        assert_eq!(
            gz_close_action(false, crate::gzguts_h::GZ_READ),
            GzCloseAction::Close(GzCloseMode::Read)
        );
    }

    #[test]
    fn close_action_routes_non_read_modes_to_write_close() {
        assert_eq!(
            gz_close_action(false, crate::gzguts_h::GZ_WRITE),
            GzCloseAction::Close(GzCloseMode::Write)
        );
        assert_eq!(
            gz_close_action(false, crate::gzguts_h::GZ_NONE),
            GzCloseAction::Close(GzCloseMode::Write)
        );
    }

    #[test]
    fn close_action_rejects_null_file_before_mode_dispatch() {
        assert_eq!(
            gz_close_action(true, crate::gzguts_h::GZ_READ),
            GzCloseAction::Return(crate::zlib_h::Z_STREAM_ERROR)
        );
        assert_eq!(
            gz_close_action(true, crate::gzguts_h::GZ_WRITE),
            GzCloseAction::Return(crate::zlib_h::Z_STREAM_ERROR)
        );
    }
}
