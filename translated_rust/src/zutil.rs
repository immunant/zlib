pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;
use core::sync::atomic::AtomicPtr;
use std::mem::MaybeUninit;
use std::sync::Mutex;

// `zcalloc` is installed as the default C callback, so allocations escape to
// the translated codec as raw storage.  Retain their owners here until the
// matching `zcfree` callback removes them.  A u128 slot provides the
// max-alignment needed by the supported zlib state records while keeping the
// storage uninitialized, matching this target's malloc branch.
static ZCALLOC_ALLOCATIONS: Mutex<Vec<(usize, Box<[MaybeUninit<u128>]>)>> = Mutex::new(Vec::new());
static Z_ERROR_MESSAGES: [&[u8]; 10] = [
    b"need dictionary\0",
    b"stream end\0",
    b"\0",
    b"file error\0",
    b"stream error\0",
    b"data error\0",
    b"insufficient memory\0",
    b"buffer error\0",
    b"incompatible version\0",
    b"\0",
];

#[no_mangle]
pub static z_errmsg: [AtomicPtr<::core::ffi::c_char>; 10] = [
    AtomicPtr::new(Z_ERROR_MESSAGES[0].as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(Z_ERROR_MESSAGES[1].as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(Z_ERROR_MESSAGES[2].as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(Z_ERROR_MESSAGES[3].as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(Z_ERROR_MESSAGES[4].as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(Z_ERROR_MESSAGES[5].as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(Z_ERROR_MESSAGES[6].as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(Z_ERROR_MESSAGES[7].as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(Z_ERROR_MESSAGES[8].as_ptr() as *mut ::core::ffi::c_char),
    AtomicPtr::new(Z_ERROR_MESSAGES[9].as_ptr() as *mut ::core::ffi::c_char),
];
pub fn zlibVersion() -> &'static [::core::ffi::c_char; 15] {
    &crate::zlib_h::ZLIB_VERSION
}
#[export_name = "zlibVersion"]

pub unsafe extern "C" fn zlibVersion_ffi() -> *const ::core::ffi::c_char {
    zlibVersion().as_ptr()
}
fn zlib_compile_flags() -> crate::stdlib::uLong {
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
    zlib_compile_flags()
}
pub fn zError(mut err: ::core::ffi::c_int) -> &'static [u8] {
    &Z_ERROR_MESSAGES[(if err < -6 as ::core::ffi::c_int || err > 2 as ::core::ffi::c_int {
        9 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int - err
    }) as usize]
}
#[export_name = "zError"]

pub unsafe extern "C" fn zError_ffi(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    zError(err).as_ptr().cast()
}
// The allocator broker keeps ownership bookkeeping pointer-free.  The FFI
// callback below is the only place an allocation is published as a C pointer.
fn zcalloc_allocate(
    items: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> Option<Box<[MaybeUninit<u128>]>> {
    // `uInt` is the ABI's fixed-width `u32`, so this is the translated C
    // branch selected on every supported target.  Keep its wrapping product
    // and allocation semantics, without retaining the unreachable calloc
    // branch as an extra foreign call in the implementation.
    let bytes = items.wrapping_mul(size) as usize;
    let words = (bytes / ::core::mem::size_of::<u128>())
        + usize::from(bytes % ::core::mem::size_of::<u128>() != 0);
    // Like malloc(0), return a freeable allocation for a zero-size request.
    let words = words.max(1);
    let mut allocation = Vec::<MaybeUninit<u128>>::new();
    if allocation.try_reserve_exact(words).is_err() {
        return None;
    }
    allocation.resize_with(words, MaybeUninit::uninit);
    Some(allocation.into_boxed_slice())
}

fn zcalloc_store(address: usize, allocation: Box<[MaybeUninit<u128>]>) -> bool {
    let mut allocations = match ZCALLOC_ALLOCATIONS.lock() {
        Ok(allocations) => allocations,
        Err(poisoned) => poisoned.into_inner(),
    };
    if allocations.try_reserve(1).is_err() {
        return false;
    }
    allocations.push((address, allocation));
    true
}

pub unsafe extern "C" fn zcalloc(
    _opaque: crate::stdlib::voidpf,
    items: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    let mut allocation = match zcalloc_allocate(items, size) {
        Some(allocation) => allocation,
        None => return ::core::ptr::null_mut(),
    };
    let pointer = allocation.as_mut_ptr().cast::<::core::ffi::c_void>();
    if zcalloc_store(pointer as usize, allocation) {
        pointer
    } else {
        ::core::ptr::null_mut()
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

// Returns whether this address was not owned by the default allocation
// broker.  The wrapper performs the corresponding C-compatible free.
fn zcfree_is_external(address: usize) -> bool {
    let allocation = {
        let mut allocations = match ZCALLOC_ALLOCATIONS.lock() {
            Ok(allocations) => allocations,
            Err(poisoned) => poisoned.into_inner(),
        };
        allocations
            .iter()
            .position(|(stored_address, _)| *stored_address == address)
            .map(|index| allocations.swap_remove(index).1)
    };
    allocation.is_none()
}

pub unsafe extern "C" fn zcfree(_opaque: crate::stdlib::voidpf, ptr: crate::stdlib::voidpf) {
    if !ptr.is_null() && zcfree_is_external(ptr as usize) {
        // Preserve zcfree's public free-compatible behavior for allocations
        // supplied by an external caller rather than this default broker.
        crate::stdlib::free(ptr);
    }
}
#[export_name = "zcfree"]

pub unsafe extern "C" fn zcfree_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut ptr: crate::stdlib::voidpf,
) {
    zcfree(opaque, ptr)
}
