pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;

pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;

static Z_ERRMSG_NEED_DICT: [::core::ffi::c_char; 16] = crate::c_char_array(b"need dictionary\0");
static Z_ERRMSG_STREAM_END: [::core::ffi::c_char; 11] = crate::c_char_array(b"stream end\0");
static Z_ERRMSG_EMPTY: [::core::ffi::c_char; 1] = crate::c_char_array(b"\0");
static Z_ERRMSG_FILE_ERROR: [::core::ffi::c_char; 11] = crate::c_char_array(b"file error\0");
static Z_ERRMSG_STREAM_ERROR: [::core::ffi::c_char; 13] = crate::c_char_array(b"stream error\0");
static Z_ERRMSG_DATA_ERROR: [::core::ffi::c_char; 11] = crate::c_char_array(b"data error\0");
static Z_ERRMSG_MEM_ERROR: [::core::ffi::c_char; 20] =
    crate::c_char_array(b"insufficient memory\0");
static Z_ERRMSG_BUF_ERROR: [::core::ffi::c_char; 13] = crate::c_char_array(b"buffer error\0");
static Z_ERRMSG_VERSION_ERROR: [::core::ffi::c_char; 21] =
    crate::c_char_array(b"incompatible version\0");

#[no_mangle]
pub static z_errmsg: [&'static ::core::ffi::c_char; 10] = [
    &Z_ERRMSG_NEED_DICT[0],
    &Z_ERRMSG_STREAM_END[0],
    &Z_ERRMSG_EMPTY[0],
    &Z_ERRMSG_FILE_ERROR[0],
    &Z_ERRMSG_STREAM_ERROR[0],
    &Z_ERRMSG_DATA_ERROR[0],
    &Z_ERRMSG_MEM_ERROR[0],
    &Z_ERRMSG_BUF_ERROR[0],
    &Z_ERRMSG_VERSION_ERROR[0],
    &Z_ERRMSG_EMPTY[0],
];
pub fn zlibVersion() -> &'static [::core::ffi::c_char; 15] {
    return &crate::zlib_h::ZLIB_VERSION;
}
#[export_name = "zlibVersion"]

pub unsafe extern "C" fn zlibVersion_ffi() -> *const ::core::ffi::c_char {
    zlibVersion().as_ptr()
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
fn z_error_index(err: ::core::ffi::c_int) -> usize {
    return (if err < -6 as ::core::ffi::c_int || err > 2 as ::core::ffi::c_int {
        9 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int - err
    }) as usize;
}

pub fn zError(err: ::core::ffi::c_int) -> &'static [u8] {
    return match z_error_index(err) {
        0 => b"need dictionary\0",
        1 => b"stream end\0",
        2 => b"\0",
        3 => b"file error\0",
        4 => b"stream error\0",
        5 => b"data error\0",
        6 => b"insufficient memory\0",
        7 => b"buffer error\0",
        8 => b"incompatible version\0",
        _ => b"\0",
    };
}
#[export_name = "zError"]

pub unsafe extern "C" fn zError_ffi(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    zError(err).as_ptr() as *const ::core::ffi::c_char
}
#[export_name = "zcalloc"]

pub unsafe extern "C" fn zcalloc_ffi(
    mut _opaque: crate::stdlib::voidpf,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    return if ::core::mem::size_of::<crate::stdlib::uInt>() as usize > 2 as usize {
        crate::stdlib::malloc(items.wrapping_mul(size) as crate::__stddef_size_t_h::size_t)
    } else {
        crate::stdlib::calloc(
            items as crate::__stddef_size_t_h::size_t,
            size as crate::__stddef_size_t_h::size_t,
        )
    };
}
#[export_name = "zcfree"]

pub unsafe extern "C" fn zcfree_ffi(
    mut _opaque: crate::stdlib::voidpf,
    mut ptr: crate::stdlib::voidpf,
) {
    crate::stdlib::free(ptr as *mut ::core::ffi::c_void);
}
