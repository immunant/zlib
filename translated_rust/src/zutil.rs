pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;

pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;

use ::core::sync::atomic::AtomicPtr;

const fn c_chars<const N: usize>(bytes: [u8; N]) -> [::core::ffi::c_char; N] {
    let mut chars = [0; N];
    let mut index = 0;
    while index < N {
        chars[index] = bytes[index] as ::core::ffi::c_char;
        index += 1;
    }
    chars
}

static NEED_DICTIONARY: [::core::ffi::c_char; 16] = c_chars(*b"need dictionary\0");
static STREAM_END: [::core::ffi::c_char; 11] = c_chars(*b"stream end\0");
static EMPTY_ERROR: [::core::ffi::c_char; 1] = c_chars(*b"\0");
static FILE_ERROR: [::core::ffi::c_char; 11] = c_chars(*b"file error\0");
static STREAM_ERROR: [::core::ffi::c_char; 13] = c_chars(*b"stream error\0");
static DATA_ERROR: [::core::ffi::c_char; 11] = c_chars(*b"data error\0");
static INSUFFICIENT_MEMORY: [::core::ffi::c_char; 20] = c_chars(*b"insufficient memory\0");
static BUFFER_ERROR: [::core::ffi::c_char; 13] = c_chars(*b"buffer error\0");
static INCOMPATIBLE_VERSION: [::core::ffi::c_char; 21] = c_chars(*b"incompatible version\0");

#[no_mangle]
pub static z_errmsg: [AtomicPtr<::core::ffi::c_char>; 10] = [
    AtomicPtr::new(NEED_DICTIONARY.as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(STREAM_END.as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(EMPTY_ERROR.as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(FILE_ERROR.as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(STREAM_ERROR.as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(DATA_ERROR.as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(INSUFFICIENT_MEMORY.as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(BUFFER_ERROR.as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(INCOMPATIBLE_VERSION.as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(EMPTY_ERROR.as_ptr() as *mut ::core::ffi::c_char),
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

fn error_message(err: ::core::ffi::c_int) -> &'static [::core::ffi::c_char] {
    match error_message_index(err) {
        0 => &NEED_DICTIONARY,
        1 => &STREAM_END,
        2 => &EMPTY_ERROR,
        3 => &FILE_ERROR,
        4 => &STREAM_ERROR,
        5 => &DATA_ERROR,
        6 => &INSUFFICIENT_MEMORY,
        7 => &BUFFER_ERROR,
        8 => &INCOMPATIBLE_VERSION,
        _ => &EMPTY_ERROR,
    }
}

pub(crate) fn z_errmsg_index(err: ::core::ffi::c_int) -> usize {
    error_message_index(err)
}

#[export_name = "zError"]
pub unsafe extern "C" fn zError_ffi(err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    error_message(err).as_ptr()
}

enum AllocationRequest {
    Malloc(size_t),
    Calloc { items: size_t, size: size_t },
}

fn allocation_byte_count(items: ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> size_t {
    items.wrapping_mul(size) as size_t
}

fn allocation_uses_malloc(uint_size: usize) -> bool {
    uint_size > 2
}

fn allocation_request_for_uint_size(
    uint_size: usize,
    items: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> AllocationRequest {
    if allocation_uses_malloc(uint_size) {
        AllocationRequest::Malloc(allocation_byte_count(items, size))
    } else {
        AllocationRequest::Calloc {
            items: items as size_t,
            size: size as size_t,
        }
    }
}

fn allocation_request(items: ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> AllocationRequest {
    allocation_request_for_uint_size(::core::mem::size_of::<crate::stdlib::uInt>(), items, size)
}

#[export_name = "zcalloc"]
pub unsafe extern "C" fn zcalloc_ffi(
    _opaque: crate::stdlib::voidpf,
    items: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    match allocation_request(items, size) {
        AllocationRequest::Malloc(bytes) => crate::stdlib::malloc(bytes),
        AllocationRequest::Calloc { items, size } => crate::stdlib::calloc(items, size),
    }
}

#[export_name = "zcfree"]
pub unsafe extern "C" fn zcfree_ffi(_opaque: crate::stdlib::voidpf, ptr: crate::stdlib::voidpf) {
    crate::stdlib::free(ptr);
}

#[cfg(test)]
mod tests {
    use super::{
        allocation_byte_count, allocation_request, allocation_request_for_uint_size,
        allocation_uses_malloc, compile_flags_for_sizes, error_message, error_message_index,
        has_error_message_index, size_class, size_flag, size_t, z_errmsg, z_errmsg_index,
        zlib_compile_flags, zlib_version, AllocationRequest, EMPTY_ERROR,
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
    fn error_messages_are_nul_terminated_and_use_canonical_fallback() {
        assert_eq!(error_message(2).last(), Some(&0));
        assert_eq!(error_message(-6).last(), Some(&0));
        assert_eq!(error_message(-7).as_ptr(), EMPTY_ERROR.as_ptr());
    }

    #[test]
    fn error_message_table_preserves_pointer_layout_and_indexing() {
        assert_eq!(
            ::core::mem::size_of_val(&z_errmsg),
            10 * ::core::mem::size_of::<*mut ::core::ffi::c_char>()
        );
        assert_eq!(
            ::core::mem::align_of_val(&z_errmsg),
            ::core::mem::align_of::<*mut ::core::ffi::c_char>()
        );

        for error in [-7, -6, -1, 0, 1, 2, 3] {
            assert_eq!(
                z_errmsg[z_errmsg_index(error)].load(::core::sync::atomic::Ordering::Relaxed)
                    as *const ::core::ffi::c_char,
                error_message(error).as_ptr()
            );
        }
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
    fn allocation_uses_malloc_only_for_uint_sizes_above_two_bytes() {
        assert!(!allocation_uses_malloc(1));
        assert!(!allocation_uses_malloc(2));
        assert!(allocation_uses_malloc(3));
        assert!(allocation_uses_malloc(4));
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

    #[test]
    fn allocation_request_uses_calloc_for_two_byte_uints() {
        match allocation_request_for_uint_size(2, 3, 4) {
            AllocationRequest::Malloc(_) => panic!("two-byte uInt must use calloc"),
            AllocationRequest::Calloc { items, size } => {
                assert_eq!(items, 3);
                assert_eq!(size, 4);
            }
        }
    }
}
