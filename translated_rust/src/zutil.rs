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

fn size_class(bytes: usize) -> crate::stdlib::uLong {
    match bytes {
        2 => 0,
        4 => 1,
        8 => 2,
        _ => 3,
    }
}

fn size_flag<T>(shift: u32) -> crate::stdlib::uLong {
    size_class(::core::mem::size_of::<T>()) << shift
}

fn compile_flags_for_sizes(
    uint_size: usize,
    ulong_size: usize,
    pointer_size: usize,
    off_t_size: usize,
) -> crate::stdlib::uLong {
    (size_class(uint_size) << 0)
        .wrapping_add(size_class(ulong_size) << 2)
        .wrapping_add(size_class(pointer_size) << 4)
        .wrapping_add(size_class(off_t_size) << 6)
}

fn zlib_compile_flags() -> crate::stdlib::uLong {
    compile_flags_for_sizes(
        ::core::mem::size_of::<crate::stdlib::uInt>(),
        ::core::mem::size_of::<crate::stdlib::uLong>(),
        ::core::mem::size_of::<usize>(),
        ::core::mem::size_of::<crate::stdlib::off_t>(),
    )
}

#[export_name = "zlibCompileFlags"]
pub unsafe extern "C" fn zlibCompileFlags_ffi() -> crate::stdlib::uLong {
    zlib_compile_flags()
}

fn has_error_message_index(err: ::core::ffi::c_int) -> bool {
    (-6..=2).contains(&err)
}

fn error_message_index(err: ::core::ffi::c_int) -> usize {
    if !has_error_message_index(err) {
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

fn allocation_byte_count(items: ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> size_t {
    items.wrapping_mul(size) as size_t
}

fn allocation_request(items: ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> AllocationRequest {
    if ::core::mem::size_of::<crate::stdlib::uInt>() > 2 {
        AllocationRequest::Malloc(allocation_byte_count(items, size))
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
    use super::{
        allocation_byte_count, allocation_request, compile_flags_for_sizes, error_message_index,
        has_error_message_index, size_class, size_flag, size_t, zlib_compile_flags, zlib_version,
        AllocationRequest,
    };

    #[test]
    fn error_message_index_range_matches_zlib_error_codes() {
        assert!(has_error_message_index(-6));
        assert!(has_error_message_index(2));
        assert!(!has_error_message_index(-7));
        assert!(!has_error_message_index(3));
    }

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
    fn size_class_normalizes_supported_and_other_sizes() {
        assert_eq!(size_class(2), 0);
        assert_eq!(size_class(4), 1);
        assert_eq!(size_class(8), 2);
        assert_eq!(size_class(1), 3);
        assert_eq!(size_class(16), 3);
    }

    #[test]
    fn size_flag_places_the_size_class_at_the_requested_offset() {
        assert_eq!(size_flag::<u16>(0), 0);
        assert_eq!(size_flag::<u32>(2), 4);
        assert_eq!(size_flag::<u64>(4), 32);
    }

    #[test]
    fn compile_flags_pack_each_size_class_into_its_zlib_field() {
        assert_eq!(compile_flags_for_sizes(2, 4, 8, 1), 0 | 4 | 32 | 192);
        assert_eq!(compile_flags_for_sizes(1, 16, 1, 16), 3 | 12 | 48 | 192);
    }

    #[test]
    fn compile_flags_use_the_platform_type_sizes() {
        assert_eq!(
            zlib_compile_flags(),
            compile_flags_for_sizes(
                ::core::mem::size_of::<crate::stdlib::uInt>(),
                ::core::mem::size_of::<crate::stdlib::uLong>(),
                ::core::mem::size_of::<usize>(),
                ::core::mem::size_of::<crate::stdlib::off_t>(),
            )
        );
    }

    #[test]
    fn allocation_byte_count_preserves_c_uint_wrapping() {
        assert_eq!(
            allocation_byte_count(::core::ffi::c_uint::MAX, 2),
            ::core::ffi::c_uint::MAX.wrapping_mul(2) as size_t
        );
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
