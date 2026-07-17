pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;

pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;

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

const ERROR_MESSAGE_COUNT: usize = 10;
const FALLBACK_ERROR_MESSAGE_INDEX: usize = ERROR_MESSAGE_COUNT - 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ErrorMessageKind {
    NeedDictionary,
    StreamEnd,
    Empty,
    FileError,
    StreamError,
    DataError,
    InsufficientMemory,
    BufferError,
    IncompatibleVersion,
    Fallback,
}

impl ErrorMessageKind {
    fn for_error(err: ::core::ffi::c_int) -> Self {
        match err {
            2 => Self::NeedDictionary,
            1 => Self::StreamEnd,
            0 => Self::Empty,
            -1 => Self::FileError,
            -2 => Self::StreamError,
            -3 => Self::DataError,
            -4 => Self::InsufficientMemory,
            -5 => Self::BufferError,
            -6 => Self::IncompatibleVersion,
            _ => Self::Fallback,
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::NeedDictionary => 0,
            Self::StreamEnd => 1,
            Self::Empty => 2,
            Self::FileError => 3,
            Self::StreamError => 4,
            Self::DataError => 5,
            Self::InsufficientMemory => 6,
            Self::BufferError => 7,
            Self::IncompatibleVersion => 8,
            Self::Fallback => FALLBACK_ERROR_MESSAGE_INDEX,
        }
    }
}

static ERROR_MESSAGES: [&[::core::ffi::c_char]; ERROR_MESSAGE_COUNT] = [
    &NEED_DICTIONARY,
    &STREAM_END,
    &EMPTY_ERROR,
    &FILE_ERROR,
    &STREAM_ERROR,
    &DATA_ERROR,
    &INSUFFICIENT_MEMORY,
    &BUFFER_ERROR,
    &INCOMPATIBLE_VERSION,
    &EMPTY_ERROR,
];

#[no_mangle]
pub static z_errmsg: [&::core::ffi::c_char; ERROR_MESSAGE_COUNT] = [
    &NEED_DICTIONARY[0],
    &STREAM_END[0],
    &EMPTY_ERROR[0],
    &FILE_ERROR[0],
    &STREAM_ERROR[0],
    &DATA_ERROR[0],
    &INSUFFICIENT_MEMORY[0],
    &BUFFER_ERROR[0],
    &INCOMPATIBLE_VERSION[0],
    &EMPTY_ERROR[0],
];

fn zlib_version() -> &'static [::core::ffi::c_char; 15] {
    &crate::zlib_h::ZLIB_VERSION
}

#[export_name = "zlibVersion"]
pub extern "C" fn zlibVersion_ffi() -> *const ::core::ffi::c_char {
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
    size_class(uint_size)
        | (size_class(ulong_size) << 2)
        | (size_class(pointer_size) << 4)
        | (size_class(off_t_size) << 6)
}

fn zlib_compile_flags() -> crate::stdlib::uLong {
    compile_flags_for_sizes(
        ::core::mem::size_of::<crate::stdlib::uInt>(),
        ::core::mem::size_of::<crate::stdlib::uLong>(),
        ::core::mem::size_of::<crate::stdlib::voidpf>(),
        ::core::mem::size_of::<crate::stdlib::off_t>(),
    )
}

#[export_name = "zlibCompileFlags"]
pub extern "C" fn zlibCompileFlags_ffi() -> crate::stdlib::uLong {
    zlib_compile_flags()
}

fn has_error_message_index(err: ::core::ffi::c_int) -> bool {
    ErrorMessageKind::for_error(err) != ErrorMessageKind::Fallback
}

fn error_message_index(err: ::core::ffi::c_int) -> usize {
    ErrorMessageKind::for_error(err).index()
}

fn error_message(err: ::core::ffi::c_int) -> &'static [::core::ffi::c_char] {
    ERROR_MESSAGES[error_message_index(err)]
}

pub(crate) fn z_error_message(err: ::core::ffi::c_int) -> &'static [::core::ffi::c_char] {
    error_message(err)
}

pub(crate) fn z_errmsg_index(err: ::core::ffi::c_int) -> usize {
    error_message_index(err)
}

#[export_name = "zError"]
pub extern "C" fn zError_ffi(err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    error_message(err).as_ptr()
}

enum AllocationRequest {
    Malloc(size_t),
    Calloc { items: size_t, size: size_t },
}

fn allocation_byte_count(items: ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> Option<size_t> {
    items.checked_mul(size).map(|bytes| bytes as size_t)
}

fn allocation_uses_malloc(uint_size: usize) -> bool {
    uint_size > 2
}

fn allocation_request_for_uint_size(
    uint_size: usize,
    items: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> Option<AllocationRequest> {
    let bytes = allocation_byte_count(items, size)?;
    if allocation_uses_malloc(uint_size) {
        Some(AllocationRequest::Malloc(bytes))
    } else {
        Some(AllocationRequest::Calloc {
            items: items as size_t,
            size: size as size_t,
        })
    }
}

fn allocation_request(
    items: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> Option<AllocationRequest> {
    allocation_request_for_uint_size(::core::mem::size_of::<crate::stdlib::uInt>(), items, size)
}

#[export_name = "zcalloc"]
pub unsafe extern "C" fn zcalloc_ffi(
    _opaque: crate::stdlib::voidpf,
    items: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    match allocation_request(items, size) {
        Some(AllocationRequest::Malloc(bytes)) => crate::stdlib::malloc(bytes),
        Some(AllocationRequest::Calloc { items, size }) => crate::stdlib::calloc(items, size),
        None => ::core::ptr::null_mut(),
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
        has_error_message_index, size_class, size_flag, z_errmsg, z_errmsg_index,
        zlib_compile_flags, zlib_version, AllocationRequest, ErrorMessageKind, EMPTY_ERROR,
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
    fn error_message_kind_classifies_each_zlib_error_code() {
        assert_eq!(
            ErrorMessageKind::for_error(2),
            ErrorMessageKind::NeedDictionary
        );
        assert_eq!(ErrorMessageKind::for_error(1), ErrorMessageKind::StreamEnd);
        assert_eq!(ErrorMessageKind::for_error(0), ErrorMessageKind::Empty);
        assert_eq!(ErrorMessageKind::for_error(-1), ErrorMessageKind::FileError);
        assert_eq!(
            ErrorMessageKind::for_error(-2),
            ErrorMessageKind::StreamError
        );
        assert_eq!(ErrorMessageKind::for_error(-3), ErrorMessageKind::DataError);
        assert_eq!(
            ErrorMessageKind::for_error(-4),
            ErrorMessageKind::InsufficientMemory
        );
        assert_eq!(
            ErrorMessageKind::for_error(-5),
            ErrorMessageKind::BufferError
        );
        assert_eq!(
            ErrorMessageKind::for_error(-6),
            ErrorMessageKind::IncompatibleVersion
        );
    }

    #[test]
    fn error_message_kind_uses_the_fallback_outside_zlib_error_codes() {
        assert_eq!(ErrorMessageKind::for_error(-7), ErrorMessageKind::Fallback);
        assert_eq!(ErrorMessageKind::for_error(3), ErrorMessageKind::Fallback);
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
                z_errmsg[z_errmsg_index(error)] as *const ::core::ffi::c_char,
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
                ::core::mem::size_of::<crate::stdlib::voidpf>(),
                ::core::mem::size_of::<crate::stdlib::off_t>(),
            )
        );
    }

    #[test]
    fn allocation_byte_count_rejects_c_uint_overflow() {
        assert_eq!(allocation_byte_count(::core::ffi::c_uint::MAX, 2), None);
    }

    #[test]
    fn allocation_byte_count_preserves_representable_request_sizes() {
        assert_eq!(allocation_byte_count(3, 4), Some(12));
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
            Some(AllocationRequest::Malloc(bytes)) => assert_eq!(bytes, 12),
            Some(AllocationRequest::Calloc { items, size }) => {
                assert_eq!(items, 3);
                assert_eq!(size, 4);
            }
            None => panic!("representable allocation request must not overflow"),
        }
    }

    #[test]
    fn allocation_request_uses_calloc_for_two_byte_uints() {
        match allocation_request_for_uint_size(2, 3, 4) {
            Some(AllocationRequest::Malloc(_)) => panic!("two-byte uInt must use calloc"),
            Some(AllocationRequest::Calloc { items, size }) => {
                assert_eq!(items, 3);
                assert_eq!(size, 4);
            }
            None => panic!("representable allocation request must not overflow"),
        }
    }

    #[test]
    fn allocation_request_rejects_overflow_before_selecting_an_allocator() {
        assert!(allocation_request(::core::ffi::c_uint::MAX, 2).is_none());
        assert!(allocation_request_for_uint_size(2, ::core::ffi::c_uint::MAX, 2).is_none());
    }
}
