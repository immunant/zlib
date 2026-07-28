pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::zlib_h::ZLIB_VERSION;
use core::sync::atomic::{AtomicPtr, Ordering};
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::{Mutex, OnceLock};

#[repr(align(16))]
#[derive(Clone)]
struct ZAllocationByte([u8; 16]);

enum ZAllocation {
    Uninitialized(Vec<MaybeUninit<ZAllocationByte>>),
    Zeroed(Vec<ZAllocationByte>),
}

/// Records how initialization completed an allocator callback pair.
///
/// This stays deliberately pointer-free: a future owned-storage facade can
/// select only `DefaultPair` without comparing ABI callbacks or treating a
/// caller callback as a Rust allocator. `Mixed` is distinct because zlib
/// permits either callback to be supplied independently.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AllocatorProvenance {
    /// The state was not installed by an initializer that observed the ABI
    /// allocator pair (for example, an empty value before initialization).
    Unknown,
    /// Both callbacks were absent and zlib installed its default pair.
    DefaultPair,
    /// At least one callback came from the caller; each flag says whether the
    /// other half was filled with zlib's default callback.
    Mixed {
        default_zalloc: bool,
        default_zfree: bool,
    },
}

pub(crate) const UNKNOWN_ALLOCATOR_PROVENANCE: AllocatorProvenance = AllocatorProvenance::Unknown;

pub(crate) const fn allocator_pair_is_fully_default(provenance: &AllocatorProvenance) -> bool {
    matches!(provenance, AllocatorProvenance::DefaultPair)
}

/// Own values behind stable, pointer-free identity keys.
///
/// ABI code may retain its allocator token in a public pointer field while
/// implementation state stays here as ordinary Rust ownership.  The existing
/// default-allocation registry uses this now; stream state can adopt the same
/// owner only when every ABI state consumer has stopped treating that token as
/// a typed allocation.
pub(crate) struct IdentityOwner<T> {
    values: Mutex<HashMap<usize, T>>,
}

impl<T> IdentityOwner<T> {
    pub(crate) fn new() -> Self {
        Self {
            values: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn try_insert(&self, identity: usize, value: T) -> Result<(), T> {
        let mut values = self
            .values
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if values.contains_key(&identity) || values.try_reserve(1).is_err() {
            return Err(value);
        }
        values.insert(identity, value);
        Ok(())
    }

    pub(crate) fn take(&self, identity: usize) -> Option<T> {
        self.values
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(&identity)
    }

    pub(crate) fn with_ref<R>(&self, identity: usize, action: impl FnOnce(&T) -> R) -> Option<R> {
        let values = self
            .values
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        values.get(&identity).map(action)
    }

    pub(crate) fn with_mut<R>(
        &self,
        identity: usize,
        action: impl FnOnce(&mut T) -> R,
    ) -> Option<R> {
        let mut values = self
            .values
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        values.get_mut(&identity).map(action)
    }
}

/// Allocate, initialize, and expose one opaque stream-state value.
///
/// A zlib allocator callback returns uninitialized foreign storage.  Keep the
/// single typed initialization boundary shared by deflate and inflate so their
/// setup paths do not each need to establish the first Rust value themselves.
pub(crate) fn with_callback_state_slot<T, R>(
    strm: &mut crate::zlib_h::z_stream,
    value: T,
    initialize: impl FnOnce(&mut crate::zlib_h::z_stream, &mut T) -> R,
) -> Option<R> {
    let allocation = (strm.zalloc?)(
        strm.opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<T>() as crate::stdlib::uInt,
    ) as *mut ::core::mem::MaybeUninit<T>;
    let mut slot = ::core::ptr::NonNull::new(allocation)?;
    // The allocation is live and uniquely owned by this initialization path.
    // Write its first Rust value before exposing the typed reference.
    let state = unsafe { slot.as_mut().write(value) };
    Some(initialize(strm, state))
}

/// Keep a callback allocation as the ABI state token while owning its typed
/// Rust state separately.  The token remains the exact pointer later passed
/// to the matching `zfree` callback; implementation code addresses the Rust
/// value only through the pointer-free identity owner.
pub(crate) fn allocate_callback_owned_state<T, R>(
    strm: &mut crate::zlib_h::z_stream,
    owner: &IdentityOwner<T>,
    value: T,
    initialize: impl FnOnce(&mut crate::zlib_h::z_stream, &mut T) -> R,
) -> Option<R> {
    let allocation = (strm.zalloc?)(
        strm.opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<T>() as crate::stdlib::uInt,
    );
    if allocation.is_null() {
        return None;
    }
    let identity = allocation.addr();
    if let Err(value) = owner.try_insert(identity, value) {
        Some(strm.zfree.expect("allocator pair checked"))
            .expect("non-null function pointer")(strm.opaque, allocation);
        drop(value);
        return None;
    }
    strm.state = allocation.cast::<crate::src::deflate::internal_state>();
    let result = owner.with_mut(identity, |state| initialize(strm, state));
    if strm.state.is_null() {
        drop(owner.take(identity));
    }
    result
}
#[no_mangle]

pub static z_errmsg: [AtomicPtr<::core::ffi::c_char>; 10] = [
    AtomicPtr::new(b"need dictionary\0".as_ptr().cast_mut().cast()),
    AtomicPtr::new(b"stream end\0".as_ptr().cast_mut().cast()),
    AtomicPtr::new(b"\0".as_ptr().cast_mut().cast()),
    AtomicPtr::new(b"file error\0".as_ptr().cast_mut().cast()),
    AtomicPtr::new(b"stream error\0".as_ptr().cast_mut().cast()),
    AtomicPtr::new(b"data error\0".as_ptr().cast_mut().cast()),
    AtomicPtr::new(b"insufficient memory\0".as_ptr().cast_mut().cast()),
    AtomicPtr::new(b"buffer error\0".as_ptr().cast_mut().cast()),
    AtomicPtr::new(b"incompatible version\0".as_ptr().cast_mut().cast()),
    AtomicPtr::new(b"\0".as_ptr().cast_mut().cast()),
];
pub fn zlibVersion() -> &'static [::core::ffi::c_char; 15] {
    &crate::zlib_h::ZLIB_VERSION
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
pub fn zError(mut err: ::core::ffi::c_int) -> &'static AtomicPtr<::core::ffi::c_char> {
    &z_errmsg[(if err < -6 as ::core::ffi::c_int || err > 2 as ::core::ffi::c_int {
        9 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int - err
    }) as usize]
}
#[export_name = "zError"]

pub unsafe extern "C" fn zError_ffi(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    zError(err).load(Ordering::Relaxed)
}
fn zallocations() -> &'static IdentityOwner<ZAllocation> {
    static ALLOCATIONS: OnceLock<IdentityOwner<ZAllocation>> = OnceLock::new();
    ALLOCATIONS.get_or_init(IdentityOwner::new)
}

/// Install zlib's default allocator pair when a caller did not provide one.
///
/// This is deliberately limited to initialization: callers that supplied
/// either callback retain that exact callback and opaque value.  Keeping this
/// policy in safe implementation code lets the stream initializers share it
/// without treating arbitrary ABI callbacks as safe Rust functions.
pub(crate) fn install_default_allocators(
    strm: &mut crate::zlib_h::z_stream,
) -> AllocatorProvenance {
    let zalloc_defaulted = strm.zalloc.is_none();
    let zfree_defaulted = strm.zfree.is_none();
    if zalloc_defaulted {
        strm.zalloc = Some(zcalloc);
        strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if zfree_defaulted {
        strm.zfree = Some(zcfree);
    }
    if zalloc_defaulted && zfree_defaulted {
        AllocatorProvenance::DefaultPair
    } else {
        AllocatorProvenance::Mixed {
            default_zalloc: zalloc_defaulted,
            default_zfree: zfree_defaulted,
        }
    }
}

pub extern "C" fn zcalloc(
    _opaque: crate::stdlib::voidpf,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    let len = items.wrapping_mul(size) as usize;
    let Some(units) = len
        .checked_add(::core::mem::size_of::<ZAllocationByte>() - 1)
        .map(|rounded| rounded / ::core::mem::size_of::<ZAllocationByte>())
    else {
        return ::core::ptr::null_mut();
    };
    let mut allocation = if ::core::mem::size_of::<crate::stdlib::uInt>() > 2 {
        let mut allocation = Vec::new();
        if allocation.try_reserve_exact(units).is_err() {
            return ::core::ptr::null_mut();
        }
        ZAllocation::Uninitialized(allocation)
    } else {
        let mut allocation = Vec::new();
        if allocation.try_reserve_exact(units).is_err() {
            return ::core::ptr::null_mut();
        }
        allocation.resize(units, ZAllocationByte([0; 16]));
        ZAllocation::Zeroed(allocation)
    };
    let pointer = match &mut allocation {
        ZAllocation::Uninitialized(bytes) => bytes.as_mut_ptr().cast::<::core::ffi::c_void>(),
        ZAllocation::Zeroed(bytes) => bytes.as_mut_ptr().cast::<::core::ffi::c_void>(),
    };
    if len != 0 {
        if zallocations().try_insert(pointer.addr(), allocation).is_err() {
            return ::core::ptr::null_mut();
        }
    }
    pointer
}
#[export_name = "zcalloc"]

pub unsafe extern "C" fn zcalloc_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut items: ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> crate::stdlib::voidpf {
    zcalloc(opaque, items, size)
}
pub extern "C" fn zcfree(_opaque: crate::stdlib::voidpf, mut ptr: crate::stdlib::voidpf) {
    if ptr.is_null() {
        return;
    }
    let allocation = zallocations().take(ptr.addr());
    drop(allocation);
}
#[export_name = "zcfree"]

pub unsafe extern "C" fn zcfree_ffi(
    mut opaque: crate::stdlib::voidpf,
    mut ptr: crate::stdlib::voidpf,
) {
    zcfree(opaque, ptr)
}
