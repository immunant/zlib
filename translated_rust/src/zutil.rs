pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;
use core::ffi::CStr;

const ZLIB_VERSION_TEXT: &CStr = c"1.3.2.1-motley";
static ERROR_NEED_DICT: [u8; 16] = *b"need dictionary\0";
static ERROR_STREAM_END: [u8; 11] = *b"stream end\0";
static ERROR_EMPTY: [u8; 1] = *b"\0";
static ERROR_FILE: [u8; 11] = *b"file error\0";
static ERROR_STREAM: [u8; 13] = *b"stream error\0";
static ERROR_DATA: [u8; 11] = *b"data error\0";
static ERROR_MEMORY: [u8; 20] = *b"insufficient memory\0";
static ERROR_BUFFER: [u8; 13] = *b"buffer error\0";
static ERROR_VERSION: [u8; 21] = *b"incompatible version\0";
#[no_mangle]

pub static z_errmsg: [::core::sync::atomic::AtomicPtr<::core::ffi::c_char>; 10] = [
    ::core::sync::atomic::AtomicPtr::new(ERROR_NEED_DICT.as_ptr() as *mut ::core::ffi::c_char),
    ::core::sync::atomic::AtomicPtr::new(ERROR_STREAM_END.as_ptr() as *mut ::core::ffi::c_char),
    ::core::sync::atomic::AtomicPtr::new(ERROR_EMPTY.as_ptr() as *mut ::core::ffi::c_char),
    ::core::sync::atomic::AtomicPtr::new(ERROR_FILE.as_ptr() as *mut ::core::ffi::c_char),
    ::core::sync::atomic::AtomicPtr::new(ERROR_STREAM.as_ptr() as *mut ::core::ffi::c_char),
    ::core::sync::atomic::AtomicPtr::new(ERROR_DATA.as_ptr() as *mut ::core::ffi::c_char),
    ::core::sync::atomic::AtomicPtr::new(ERROR_MEMORY.as_ptr() as *mut ::core::ffi::c_char),
    ::core::sync::atomic::AtomicPtr::new(ERROR_BUFFER.as_ptr() as *mut ::core::ffi::c_char),
    ::core::sync::atomic::AtomicPtr::new(ERROR_VERSION.as_ptr() as *mut ::core::ffi::c_char),
    ::core::sync::atomic::AtomicPtr::new(ERROR_EMPTY.as_ptr() as *mut ::core::ffi::c_char),
];

static ERROR_MESSAGES: [&[u8]; 10] = [
    &ERROR_NEED_DICT,
    &ERROR_STREAM_END,
    &ERROR_EMPTY,
    &ERROR_FILE,
    &ERROR_STREAM,
    &ERROR_DATA,
    &ERROR_MEMORY,
    &ERROR_BUFFER,
    &ERROR_VERSION,
    &ERROR_EMPTY,
];
fn zlib_version() -> &'static CStr {
    ZLIB_VERSION_TEXT
}
#[export_name = "zlibVersion"]

pub unsafe extern "C" fn zlibVersion_ffi() -> *const ::core::ffi::c_char {
    zlib_version().as_ptr()
}
pub fn zlibCompileFlags() -> crate::stdlib::uLong {
    let mut flags: crate::stdlib::uLong = 0;
    flags = 0 as crate::stdlib::uLong;
    match ::core::mem::size_of::<crate::stdlib::uInt>() as ::core::ffi::c_int {
        2 => {}
        4 => {
            flags = flags.wrapping_add(1 as crate::stdlib::uLong);
        }
        8 => {
            flags = flags.wrapping_add(2 as crate::stdlib::uLong);
        }
        _ => {
            flags = flags.wrapping_add(3 as crate::stdlib::uLong);
        }
    }
    match ::core::mem::size_of::<crate::stdlib::uLong>() as ::core::ffi::c_int {
        2 => {}
        4 => {
            flags = flags.wrapping_add(
                ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as crate::stdlib::uLong,
            );
        }
        8 => {
            flags = flags.wrapping_add(
                ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as crate::stdlib::uLong,
            );
        }
        _ => {
            flags = flags.wrapping_add(
                ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as crate::stdlib::uLong,
            );
        }
    }
    match ::core::mem::size_of::<crate::stdlib::voidpf>() as ::core::ffi::c_int {
        2 => {}
        4 => {
            flags = flags.wrapping_add(
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as crate::stdlib::uLong,
            );
        }
        8 => {
            flags = flags.wrapping_add(
                ((2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as crate::stdlib::uLong,
            );
        }
        _ => {
            flags = flags.wrapping_add(
                ((3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as crate::stdlib::uLong,
            );
        }
    }
    match ::core::mem::size_of::<crate::stdlib::off_t>() as ::core::ffi::c_int {
        2 => {}
        4 => {
            flags = flags.wrapping_add(
                ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as crate::stdlib::uLong,
            );
        }
        8 => {
            flags = flags.wrapping_add(
                ((2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as crate::stdlib::uLong,
            );
        }
        _ => {
            flags = flags.wrapping_add(
                ((3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as crate::stdlib::uLong,
            );
        }
    }
    return flags;
}
#[export_name = "zlibCompileFlags"]

pub unsafe extern "C" fn zlibCompileFlags_ffi() -> crate::stdlib::uLong {
    zlibCompileFlags()
}
pub fn zError(err: ::core::ffi::c_int) -> &'static [u8] {
    &ERROR_MESSAGES[(if err < -6 as ::core::ffi::c_int || err > 2 as ::core::ffi::c_int {
        9 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int - err
    }) as usize]
}
#[export_name = "zError"]

pub unsafe extern "C" fn zError_ffi(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    zError(err).as_ptr().cast()
}
pub unsafe extern "C" fn zcalloc(
    _opaque: crate::stdlib::voidpf,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    crate::stdlib::malloc(items.wrapping_mul(size) as crate::__stddef_size_t_h::size_t)
}
#[export_name = "zcalloc"]

pub unsafe extern "C" fn zcalloc_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    zcalloc(opaque, items, size)
}
pub unsafe extern "C" fn zcfree(_opaque: crate::stdlib::voidpf, mut ptr: crate::stdlib::voidpf) {
    crate::stdlib::free(ptr as *mut ::core::ffi::c_void);
}
#[export_name = "zcfree"]

pub unsafe extern "C" fn zcfree_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut ptr: crate::stdlib::voidpf,
) {
    zcfree(opaque, ptr)
}
