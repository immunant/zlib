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
pub enum GzClose {
    Read(crate::src::gzread::GzCloseRead),
    Write(crate::src::gzwrite::GzCloseWrite),
}

pub fn gzclose(state: &mut crate::gzguts_h::gz_state) -> GzClose {
    if state.mode == crate::gzguts_h::GZ_READ {
        GzClose::Read(crate::src::gzread::gzclose_r(state))
    } else {
        GzClose::Write(crate::src::gzwrite::gzclose_w(state))
    }
}
#[export_name = "gzclose"]

pub unsafe extern "C" fn gzclose_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    match gzclose(state) {
        GzClose::Read(close) => {
            if !close.valid {
                return close.result;
            }
            if close.end_inflater {
                crate::src::inflate::inflateEnd(&mut state.strm);
            }
            crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
            let read_file = state.read_file.take();
            drop(Box::from_raw(state));
            drop(read_file);
            close.result
        }
        GzClose::Write(close) => {
            if !close.valid {
                return close.result;
            }
            if close.end_deflater {
                crate::src::deflate::deflateEnd(&mut state.strm);
            }
            crate::src::gzlib::gz_error_safe(state, crate::zlib_h::Z_OK, None);
            let mut result = close.result;
            if let Some(file) = state.write_file.take() {
                drop(file);
            } else if crate::stdlib::close(state.fd) == -1 {
                result = crate::zlib_h::Z_ERRNO;
            }
            drop(Box::from_raw(state));
            result
        }
    }
}
