pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;
#[no_mangle]

pub static z_errmsg: [::core::sync::atomic::AtomicPtr<::core::ffi::c_char>; 10] = [
    ::core::sync::atomic::AtomicPtr::new(b"need dictionary\0".as_ptr() as *mut _),
    ::core::sync::atomic::AtomicPtr::new(b"stream end\0".as_ptr() as *mut _),
    ::core::sync::atomic::AtomicPtr::new(b"\0".as_ptr() as *mut _),
    ::core::sync::atomic::AtomicPtr::new(b"file error\0".as_ptr() as *mut _),
    ::core::sync::atomic::AtomicPtr::new(b"stream error\0".as_ptr() as *mut _),
    ::core::sync::atomic::AtomicPtr::new(b"data error\0".as_ptr() as *mut _),
    ::core::sync::atomic::AtomicPtr::new(b"insufficient memory\0".as_ptr() as *mut _),
    ::core::sync::atomic::AtomicPtr::new(b"buffer error\0".as_ptr() as *mut _),
    ::core::sync::atomic::AtomicPtr::new(b"incompatible version\0".as_ptr() as *mut _),
    ::core::sync::atomic::AtomicPtr::new(b"\0".as_ptr() as *mut _),
];
pub extern "C" fn zlibVersion() -> *const ::core::ffi::c_char {
    return crate::zlib_h::ZLIB_VERSION.as_ptr();
}
#[export_name = "zlibVersion"]

pub extern "C" fn zlibVersion_ffi() -> *const ::core::ffi::c_char {
    zlibVersion()
}

// zlib encodes the size of each ABI type in two bits. Keeping that encoding
// in a value-only helper makes the exported flag calculation independent of
// the repetitive C-style switch structure.
fn zlib_size_flag(size: usize, shift: u32) -> crate::stdlib::uLong {
    let encoded = match size {
        2 => 0,
        4 => 1,
        8 => 2,
        _ => 3,
    };
    (encoded << shift) as crate::stdlib::uLong
}

pub extern "C" fn zlibCompileFlags() -> crate::stdlib::uLong {
    zlib_size_flag(::core::mem::size_of::<crate::stdlib::uInt>(), 0)
        | zlib_size_flag(::core::mem::size_of::<crate::stdlib::uLong>(), 2)
        | zlib_size_flag(::core::mem::size_of::<crate::stdlib::voidpf>(), 4)
        | zlib_size_flag(::core::mem::size_of::<crate::stdlib::off_t>(), 6)
}
#[export_name = "zlibCompileFlags"]

pub extern "C" fn zlibCompileFlags_ffi() -> crate::stdlib::uLong {
    zlibCompileFlags()
}
pub extern "C" fn zError(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    return z_errmsg[(if err < -6 as ::core::ffi::c_int || err > 2 as ::core::ffi::c_int {
        9 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int - err
    }) as usize]
        .load(::core::sync::atomic::Ordering::Relaxed);
}
#[export_name = "zError"]

pub extern "C" fn zError_ffi(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    zError(err)
}
// `opaque` is deliberately ignored and every `uInt` size is valid for the
// allocator, so Rust callers need no safety precondition to select zlib's
// default allocation callback. The exported ABI wrapper below remains unsafe
// for C callers.
pub extern "C" fn zcalloc(
    _opaque: crate::stdlib::voidpf,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    if ::core::mem::size_of::<crate::stdlib::uInt>() > 2 as usize {
        crate::stdlib::malloc(items.wrapping_mul(size) as crate::__stddef_size_t_h::size_t)
    } else {
        crate::stdlib::calloc(
            items as crate::__stddef_size_t_h::size_t,
            size as crate::__stddef_size_t_h::size_t,
        )
    }
}
#[export_name = "zcalloc"]

pub unsafe extern "C" fn zcalloc_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    zcalloc(opaque, items, size)
}
// The exported wrapper owns the foreign-call boundary. This implementation
// adapter only forwards the allocation pointer whose ownership its callers
// already validated to C's matching deallocator.
pub extern "C" fn zcfree(_opaque: crate::stdlib::voidpf, mut ptr: crate::stdlib::voidpf) {
    crate::stdlib::free(ptr as *mut ::core::ffi::c_void);
}
#[export_name = "zcfree"]

pub unsafe extern "C" fn zcfree_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut ptr: crate::stdlib::voidpf,
) {
    zcfree(opaque, ptr)
}

// Initialization entry points share zlib's allocator-defaulting rule: only
// a stream that arrived with no allocator gets the default allocator and a
// reset opaque value.  A supplied deallocator must be retained, including
// when the allocator is supplied separately.  Keeping that decision here
// prevents the three stream initializers from drifting on either detail.
pub(crate) fn prepare_stream_allocator(stream: &mut crate::zlib_h::z_stream) -> bool {
    let uses_default_allocator = stream.zalloc.is_none();
    if uses_default_allocator {
        stream.zalloc = Some(
            zcalloc
                as extern "C" fn(
                    crate::stdlib::voidpf,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> crate::stdlib::voidpf,
        ) as crate::zlib_h::alloc_func;
        stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if stream.zfree.is_none() {
        stream.zfree = Some(
            zcfree as extern "C" fn(crate::stdlib::voidpf, crate::stdlib::voidpf) -> (),
        ) as crate::zlib_h::free_func;
    }
    uses_default_allocator
}
