#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#![feature(register_tool)]
#![register_tool(c2rust)]

pub const fn c_char_bytes<const N: usize>(bytes: [u8; N]) -> [::core::ffi::c_char; N] {
    let mut chars = [0; N];
    let mut index = 0;
    while index < N {
        chars[index] = bytes[index] as ::core::ffi::c_char;
        index += 1;
    }
    chars
}

pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod internal {
    pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}
pub mod limits_h {
    pub const INT_MAX: ::core::ffi::c_int = crate::internal::__INT_MAX__;
}
pub mod gzguts_h {
    pub const GZBUFSIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;

    pub const GZ_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const GZ_READ: ::core::ffi::c_int = 7247 as ::core::ffi::c_int;

    pub const GZ_WRITE: ::core::ffi::c_int = 31153 as ::core::ffi::c_int;

    pub const GZ_APPEND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const LOOK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const COPY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const GZIP: ::core::ffi::c_int = 2;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct gz_state {
        pub x: crate::zlib_h::gzFile_s,
        pub mode: ::core::ffi::c_int,
        pub fd: ::core::ffi::c_int,
        pub path: *mut ::core::ffi::c_char,
        // This is private gzip bookkeeping. `gzFile` is opaque to callers,
        // so retaining the already-known path length avoids re-reading the
        // owned C string when an error message is assembled.
        pub path_len: crate::stdlib::z_size_t,
        pub size: ::core::ffi::c_uint,
        pub want: ::core::ffi::c_uint,
        pub in_0: *mut ::core::ffi::c_uchar,
        pub out: *mut ::core::ffi::c_uchar,
        pub direct: ::core::ffi::c_int,
        pub junk: ::core::ffi::c_int,
        pub how: ::core::ffi::c_int,
        pub again: ::core::ffi::c_int,
        pub start: crate::stdlib::off64_t,
        pub eof: ::core::ffi::c_int,
        pub past: ::core::ffi::c_int,
        pub level: ::core::ffi::c_int,
        pub strategy: ::core::ffi::c_int,
        pub reset: ::core::ffi::c_int,
        pub skip: crate::stdlib::off64_t,
        pub err: ::core::ffi::c_int,
        pub msg: *mut ::core::ffi::c_char,
        pub strm: crate::zlib_h::z_stream,
    }

    pub type gz_statep = *mut crate::gzguts_h::gz_state;
}
pub mod zutil_h {
    pub type uch = ::core::ffi::c_uchar;

    pub type uchf = crate::zutil_h::uch;

    pub type ush = ::core::ffi::c_ushort;

    pub type ushf = crate::zutil_h::ush;

    pub type ulg = ::core::ffi::c_ulong;

    pub const DEF_WBITS: ::core::ffi::c_int = crate::stdlib::MAX_WBITS;

    pub const DEF_MEM_LEVEL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

    pub const MIN_MATCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const MAX_MATCH: ::core::ffi::c_int = 258 as ::core::ffi::c_int;

    pub const PRESET_DICT: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub mod zlib_h {
    pub const ZLIB_VERSION: [::core::ffi::c_char; 15] = crate::c_char_bytes(*b"1.3.2.1-motley\0");

    // zlib invokes allocator hooks only with the ownership and size values
    // established by its stream contract.  The hooks receive opaque raw
    // values but have no additional Rust-side precondition, so callers that
    // have validated that contract need not cross an unsafe call boundary.
    pub type alloc_func = Option<
        extern "C" fn(
            crate::stdlib::voidpf,
            crate::stdlib::uInt,
            crate::stdlib::uInt,
        ) -> crate::stdlib::voidpf,
    >;

    pub type free_func =
        Option<extern "C" fn(crate::stdlib::voidpf, crate::stdlib::voidpf) -> ()>;

    pub type z_stream = crate::zlib_h::z_stream_s;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct z_stream_s {
        pub next_in: *mut crate::stdlib::Bytef,
        pub avail_in: crate::stdlib::uInt,
        pub total_in: crate::stdlib::uLong,
        pub next_out: *mut crate::stdlib::Bytef,
        pub avail_out: crate::stdlib::uInt,
        pub total_out: crate::stdlib::uLong,
        pub msg: *mut ::core::ffi::c_char,
        pub state: *mut crate::src::deflate::internal_state,
        pub zalloc: crate::zlib_h::alloc_func,
        pub zfree: crate::zlib_h::free_func,
        pub opaque: crate::stdlib::voidpf,
        pub data_type: ::core::ffi::c_int,
        pub adler: crate::stdlib::uLong,
        pub reserved: crate::stdlib::uLong,
    }

    pub type z_streamp = *mut crate::zlib_h::z_stream;

    pub type gz_header = crate::zlib_h::gz_header_s;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct gz_header_s {
        pub text: ::core::ffi::c_int,
        pub time: crate::stdlib::uLong,
        pub xflags: ::core::ffi::c_int,
        pub os: ::core::ffi::c_int,
        pub extra: *mut crate::stdlib::Bytef,
        pub extra_len: crate::stdlib::uInt,
        pub extra_max: crate::stdlib::uInt,
        pub name: *mut crate::stdlib::Bytef,
        pub name_max: crate::stdlib::uInt,
        pub comment: *mut crate::stdlib::Bytef,
        pub comm_max: crate::stdlib::uInt,
        pub hcrc: ::core::ffi::c_int,
        pub done: ::core::ffi::c_int,
    }

    pub type gz_headerp = *mut crate::zlib_h::gz_header;

    pub const Z_NO_FLUSH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const Z_PARTIAL_FLUSH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const Z_FULL_FLUSH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const Z_FINISH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

    pub const Z_BLOCK: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

    pub const Z_TREES: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

    pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const Z_NEED_DICT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const Z_ERRNO: ::core::ffi::c_int = -1 as ::core::ffi::c_int;

    pub const Z_STREAM_ERROR: ::core::ffi::c_int = -2 as ::core::ffi::c_int;

    pub const Z_DATA_ERROR: ::core::ffi::c_int = -3 as ::core::ffi::c_int;

    pub const Z_MEM_ERROR: ::core::ffi::c_int = -4 as ::core::ffi::c_int;

    pub const Z_BUF_ERROR: ::core::ffi::c_int = -5 as ::core::ffi::c_int;

    pub const Z_VERSION_ERROR: ::core::ffi::c_int = -6 as ::core::ffi::c_int;

    pub const Z_DEFAULT_COMPRESSION: ::core::ffi::c_int = -1 as ::core::ffi::c_int;

    pub const Z_FILTERED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const Z_HUFFMAN_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const Z_RLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const Z_FIXED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

    pub const Z_DEFAULT_STRATEGY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const Z_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const Z_TEXT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const Z_UNKNOWN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const Z_DEFLATED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

    pub const Z_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub type in_func = Option<
        extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_uchar,
        ) -> ::core::ffi::c_uint,
    >;

    pub type out_func = Option<
        extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_uchar,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >;

    pub type gzFile = *mut crate::zlib_h::gzFile_s;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct gzFile_s {
        pub have: ::core::ffi::c_uint,
        pub next: *mut ::core::ffi::c_uchar,
        pub pos: crate::stdlib::off64_t,
    }
}
pub mod stdlib {
    unsafe extern "C" {
        // This accessor has no inputs and only returns libc's thread-local
        // errno slot. Calling it cannot violate Rust memory safety; callers
        // still need an unsafe operation to read or write the returned raw
        // pointer.
        pub safe fn __errno_location() -> *mut ::core::ffi::c_int;
        // Gzip uses only the integer-only descriptor-control forms below:
        // `F_GETFL`, `F_SETFL`, `F_GETFD`, and `F_SETFD`. Invalid descriptors
        // or flags are reported by libc, and no Rust-managed memory is
        // borrowed or dereferenced at this boundary.
        pub(crate) safe fn fcntl(
            __fd: ::core::ffi::c_int,
            __cmd: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;

        // `open()` passes the pathname to the kernel, which reports an
        // invalid user address as `EFAULT`; it does not dereference or retain
        // Rust-managed memory. The gzip implementation supplies a live C
        // string, but the syscall itself has no Rust-side safety precondition.
        pub safe fn open(
            __file: *const ::core::ffi::c_char,
            __oflag: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: crate::__stddef_size_t_h::size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub safe fn malloc(__size: crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void;

        pub safe fn calloc(
            __nmemb: crate::__stddef_size_t_h::size_t,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        // `free` neither reads Rust-managed memory nor imposes an additional
        // Rust-side precondition. zlib's allocator adapters retain ownership
        // validation at their call sites.
        pub safe fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn memchr(
            __s: *const ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn strlen(__s: *const ::core::ffi::c_char) -> crate::__stddef_size_t_h::size_t;

        // `strerror()` takes only an error number. libc handles unknown values
        // by returning a diagnostic string, so it has no Rust-memory or
        // lifetime precondition for the call itself. Consumers still bind the
        // returned pointer only where their surrounding operation permits it.
        pub safe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
        // Seeking an arbitrary descriptor and closing an arbitrary descriptor
        // are defined by libc to report errors, not invoke undefined behavior.
        // Neither operation accepts a Rust reference or memory range.
        pub safe fn lseek64(
            __fd: ::core::ffi::c_int,
            __offset: crate::stdlib::__off64_t,
            __whence: ::core::ffi::c_int,
        ) -> crate::stdlib::__off64_t;

        pub safe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;

        // POSIX validates this transient user buffer in the kernel and
        // reports an invalid address as `EFAULT`; it neither retains nor
        // dereferences Rust-managed memory in-process.
        pub safe fn read(
            __fd: ::core::ffi::c_int,
            __buf: *mut ::core::ffi::c_void,
            __nbytes: crate::__stddef_size_t_h::size_t,
        ) -> crate::stdlib::ssize_t;

        // As with `read`, the kernel reports an invalid transient source as
        // `EFAULT` and does not retain the pointer after the syscall.
        pub safe fn write(
            __fd: ::core::ffi::c_int,
            __buf: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> crate::stdlib::ssize_t;
    }
    pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
    pub const EWOULDBLOCK: ::core::ffi::c_int = crate::stdlib::EAGAIN;
    pub const __O_LARGEFILE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;

    pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;

    pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;

    pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;

    pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;

    pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;

    pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;

    pub const O_LARGEFILE: ::core::ffi::c_int = crate::stdlib::__O_LARGEFILE;

    pub const O_CLOEXEC: ::core::ffi::c_int = crate::stdlib::__O_CLOEXEC;

    pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    pub type off_t = crate::stdlib::__off_t;

    pub type off64_t = crate::stdlib::__off64_t;

    pub type ssize_t = isize;
    pub type __off_t = ::core::ffi::c_long;

    pub type __off64_t = ::core::ffi::c_long;
    pub type z_size_t = crate::__stddef_size_t_h::size_t;

    pub const MAX_MEM_LEVEL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

    pub const MAX_WBITS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

    pub type Byte = ::core::ffi::c_uchar;

    pub type uInt = ::core::ffi::c_uint;

    pub type uLong = ::core::ffi::c_ulong;

    pub type Bytef = crate::stdlib::Byte;

    pub type charf = ::core::ffi::c_char;

    pub type intf = ::core::ffi::c_int;

    pub type uLongf = crate::stdlib::uLong;

    pub type voidpc = *const ::core::ffi::c_void;

    pub type voidpf = *mut ::core::ffi::c_void;

    pub type voidp = *mut ::core::ffi::c_void;

    pub type z_crc_t = ::core::ffi::c_uint;
}
pub mod src {
    pub mod adler32;
    pub mod compress;
    pub mod crc32;
    pub mod deflate;
    pub mod gzclose;
    pub mod gzlib;
    pub mod gzread;
    pub mod gzwrite;
    pub mod infback;
    pub mod inffast;
    pub mod inflate;
    pub mod inftrees;
    pub mod trees;
    pub mod uncompr;
    pub mod zutil;
} // mod src
