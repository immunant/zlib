#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(allocator_api)]
#![feature(register_tool)]
#![register_tool(c2rust)]

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

    #[repr(C)]

    pub struct gz_state {
        pub x: crate::zlib_h::gzFile_s,
        pub mode: ::core::ffi::c_int,
        pub fd: Option<::std::os::fd::OwnedFd>,
        pub path: ::core::mem::ManuallyDrop<Option<::std::ffi::CString>>,
        pub size: ::core::ffi::c_uint,
        pub want: ::core::ffi::c_uint,
        pub in_0: ::core::mem::ManuallyDrop<Vec<u8>>,
        pub out: ::core::mem::ManuallyDrop<Vec<u8>>,
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
        pub msg: ::core::mem::ManuallyDrop<Option<::std::ffi::CString>>,
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
pub mod zlib_h {
    pub const ZLIB_VERSION: [::core::ffi::c_char; 15] = [
        b'1' as ::core::ffi::c_char,
        b'.' as ::core::ffi::c_char,
        b'3' as ::core::ffi::c_char,
        b'.' as ::core::ffi::c_char,
        b'2' as ::core::ffi::c_char,
        b'.' as ::core::ffi::c_char,
        b'1' as ::core::ffi::c_char,
        b'-' as ::core::ffi::c_char,
        b'm' as ::core::ffi::c_char,
        b'o' as ::core::ffi::c_char,
        b't' as ::core::ffi::c_char,
        b'l' as ::core::ffi::c_char,
        b'e' as ::core::ffi::c_char,
        b'y' as ::core::ffi::c_char,
        0,
    ];

    pub type alloc_func = Option<
        unsafe extern "C" fn(
            crate::stdlib::voidpf,
            crate::stdlib::uInt,
            crate::stdlib::uInt,
        ) -> crate::stdlib::voidpf,
    >;

    pub type free_func =
        Option<unsafe extern "C" fn(crate::stdlib::voidpf, crate::stdlib::voidpf) -> ()>;

    pub type z_stream = crate::zlib_h::z_stream_s;

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

    pub fn copy_z_stream(dest: &mut z_stream, source: &z_stream) {
        dest.next_in = source.next_in;
        dest.avail_in = source.avail_in;
        dest.total_in = source.total_in;
        dest.next_out = source.next_out;
        dest.avail_out = source.avail_out;
        dest.total_out = source.total_out;
        dest.msg = source.msg;
        dest.state = source.state;
        dest.zalloc = source.zalloc;
        dest.zfree = source.zfree;
        dest.opaque = source.opaque;
        dest.data_type = source.data_type;
        dest.adler = source.adler;
        dest.reserved = source.reserved;
    }

    pub type z_streamp = *mut crate::zlib_h::z_stream;

    pub type gz_header = crate::zlib_h::gz_header_s;

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
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_uchar,
        ) -> ::core::ffi::c_uint,
    >;

    pub type out_func = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_uchar,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int,
    >;

    pub type gzFile = *mut crate::zlib_h::gzFile_s;

    #[repr(C)]

    pub struct gzFile_s {
        pub have: ::core::ffi::c_uint,
        pub next: *mut ::core::ffi::c_uchar,
        pub pos: crate::stdlib::off64_t,
    }
}
pub mod stdlib {
    extern "C" {
        pub fn __errno_location() -> *mut ::core::ffi::c_int;
        pub fn fcntl(
            __fd: ::core::ffi::c_int,
            __cmd: ::core::ffi::c_int,
            ...
        ) -> ::core::ffi::c_int;

        pub fn open(
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
        pub fn malloc(__size: crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void;

        pub fn calloc(
            __nmemb: crate::__stddef_size_t_h::size_t,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

        pub fn free(__ptr: *mut ::core::ffi::c_void);
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

        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
        pub fn lseek64(
            __fd: ::core::ffi::c_int,
            __offset: crate::stdlib::__off64_t,
            __whence: ::core::ffi::c_int,
        ) -> crate::stdlib::__off64_t;

        pub fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;

        pub fn read(
            __fd: ::core::ffi::c_int,
            __buf: *mut ::core::ffi::c_void,
            __nbytes: crate::__stddef_size_t_h::size_t,
        ) -> crate::stdlib::ssize_t;

        pub fn write(
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
