pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;

pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;

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

fn zlib_version() -> &'static [::core::ffi::c_char; 15] {
    &crate::zlib_h::ZLIB_VERSION
}

#[export_name = "zlibVersion"]
pub unsafe extern "C" fn zlibVersion_ffi() -> *const ::core::ffi::c_char {
    zlib_version().as_ptr()
}

fn size_flag<T>(shift: u32) -> crate::stdlib::uLong {
    let flag = match ::core::mem::size_of::<T>() {
        2 => 0,
        4 => 1,
        8 => 2,
        _ => 3,
    };

    (flag as crate::stdlib::uLong) << shift
}

fn zlib_compile_flags() -> crate::stdlib::uLong {
    size_flag::<crate::stdlib::uInt>(0)
        .wrapping_add(size_flag::<crate::stdlib::uLong>(2))
        .wrapping_add(size_flag::<usize>(4))
        .wrapping_add(size_flag::<crate::stdlib::off_t>(6))
}

#[export_name = "zlibCompileFlags"]
pub unsafe extern "C" fn zlibCompileFlags_ffi() -> crate::stdlib::uLong {
    zlib_compile_flags()
}

fn error_message_index(err: ::core::ffi::c_int) -> usize {
    if !(-6..=2).contains(&err) {
        9
    } else {
        (2 - err) as usize
    }
}

#[export_name = "zError"]
pub unsafe extern "C" fn zError_ffi(err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    z_errmsg[error_message_index(err)] as *const ::core::ffi::c_char
}

enum AllocationRequest {
    Malloc(size_t),
    Calloc { items: size_t, size: size_t },
}

fn allocation_request(items: ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> AllocationRequest {
    if ::core::mem::size_of::<crate::stdlib::uInt>() > 2 {
        AllocationRequest::Malloc(items.wrapping_mul(size) as size_t)
    } else {
        AllocationRequest::Calloc {
            items: items as size_t,
            size: size as size_t,
        }
    }
}

#[export_name = "zcalloc"]
pub unsafe extern "C" fn zcalloc_ffi(
    opaque: crate::stdlib::voidpf,
    items: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    let _ = opaque;
    match allocation_request(items, size) {
        AllocationRequest::Malloc(bytes) => crate::stdlib::malloc(bytes),
        AllocationRequest::Calloc { items, size } => crate::stdlib::calloc(items, size),
    }
}

#[export_name = "zcfree"]
pub unsafe extern "C" fn zcfree_ffi(opaque: crate::stdlib::voidpf, ptr: crate::stdlib::voidpf) {
    let _ = opaque;
    crate::stdlib::free(ptr);
}

#[cfg(test)]
mod tests {
    use super::{allocation_request, error_message_index, zlib_version, AllocationRequest};

    #[test]
    fn error_messages_use_zlib_error_indexing() {
        assert_eq!(error_message_index(2), 0);
        assert_eq!(error_message_index(0), 2);
        assert_eq!(error_message_index(-6), 8);
        assert_eq!(error_message_index(-7), 9);
        assert_eq!(error_message_index(3), 9);
    }

    #[test]
    fn version_is_nul_terminated() {
        assert_eq!(zlib_version().last(), Some(&0));
    }

    #[test]
    fn allocation_request_preserves_the_platform_choice() {
        match allocation_request(3, 4) {
            AllocationRequest::Malloc(bytes) => assert_eq!(bytes, 12),
            AllocationRequest::Calloc { items, size } => {
                assert_eq!(items, 3);
                assert_eq!(size, 4);
            }
        }
    }
}
