pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;
static Z_ERRMSG_NEED_DICT: &[u8] = b"need dictionary\0";
static Z_ERRMSG_STREAM_END: &[u8] = b"stream end\0";
static Z_ERRMSG_EMPTY: &[u8] = b"\0";
static Z_ERRMSG_FILE: &[u8] = b"file error\0";
static Z_ERRMSG_STREAM: &[u8] = b"stream error\0";
static Z_ERRMSG_DATA: &[u8] = b"data error\0";
static Z_ERRMSG_MEMORY: &[u8] = b"insufficient memory\0";
static Z_ERRMSG_BUFFER: &[u8] = b"buffer error\0";
static Z_ERRMSG_VERSION: &[u8] = b"incompatible version\0";
#[no_mangle]

pub static mut z_errmsg: [*mut ::core::ffi::c_char; 10] = [
    Z_ERRMSG_NEED_DICT.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    Z_ERRMSG_STREAM_END.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    Z_ERRMSG_EMPTY.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    Z_ERRMSG_FILE.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    Z_ERRMSG_STREAM.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    Z_ERRMSG_DATA.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    Z_ERRMSG_MEMORY.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    Z_ERRMSG_BUFFER.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    Z_ERRMSG_VERSION.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    Z_ERRMSG_EMPTY.as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];

/// Return zlib's fixed diagnostic for an error code without reading the
/// legacy mutable C export above.  The export remains for C ABI compatibility.
pub fn z_error_message(error: ::core::ffi::c_int) -> &'static ::core::ffi::CStr {
    let message = match error {
        2 => Z_ERRMSG_NEED_DICT,
        1 => Z_ERRMSG_STREAM_END,
        0 => Z_ERRMSG_EMPTY,
        -1 => Z_ERRMSG_FILE,
        -2 => Z_ERRMSG_STREAM,
        -3 => Z_ERRMSG_DATA,
        -4 => Z_ERRMSG_MEMORY,
        -5 => Z_ERRMSG_BUFFER,
        -6 => Z_ERRMSG_VERSION,
        _ => Z_ERRMSG_EMPTY,
    };
    ::core::ffi::CStr::from_bytes_with_nul(message)
        .expect("fixed zlib error strings are NUL-terminated")
}
pub fn zlibVersion() -> &'static ::core::ffi::CStr {
    c"1.3.2.1-motley"
}
#[export_name = "zlibVersion"]

pub unsafe extern "C" fn zlibVersion_ffi() -> *const ::core::ffi::c_char {
    zlibVersion().as_ptr()
}
pub fn zlibCompileFlags() -> crate::stdlib::uLong {
    let mut flags = 0;
    match ::core::mem::size_of::<crate::stdlib::uInt>() {
        2 => {}
        4 => {
            flags |= 1;
        }
        8 => {
            flags |= 2;
        }
        _ => {
            flags |= 3;
        }
    }
    match ::core::mem::size_of::<crate::stdlib::uLong>() {
        2 => {}
        4 => {
            flags |= 1 << 2;
        }
        8 => {
            flags |= 2 << 2;
        }
        _ => {
            flags |= 3 << 2;
        }
    }
    match ::core::mem::size_of::<usize>() {
        2 => {}
        4 => {
            flags |= 1 << 4;
        }
        8 => {
            flags |= 2 << 4;
        }
        _ => {
            flags |= 3 << 4;
        }
    }
    match ::core::mem::size_of::<crate::stdlib::off_t>() {
        2 => {}
        4 => {
            flags |= 1 << 6;
        }
        8 => {
            flags |= 2 << 6;
        }
        _ => {
            flags |= 3 << 6;
        }
    }
    flags
}
#[export_name = "zlibCompileFlags"]

pub unsafe extern "C" fn zlibCompileFlags_ffi() -> crate::stdlib::uLong {
    zlibCompileFlags()
}
pub fn zError(err: ::core::ffi::c_int) -> &'static ::core::ffi::CStr {
    z_error_message(err)
}
#[export_name = "zError"]

pub unsafe extern "C" fn zError_ffi(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    zError(err).as_ptr()
}
pub extern "C" fn zcalloc(
    _opaque: crate::zlib_h::Opaque,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    return if ::core::mem::size_of::<crate::stdlib::uInt>() > 2 as usize {
        crate::stdlib::malloc(items.wrapping_mul(size) as crate::__stddef_size_t_h::size_t)
    } else {
        crate::stdlib::calloc(
            items as crate::__stddef_size_t_h::size_t,
            size as crate::__stddef_size_t_h::size_t,
        )
    };
}
#[export_name = "zcalloc"]

pub unsafe extern "C" fn zcalloc_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    zcalloc(
        crate::zlib_h::Opaque::from_address(opaque.addr()),
        items,
        size,
    )
}
pub extern "C" fn zcfree(_opaque: crate::zlib_h::Opaque, mut ptr: crate::stdlib::voidpf) {
    crate::stdlib::free(ptr as *mut ::core::ffi::c_void);
}
#[export_name = "zcfree"]

pub unsafe extern "C" fn zcfree_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut ptr: crate::stdlib::voidpf,
) {
    zcfree(crate::zlib_h::Opaque::from_address(opaque.addr()), ptr)
}
