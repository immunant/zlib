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

const NEED_DICT: [::core::ffi::c_char; 16] = [
    b'n' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'd' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'd' as ::core::ffi::c_char,
    b'i' as ::core::ffi::c_char,
    b'c' as ::core::ffi::c_char,
    b't' as ::core::ffi::c_char,
    b'i' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'n' as ::core::ffi::c_char,
    b'a' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'y' as ::core::ffi::c_char,
    0,
];
const STREAM_END: [::core::ffi::c_char; 11] = [
    b's' as ::core::ffi::c_char,
    b't' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'a' as ::core::ffi::c_char,
    b'm' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'n' as ::core::ffi::c_char,
    b'd' as ::core::ffi::c_char,
    0,
];
const EMPTY: [::core::ffi::c_char; 1] = [0];
const FILE_ERROR: [::core::ffi::c_char; 11] = [
    b'f' as ::core::ffi::c_char,
    b'i' as ::core::ffi::c_char,
    b'l' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    0,
];
const STREAM_ERROR: [::core::ffi::c_char; 13] = [
    b's' as ::core::ffi::c_char,
    b't' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'a' as ::core::ffi::c_char,
    b'm' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    0,
];
const DATA_ERROR: [::core::ffi::c_char; 11] = [
    b'd' as ::core::ffi::c_char,
    b'a' as ::core::ffi::c_char,
    b't' as ::core::ffi::c_char,
    b'a' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    0,
];
const MEM_ERROR: [::core::ffi::c_char; 20] = [
    b'i' as ::core::ffi::c_char,
    b'n' as ::core::ffi::c_char,
    b's' as ::core::ffi::c_char,
    b'u' as ::core::ffi::c_char,
    b'f' as ::core::ffi::c_char,
    b'f' as ::core::ffi::c_char,
    b'i' as ::core::ffi::c_char,
    b'c' as ::core::ffi::c_char,
    b'i' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'n' as ::core::ffi::c_char,
    b't' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'm' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'm' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'y' as ::core::ffi::c_char,
    0,
];
const BUF_ERROR: [::core::ffi::c_char; 13] = [
    b'b' as ::core::ffi::c_char,
    b'u' as ::core::ffi::c_char,
    b'f' as ::core::ffi::c_char,
    b'f' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    0,
];
const VERSION_ERROR: [::core::ffi::c_char; 21] = [
    b'i' as ::core::ffi::c_char,
    b'n' as ::core::ffi::c_char,
    b'c' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'm' as ::core::ffi::c_char,
    b'p' as ::core::ffi::c_char,
    b'a' as ::core::ffi::c_char,
    b't' as ::core::ffi::c_char,
    b'i' as ::core::ffi::c_char,
    b'b' as ::core::ffi::c_char,
    b'l' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'v' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b's' as ::core::ffi::c_char,
    b'i' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'n' as ::core::ffi::c_char,
    0,
];

#[no_mangle]
pub static z_errmsg: [&::core::ffi::c_char; 10] = [
    &NEED_DICT[0],
    &STREAM_END[0],
    &EMPTY[0],
    &FILE_ERROR[0],
    &STREAM_ERROR[0],
    &DATA_ERROR[0],
    &MEM_ERROR[0],
    &BUF_ERROR[0],
    &VERSION_ERROR[0],
    &EMPTY[0],
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
    z_errmsg[z_error_index(err)]
}
#[export_name = "zcalloc"]
pub unsafe extern "C" fn zcalloc_ffi(
    _opaque: crate::stdlib::voidpf,
    items: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    let items = items as crate::__stddef_size_t_h::size_t;
    let size = size as crate::__stddef_size_t_h::size_t;
    if items.checked_mul(size).is_none() {
        return ::core::ptr::null_mut();
    }
    if ::core::mem::size_of::<crate::stdlib::uInt>() > 2 {
        crate::stdlib::malloc(items * size)
    } else {
        crate::stdlib::calloc(items, size)
    }
}
#[export_name = "zcfree"]
pub unsafe extern "C" fn zcfree_ffi(_opaque: crate::stdlib::voidpf, ptr: crate::stdlib::voidpf) {
    crate::stdlib::free(ptr as *mut ::core::ffi::c_void);
}
