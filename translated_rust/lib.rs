#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(register_tool)]
#![feature(strict_provenance)]
#![register_tool(c2rust)]

pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod internal {
    pub const fn c_char_array<const N: usize>(bytes: &[u8; N]) -> [::core::ffi::c_char; N] {
        let mut chars = [0; N];
        let mut index = 0;
        while index < N {
            chars[index] = bytes[index] as ::core::ffi::c_char;
            index += 1;
        }
        chars
    }
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
        pub fd: ::core::ffi::c_int,
        // Write descriptors are adopted at the C ABI boundary.  Keeping the
        // owned handle here lets the gzip write path use `std::io::Write`
        // instead of calling the libc write entry point.
        pub write_file: Option<std::fs::File>,
        // Read descriptors use the same ownership model.  `gzread` can then
        // pass bounded Rust slices to `Read::read` instead of handing a raw
        // buffer to libc.
        pub read_file: Option<std::fs::File>,
        // The reader uses flate2's safe streaming gzip API.  This is separate
        // from `strm`, which is retained for the public zlib stream ABI and
        // the translated implementation used by other entry points.
        pub gzip_inflater: Option<flate2::Decompress>,
        // `gz_state` is opaque at the C ABI boundary.  Keep the text it owns
        // in Rust containers instead of separately allocated C buffers.
        pub path: std::ffi::CString,
        pub size: ::core::ffi::c_uint,
        pub want: ::core::ffi::c_uint,
        // Gzip I/O storage is entirely Rust-owned.  The translated stream
        // receives temporary cursors into these buffers only while it runs.
        pub in_buf: Vec<u8>,
        pub out_buf: Vec<u8>,
        // End of the pending input in `in_buf`.  This keeps buffered-write
        // progress in a Rust index rather than deriving it from raw cursors.
        pub in_end: usize,
        pub out_start: usize,
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
        pub msg: Option<std::ffi::CString>,
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

#[macro_export]
macro_rules! input_cursor {
    ($pointer:expr) => {
        crate::zlib_h::InputBuffer(::core::num::NonZeroUsize::new(($pointer).addr()))
    };
}

#[macro_export]
macro_rules! input_pointer {
    ($cursor:expr) => {
        ::core::ptr::with_exposed_provenance_mut::<crate::stdlib::Bytef>(
            ($cursor).0.expect("non-null input cursor").get(),
        )
    };
}

#[macro_export]
macro_rules! output_cursor {
    ($pointer:expr) => {
        crate::zlib_h::OutputBuffer(::core::num::NonZeroUsize::new(($pointer).addr()))
    };
}

#[macro_export]
macro_rules! output_pointer {
    ($cursor:expr) => {
        ::core::ptr::with_exposed_provenance_mut::<crate::stdlib::Bytef>(
            ($cursor).0.expect("non-null output cursor").get(),
        )
    };
}

/// Own a copy of a fixed zlib diagnostic while keeping `z_stream_s::msg`
/// pointer-sized for the C ABI.
#[macro_export]
macro_rules! stream_message {
    ($message:expr) => {
        Some(::std::rc::Rc::new(($message).to_owned()))
    };
}
pub mod zlib_h {
    pub const ZLIB_VERSION: [::core::ffi::c_char; 15] = unsafe {
        ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"1.3.2.1-motley\0")
    };

    /// A nullable, pointer-sized marker for the stream's internal Rust
    /// allocator.  Keeping the marker as a non-zero integer maintains the C
    /// stream layout without retaining a callable raw allocator pointer.
    pub type alloc_func = Option<::core::num::NonZeroUsize>;
    pub type free_func = Option<::core::num::NonZeroUsize>;

    pub fn default_stream_allocator() -> ::core::num::NonZeroUsize {
        ::core::num::NonZeroUsize::new(1).expect("one is non-zero")
    }

    pub type z_stream = crate::zlib_h::z_stream_s;

    /// A nullable, ABI-compatible input cursor.  The C cursor address is
    /// represented as a non-zero address, so the stream no longer stores a
    /// raw pointer field directly.  Conversion back to `NonNull` is confined
    /// to the existing translated pointer-consuming code.
    #[repr(transparent)]
    #[derive(Copy, Clone, Default)]
    pub struct InputBuffer(pub Option<::core::num::NonZeroUsize>);

    impl InputBuffer {
        pub fn advance(self, bytes: usize) -> Self {
            Self(self.0.and_then(|address| {
                ::core::num::NonZeroUsize::new(address.get().wrapping_add(bytes))
            }))
        }
    }

    /// A nullable cursor into the caller-owned output range.  zlib's stream
    /// API does not own this memory, so an address cursor preserves that API
    /// while keeping the stream itself free of raw pointer fields.
    #[repr(transparent)]
    #[derive(Copy, Clone, Default)]
    pub struct OutputBuffer(pub Option<::core::num::NonZeroUsize>);

    impl OutputBuffer {
        pub fn is_null(self) -> bool {
            self.0.is_none()
        }

        pub fn advance(self, bytes: usize) -> Self {
            Self(self.0.and_then(|address| {
                ::core::num::NonZeroUsize::new(address.get().wrapping_add(bytes))
            }))
        }

        pub fn bytes_from(self, start_address: usize) -> usize {
            self.0
                .map_or(0, |address| address.get().wrapping_sub(start_address))
        }
    }

    /// A nullable, ABI-compatible allocator context.  C can still store its
    /// `void *` context in this pointer-sized field, while Rust allocator
    /// calls carry an explicit non-pointer value.
    #[repr(transparent)]
    #[derive(Copy, Clone, Default)]
    pub struct Opaque(pub Option<::core::num::NonZeroUsize>);

    impl Opaque {
        pub fn from_address(address: usize) -> Self {
            Self(::core::num::NonZeroUsize::new(address))
        }
    }

    /// The compressor and decompressor use different private state types.  C
    /// stored either allocation behind `z_stream::state`; Rust keeps that
    /// choice explicit and owns the allocation instead.
    #[derive(Clone)]
    pub enum StreamState {
        Deflate(::std::rc::Rc<::std::cell::RefCell<crate::src::deflate::internal_state>>),
        Inflate(::std::rc::Rc<::std::cell::RefCell<crate::src::inflate::inflate_state>>),
    }

    #[derive(Clone)]
    #[repr(C)]
    pub struct z_stream_s {
        pub next_in: InputBuffer,
        pub avail_in: crate::stdlib::uInt,
        pub total_in: crate::stdlib::uLong,
        pub next_out: OutputBuffer,
        pub avail_out: crate::stdlib::uInt,
        pub total_out: crate::stdlib::uLong,
        // `Rc<CString>` owns a fixed diagnostic without a raw pointer while the
        // nullable smart pointer retains the original one-word C ABI layout.
        pub msg: Option<::std::rc::Rc<::std::ffi::CString>>,
        // This remains one word, but is now a nullable Rust-owned state
        // handle rather than an untyped allocation pointer.
        pub state: Option<Box<StreamState>>,
        pub zalloc: crate::zlib_h::alloc_func,
        pub zfree: crate::zlib_h::free_func,
        pub opaque: Opaque,
        pub data_type: ::core::ffi::c_int,
        pub adler: crate::stdlib::uLong,
        pub reserved: crate::stdlib::uLong,
    }

    pub fn set_stream_message(stream: &mut z_stream_s, message: &'static ::core::ffi::CStr) {
        stream.msg = crate::stream_message!(message);
    }

    pub fn clear_stream_message(stream: &mut z_stream_s) {
        stream.msg = None;
    }

    impl z_stream_s {
        pub fn deflate_state(
            &self,
        ) -> Option<::std::rc::Rc<::std::cell::RefCell<crate::src::deflate::internal_state>>>
        {
            match self.state.as_deref() {
                Some(StreamState::Deflate(state)) => Some(::std::rc::Rc::clone(state)),
                _ => None,
            }
        }

        pub fn inflate_state(
            &self,
        ) -> Option<::std::rc::Rc<::std::cell::RefCell<crate::src::inflate::inflate_state>>>
        {
            match self.state.as_deref() {
                Some(StreamState::Inflate(state)) => Some(::std::rc::Rc::clone(state)),
                _ => None,
            }
        }

        pub fn set_deflate_state(&mut self, state: crate::src::deflate::internal_state) {
            self.state = Some(Box::new(StreamState::Deflate(::std::rc::Rc::new(
                ::std::cell::RefCell::new(state),
            ))));
        }

        pub fn set_inflate_state(&mut self, state: crate::src::inflate::inflate_state) {
            self.state = Some(Box::new(StreamState::Inflate(::std::rc::Rc::new(
                ::std::cell::RefCell::new(state),
            ))));
        }
    }

    pub type z_streamp = *mut crate::zlib_h::z_stream;

    pub type gz_header = crate::zlib_h::gz_header_s;

    #[derive(Clone)]
    pub struct gz_header_s {
        pub text: ::core::ffi::c_int,
        pub time: crate::stdlib::uLong,
        pub xflags: ::core::ffi::c_int,
        pub os: ::core::ffi::c_int,
        pub extra: Option<Vec<crate::stdlib::Bytef>>,
        pub extra_len: crate::stdlib::uInt,
        pub extra_max: crate::stdlib::uInt,
        pub name: Option<Vec<crate::stdlib::Bytef>>,
        pub name_max: crate::stdlib::uInt,
        pub comment: Option<Vec<crate::stdlib::Bytef>>,
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

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct gzFile_s {
        pub have: ::core::ffi::c_uint,
        /// Byte offset of the next buffered byte in `gz_state::out_buf`.
        /// `gzFile_s` is opaque at the C boundary, so the cursor can retain
        /// Rust's bounds and provenance guarantees instead of storing a raw
        /// pointer into the vector.
        pub next: usize,
        pub pos: crate::stdlib::off64_t,
    }
}
pub mod stdlib {
    unsafe extern "C" {
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
        pub safe fn malloc(__size: crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void;

        pub safe fn calloc(
            __nmemb: crate::__stddef_size_t_h::size_t,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;

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
