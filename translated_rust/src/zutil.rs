pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;



pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;

fn zlib_version() -> &'static [::core::ffi::c_char; 15] {
    &crate::zlib_h::ZLIB_VERSION
}

fn zlib_compile_flags() -> crate::stdlib::uLong {
    let mut flags = 0;

    flags |= match ::core::mem::size_of::<crate::stdlib::uInt>() {
        2 => 0,
        4 => 1,
        8 => 2,
        _ => 3,
    };
    flags |= match ::core::mem::size_of::<crate::stdlib::uLong>() {
        2 => 0,
        4 => 1 << 2,
        8 => 2 << 2,
        _ => 3 << 2,
    };
    flags |= match ::core::mem::size_of::<crate::stdlib::voidpf>() {
        2 => 0,
        4 => 1 << 4,
        8 => 2 << 4,
        _ => 3 << 4,
    };
    flags |= match ::core::mem::size_of::<crate::stdlib::off_t>() {
        2 => 0,
        4 => 1 << 6,
        8 => 2 << 6,
        _ => 3 << 6,
    };

    flags
}

fn z_error_index(err: ::core::ffi::c_int) -> usize {
    match err {
        2 => 0,
        1 => 1,
        0 => 2,
        -1 => 3,
        -2 => 4,
        -3 => 5,
        -4 => 6,
        -5 => 7,
        -6 => 8,
        _ => 9,
    }
}

#[no_mangle]

pub static mut z_errmsg: [*mut ::core::ffi::c_char; 10] = [
    b"need dictionary\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"stream end\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"file error\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"stream error\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"data error\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"insufficient memory\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"buffer error\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"incompatible version\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
#[export_name = "zlibVersion"]
pub unsafe extern "C" fn zlibVersion_ffi() -> *const ::core::ffi::c_char {
    zlib_version().as_ptr()
}
#[export_name = "zlibCompileFlags"]
pub unsafe extern "C" fn zlibCompileFlags_ffi() -> crate::stdlib::uLong {
    zlib_compile_flags()
}
#[export_name = "zError"]
pub unsafe extern "C" fn zError_ffi(err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    z_errmsg[z_error_index(err)].cast_const()
}
pub unsafe extern "C" fn zcalloc(
    mut _opaque: crate::stdlib::voidpf,
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
    zcalloc(opaque, items, size)
}
pub unsafe extern "C" fn zcfree(mut _opaque: crate::stdlib::voidpf, mut ptr: crate::stdlib::voidpf) {
    crate::stdlib::free(ptr as *mut ::core::ffi::c_void);
}
#[export_name = "zcfree"]

pub unsafe extern "C" fn zcfree_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut ptr: crate::stdlib::voidpf,
) {
    zcfree(opaque, ptr)
}
