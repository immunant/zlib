// =============== BEGIN inflate_h ================
pub type inflate_mode = ::core::ffi::c_uint;

pub const HEAD: crate::src::inflate::inflate_mode = 16180;

pub const FLAGS: crate::src::inflate::inflate_mode = 16181;

pub const TIME: crate::src::inflate::inflate_mode = 16182;

pub const OS: crate::src::inflate::inflate_mode = 16183;

pub const EXLEN: crate::src::inflate::inflate_mode = 16184;

pub const EXTRA: crate::src::inflate::inflate_mode = 16185;

pub const NAME: crate::src::inflate::inflate_mode = 16186;

pub const COMMENT: crate::src::inflate::inflate_mode = 16187;

pub const HCRC: crate::src::inflate::inflate_mode = 16188;

pub const DICTID: crate::src::inflate::inflate_mode = 16189;

pub const DICT: crate::src::inflate::inflate_mode = 16190;

pub const TYPE: crate::src::inflate::inflate_mode = 16191;

pub const TYPEDO: crate::src::inflate::inflate_mode = 16192;

pub const STORED: crate::src::inflate::inflate_mode = 16193;

pub const COPY_: crate::src::inflate::inflate_mode = 16194;

pub const COPY_1: crate::src::inflate::inflate_mode = 16195;

pub const TABLE: crate::src::inflate::inflate_mode = 16196;

pub const LENLENS: crate::src::inflate::inflate_mode = 16197;

pub const CODELENS: crate::src::inflate::inflate_mode = 16198;

pub const LEN_: crate::src::inflate::inflate_mode = 16199;

pub const LEN: crate::src::inflate::inflate_mode = 16200;

pub const LENEXT: crate::src::inflate::inflate_mode = 16201;

pub const DIST: crate::src::inflate::inflate_mode = 16202;

pub const DISTEXT: crate::src::inflate::inflate_mode = 16203;

pub const MATCH: crate::src::inflate::inflate_mode = 16204;

pub const LIT: crate::src::inflate::inflate_mode = 16205;

pub const CHECK: crate::src::inflate::inflate_mode = 16206;

pub const LENGTH: crate::src::inflate::inflate_mode = 16207;

pub const DONE: crate::src::inflate::inflate_mode = 16208;

pub const BAD: crate::src::inflate::inflate_mode = 16209;

pub const MEM: crate::src::inflate::inflate_mode = 16210;

pub const SYNC: crate::src::inflate::inflate_mode = 16211;

#[derive(Copy, Clone)]
pub enum CodeTable {
    Empty,
    Dynamic(usize),
    FixedLens,
    FixedDists,
}

#[repr(C)]

pub struct inflate_state {
    pub mode: crate::src::inflate::inflate_mode,
    pub last: ::core::ffi::c_int,
    pub wrap: ::core::ffi::c_int,
    pub havedict: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub dmax: ::core::ffi::c_uint,
    pub check: ::core::ffi::c_ulong,
    pub total: ::core::ffi::c_ulong,
    pub head: Option<::std::rc::Rc<::std::cell::RefCell<crate::zlib_h::gz_header>>>,
    pub wbits: ::core::ffi::c_uint,
    pub wsize: ::core::ffi::c_uint,
    pub whave: ::core::ffi::c_uint,
    pub wnext: ::core::ffi::c_uint,
    pub window: Option<Vec<::core::ffi::c_uchar>>,
    pub hold: ::core::ffi::c_ulong,
    pub bits: ::core::ffi::c_uint,
    pub length: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub extra: ::core::ffi::c_uint,
    pub lencode: CodeTable,
    pub distcode: CodeTable,
    pub lenbits: ::core::ffi::c_uint,
    pub distbits: ::core::ffi::c_uint,
    pub ncode: ::core::ffi::c_uint,
    pub nlen: ::core::ffi::c_uint,
    pub ndist: ::core::ffi::c_uint,
    pub have: ::core::ffi::c_uint,
    pub next: *mut crate::src::inftrees::code,
    pub lens: [::core::ffi::c_ushort; 320],
    pub work: [::core::ffi::c_ushort; 288],
    pub codes: [crate::src::inftrees::code; 1444],
    pub sane: ::core::ffi::c_int,
    pub back: ::core::ffi::c_int,
    pub was: ::core::ffi::c_uint,
}

impl Default for inflate_state {
    fn default() -> Self {
        let code = crate::src::inftrees::code {
            op: 0,
            bits: 0,
            val: 0,
        };
        Self {
            mode: HEAD,
            last: 0,
            wrap: 0,
            havedict: 0,
            flags: 0,
            dmax: 0,
            check: 0,
            total: 0,
            head: None,
            wbits: 0,
            wsize: 0,
            whave: 0,
            wnext: 0,
            window: None,
            hold: 0,
            bits: 0,
            length: 0,
            offset: 0,
            extra: 0,
            lencode: CodeTable::Empty,
            distcode: CodeTable::Empty,
            lenbits: 0,
            distbits: 0,
            ncode: 0,
            nlen: 0,
            ndist: 0,
            have: 0,
            next: ::core::ptr::null_mut(),
            lens: [0; 320],
            work: [0; 288],
            codes: [code; 1444],
            sane: 0,
            back: 0,
            was: 0,
        }
    }
}

impl inflate_state {
    pub fn code_at(&self, table: CodeTable, index: usize) -> crate::src::inftrees::code {
        let code = match table {
            CodeTable::Empty => None,
            CodeTable::Dynamic(start) => start
                .checked_add(index)
                .and_then(|index| self.codes.get(index)),
            CodeTable::FixedLens => crate::src::inftrees::inffixed_h::lenfix.get(index),
            CodeTable::FixedDists => crate::src::inftrees::inffixed_h::distfix.get(index),
        };
        *code.expect("valid inflate code table index")
    }

    pub fn lencode_at(&self, index: usize) -> crate::src::inftrees::code {
        self.code_at(self.lencode, index)
    }

    pub fn distcode_at(&self, index: usize) -> crate::src::inftrees::code {
        self.code_at(self.distcode, index)
    }

    fn copy_for_inflate_copy(&self) -> Option<Self> {
        let window = if let Some(window) = self.window.as_ref() {
            let mut copied_window = Vec::new();
            if copied_window.try_reserve_exact(window.len()).is_err() {
                return None;
            }
            copied_window.extend_from_slice(window);
            Some(copied_window)
        } else {
            None
        };
        Some(Self {
            mode: self.mode,
            last: self.last,
            wrap: self.wrap,
            havedict: self.havedict,
            flags: self.flags,
            dmax: self.dmax,
            check: self.check,
            total: self.total,
            head: self.head.clone(),
            wbits: self.wbits,
            wsize: self.wsize,
            whave: self.whave,
            wnext: self.wnext,
            window,
            hold: self.hold,
            bits: self.bits,
            length: self.length,
            offset: self.offset,
            extra: self.extra,
            lencode: self.lencode,
            distcode: self.distcode,
            lenbits: self.lenbits,
            distbits: self.distbits,
            ncode: self.ncode,
            nlen: self.nlen,
            ndist: self.ndist,
            have: self.have,
            next: self.next,
            lens: self.lens,
            work: self.work,
            codes: self.codes,
            sane: self.sane,
            back: self.back,
            was: self.was,
        })
    }
}
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::adler32::adler32;
pub use crate::src::crc32::crc32;
pub use crate::src::deflate::internal_state;
pub use crate::src::inftrees::code;
pub use crate::src::inftrees::codetype;
pub use crate::src::inftrees::inflate_fixed;
pub use crate::src::inftrees::inflate_table;
pub use crate::src::inftrees::CODES;
pub use crate::src::inftrees::DISTS;
pub use crate::src::inftrees::ENOUGH;
pub use crate::src::inftrees::ENOUGH_DISTS;
pub use crate::src::inftrees::ENOUGH_LENS;
pub use crate::src::inftrees::LENS;
pub use crate::src::zutil::zcalloc;
pub use crate::src::zutil::zcfree;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::stdlib::MAX_WBITS;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gz_header;
pub use crate::zlib_h::gz_header_s;
pub use crate::zlib_h::gz_headerp;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BLOCK;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_DEFLATED;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NEED_DICT;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
pub use crate::zlib_h::Z_TREES;
pub use crate::zlib_h::Z_VERSION_ERROR;
pub use crate::zutil_h::DEF_WBITS;

unsafe extern "C" fn inflateStateCheck(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as ::core::ffi::c_int;
    }
    let Some(state_handle) = (&*strm).inflate_state() else {
        return 1 as ::core::ffi::c_int;
    };
    let state = state_handle.borrow();
    if ((*state).mode as ::core::ffi::c_uint)
        < crate::src::inflate::HEAD as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*state).mode as ::core::ffi::c_uint
            > crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

fn inflate_state_invalid(strm: &crate::zlib_h::z_stream) -> bool {
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return true;
    }
    let Some(state_handle) = strm.inflate_state() else {
        return true;
    };
    let state = state_handle.borrow();
    (state.mode as ::core::ffi::c_uint)
        < crate::src::inflate::HEAD as ::core::ffi::c_int as ::core::ffi::c_uint
        || (state.mode as ::core::ffi::c_uint)
            > crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
}
pub unsafe extern "C" fn inflateResetKeep(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = (&*strm)
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    (*state).total = 0 as ::core::ffi::c_ulong;
    (*strm).total_out = (*state).total as crate::stdlib::uLong;
    (*strm).total_in = (*strm).total_out;
    crate::zlib_h::clear_stream_message(&mut *strm);
    (*strm).data_type = 0 as ::core::ffi::c_int;
    if (*state).wrap != 0 {
        (*strm).adler = ((*state).wrap & 1 as ::core::ffi::c_int) as crate::stdlib::uLong;
    }
    (*state).mode = crate::src::inflate::HEAD;
    (*state).last = 0 as ::core::ffi::c_int;
    (*state).havedict = 0 as ::core::ffi::c_int;
    (*state).flags = -1 as ::core::ffi::c_int;
    (*state).dmax = 32768 as ::core::ffi::c_uint;
    (*state).head = None;
    (*state).hold = 0 as ::core::ffi::c_ulong;
    (*state).bits = 0 as ::core::ffi::c_uint;
    (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
    (*state).distcode = CodeTable::Dynamic(0);
    (*state).lencode = CodeTable::Dynamic(0);
    (*state).sane = 1 as ::core::ffi::c_int;
    (*state).back = -1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateResetKeep(strm)
}
pub unsafe extern "C" fn inflateReset(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = (&*strm)
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    (*state).wsize = 0 as ::core::ffi::c_uint;
    (*state).whave = 0 as ::core::ffi::c_uint;
    (*state).wnext = 0 as ::core::ffi::c_uint;
    drop(state);
    return inflateResetKeep(strm);
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateReset(strm)
}
pub unsafe extern "C" fn inflateReset2(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut wrap: ::core::ffi::c_int = 0;
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = (&*strm)
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    if windowBits < 0 as ::core::ffi::c_int {
        if windowBits < -15 as ::core::ffi::c_int {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        wrap = 0 as ::core::ffi::c_int;
        windowBits = -windowBits;
    } else {
        wrap = (windowBits >> 4 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int;
        if windowBits < 48 as ::core::ffi::c_int {
            windowBits &= 15 as ::core::ffi::c_int;
        }
    }
    if windowBits != 0
        && (windowBits < 8 as ::core::ffi::c_int || windowBits > 15 as ::core::ffi::c_int)
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).window.is_some() && (*state).wbits != windowBits as ::core::ffi::c_uint {
        (*state).window = None;
    }
    (*state).wrap = wrap;
    (*state).wbits = windowBits as ::core::ffi::c_uint;
    drop(state);
    return inflateReset(strm);
}
#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateReset2(strm, windowBits)
}
pub unsafe extern "C" fn inflateInit2_(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if version.is_null()
        || *version.offset(0 as isize) as ::core::ffi::c_int
            != crate::zlib_h::ZLIB_VERSION[0 as usize] as ::core::ffi::c_int
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::zlib_h::clear_stream_message(&mut *strm);
    if (*strm).zalloc.is_none() {
        (*strm).zalloc = Some(crate::zlib_h::default_stream_allocator());
        (*strm).opaque = crate::zlib_h::Opaque::default();
    }
    if (*strm).zfree.is_none() {
        (*strm).zfree = Some(crate::zlib_h::default_stream_allocator());
    }
    let mut state = crate::src::inflate::inflate_state::default();
    state.mode = crate::src::inflate::HEAD;
    (*strm).set_inflate_state(state);
    ret = inflateReset2(strm, windowBits);
    if ret != crate::zlib_h::Z_OK {
        (*strm).state = None;
    }
    return ret;
}
#[export_name = "inflateInit2_"]

pub unsafe extern "C" fn inflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit2_(strm, windowBits, version, stream_size)
}
pub unsafe extern "C" fn inflateInit_(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return inflateInit2_(strm, crate::zutil_h::DEF_WBITS, version, stream_size);
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit_(strm, version, stream_size)
}
pub unsafe extern "C" fn inflatePrime(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if bits == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_OK;
    }
    let state_handle = (&*strm)
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    if bits < 0 as ::core::ffi::c_int {
        (*state).hold = 0 as ::core::ffi::c_ulong;
        (*state).bits = 0 as ::core::ffi::c_uint;
        return crate::zlib_h::Z_OK;
    }
    if bits > 16 as ::core::ffi::c_int
        || ((*state).bits as crate::stdlib::uInt).wrapping_add(bits as crate::stdlib::uInt)
            > 32 as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    value = (value as ::core::ffi::c_long
        & ((1 as ::core::ffi::c_long) << bits) - 1 as ::core::ffi::c_long)
        as ::core::ffi::c_int;
    (*state).hold = (*state)
        .hold
        .wrapping_add((value as ::core::ffi::c_ulong) << (*state).bits);
    (*state).bits = (*state)
        .bits
        .wrapping_add(bits as crate::stdlib::uInt as ::core::ffi::c_uint);
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflatePrime(strm, bits, value)
}
fn updatewindow_from_slice(
    state: &mut crate::src::inflate::inflate_state,
    source: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let mut copy = source.len();
    if state.window.is_none() {
        let size = (1usize) << state.wbits;
        let mut window = Vec::new();
        if window.try_reserve_exact(size).is_err() {
            return 1 as ::core::ffi::c_int;
        }
        window.resize(size, 0);
        state.window = Some(window);
    }
    if state.wsize == 0 as ::core::ffi::c_uint {
        state.wsize = (1 as ::core::ffi::c_uint) << state.wbits;
        state.wnext = 0 as ::core::ffi::c_uint;
        state.whave = 0 as ::core::ffi::c_uint;
    }
    let wsize = state.wsize as usize;
    if copy >= wsize {
        let window = state.window.as_mut().expect("window was allocated above");
        window[..wsize].copy_from_slice(&source[source.len() - wsize..]);
        state.wnext = 0 as ::core::ffi::c_uint;
        state.whave = state.wsize;
    } else {
        let mut dist = wsize - state.wnext as usize;
        if dist > copy {
            dist = copy;
        }
        let window = state.window.as_mut().expect("window was allocated above");
        let begin = state.wnext as usize;
        let input_start = source.len() - copy;
        window[begin..begin + dist].copy_from_slice(&source[input_start..input_start + dist]);
        copy -= dist;
        if copy != 0 {
            let window = state.window.as_mut().expect("window was allocated above");
            window[..copy].copy_from_slice(&source[source.len() - copy..]);
            state.wnext = copy as ::core::ffi::c_uint;
            state.whave = state.wsize;
        } else {
            state.wnext = state.wnext.wrapping_add(dist as ::core::ffi::c_uint);
            if state.wnext == state.wsize {
                state.wnext = 0 as ::core::ffi::c_uint;
            }
            if state.whave < state.wsize {
                state.whave = state.whave.wrapping_add(dist as ::core::ffi::c_uint);
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn updatewindow(
    _strm: crate::zlib_h::z_streamp,
    state: &mut crate::src::inflate::inflate_state,
    end: *const crate::stdlib::Bytef,
    copy: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let source = ::core::slice::from_raw_parts(end.sub(copy as usize), copy as usize);
    updatewindow_from_slice(state, source)
}
pub unsafe extern "C" fn inflate(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut put: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut have: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut in_0: ::core::ffi::c_uint = 0;
    let mut out: ::core::ffi::c_uint = 0;
    let mut copy: ::core::ffi::c_uint = 0;
    let mut from: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut window_index: Option<usize> = None;
    let mut here: crate::src::inftrees::code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut last: crate::src::inftrees::code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut len: ::core::ffi::c_uint = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut hbuf: [::core::ffi::c_uchar; 4] = [0; 4];
    static mut order: [::core::ffi::c_ushort; 19] = [
        16 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_ushort,
        8 as ::core::ffi::c_ushort,
        7 as ::core::ffi::c_ushort,
        9 as ::core::ffi::c_ushort,
        6 as ::core::ffi::c_ushort,
        10 as ::core::ffi::c_ushort,
        5 as ::core::ffi::c_ushort,
        11 as ::core::ffi::c_ushort,
        4 as ::core::ffi::c_ushort,
        12 as ::core::ffi::c_ushort,
        3 as ::core::ffi::c_ushort,
        13 as ::core::ffi::c_ushort,
        2 as ::core::ffi::c_ushort,
        14 as ::core::ffi::c_ushort,
        1 as ::core::ffi::c_ushort,
        15 as ::core::ffi::c_ushort,
    ];
    if inflateStateCheck(strm) != 0
        || (*strm).next_out.is_null()
        || (*strm).next_in.0.is_none() && (*strm).avail_in != 0 as crate::stdlib::uInt
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = (&*strm)
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    if (*state).mode as ::core::ffi::c_uint
        == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*state).mode = crate::src::inflate::TYPEDO;
    }
    put = crate::output_pointer!((*strm).next_out) as *mut ::core::ffi::c_uchar;
    left = (*strm).avail_out as ::core::ffi::c_uint;
    let input_cursor = (*strm).next_in;
    next = match input_cursor.0 {
        Some(address) => ::core::ptr::with_exposed_provenance_mut(address.get()),
        None => ::core::ptr::null_mut(),
    } as *mut ::core::ffi::c_uchar;
    have = (*strm).avail_in as ::core::ffi::c_uint;
    hold = (*state).hold;
    bits = (*state).bits;
    in_0 = have;
    out = left;
    ret = crate::zlib_h::Z_OK;
    '_inf_leave: loop {
        'c_2425: {
            'c_2327: {
                'c_2422: {
                    'c_2325: {
                        's_2462: {
                            'c_2322: {
                                'c_2410: {
                                    'c_2319: {
                                        'c_2398: {
                                            'c_2397: {
                                                'c_2317: {
                                                    'c_2340: {
                                                        'c_2443: {
                                                            'c_2339: {
                                                                's_519: {
                                                                    'c_2356: {
                                                                        's_1689: {
                                                                            'c_2355: {
                                                                                's_425: {
                                                                                    'c_2336: {
                                                                                        's_1582: {
                                                                                            match (*state).mode as ::core::ffi::c_uint {
                                                                                                16180 => {
                                                                                                    if (*state).wrap == 0 as ::core::ffi::c_int {
                                                                                                        (*state).mode = crate::src::inflate::TYPEDO;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if have == 0 as ::core::ffi::c_uint {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            have = have.wrapping_sub(1);
                                                                                                            let c2rust_fresh0 = next;
                                                                                                            next = next.offset(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (*c2rust_fresh0 as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        if (*state).wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if (*state).wbits == 0 as ::core::ffi::c_uint {
                                                                                                                (*state).wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            (*state).check = crate::src::crc32::crc32(
                                                                                                                0 as crate::stdlib::uLong,
                                                                                                                &[],
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            (*state).check = crate::src::crc32::crc32(
                                                                                                                (*state).check as crate::stdlib::uLong,
                                                                                                                &hbuf[..2],
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                            (*state).mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            if let Some(head) = (*state).head.clone() {
                                                                                                                head.borrow_mut().done = -1 as ::core::ffi::c_int;
                                                                                                            }
                                                                                                            if (*state).wrap & 1 as ::core::ffi::c_int == 0
                                                                                                                || (((hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(hold >> 8 as ::core::ffi::c_int)
                                                                                                                    .wrapping_rem(31 as ::core::ffi::c_ulong) != 0
                                                                                                            {
                                                                                                                crate::zlib_h::set_stream_message(&mut *strm, c"incorrect header check");
                                                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                crate::zlib_h::set_stream_message(&mut *strm, c"unknown compression method");
                                                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else {
                                                                                                                hold >>= 4 as ::core::ffi::c_int;
                                                                                                                bits = bits
                                                                                                                    .wrapping_sub(
                                                                                                                        4 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                                    );
                                                                                                                len = (hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    .wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                                if (*state).wbits == 0 as ::core::ffi::c_uint {
                                                                                                                    (*state).wbits = len;
                                                                                                                }
                                                                                                                if len > 15 as ::core::ffi::c_uint || len > (*state).wbits {
                                                                                                                    crate::zlib_h::set_stream_message(&mut *strm, c"invalid window size");
                                                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    (*state).dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    (*state).flags = 0 as ::core::ffi::c_int;
                                                                                                                    (*state).check = crate::src::adler32::ADLER32_INITIAL
                                                                                                                        as ::core::ffi::c_ulong;
                                                                                                                    (*strm).adler = (*state).check as crate::stdlib::uLong;
                                                                                                                    (*state).mode = (if hold & 0x200 as ::core::ffi::c_ulong
                                                                                                                        != 0
                                                                                                                    {
                                                                                                                        crate::src::inflate::DICTID as ::core::ffi::c_int
                                                                                                                    } else {
                                                                                                                        crate::src::inflate::TYPE as ::core::ffi::c_int
                                                                                                                    }) as crate::src::inflate::inflate_mode;
                                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                                    continue '_inf_leave;
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                16181 => {
                                                                                                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh1 = next;
                                                                                                        next = next.offset(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (*c2rust_fresh1 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    (*state).flags = hold as ::core::ffi::c_int;
                                                                                                    if (*state).flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        crate::zlib_h::set_stream_message(&mut *strm, c"unknown compression method");
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if (*state).flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        crate::zlib_h::set_stream_message(&mut *strm, c"unknown header flags set");
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        if let Some(head) = (*state).head.clone() {
                                                                                                            head.borrow_mut().text = (hold >> 8 as ::core::ffi::c_int
                                                                                                                & 1 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
                                                                                                        }
                                                                                                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                                                                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                        {
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            (*state).check = crate::src::crc32::crc32(
                                                                                                                (*state).check as crate::stdlib::uLong,
                                                                                                                &hbuf[..2],
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                        }
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        (*state).mode = crate::src::inflate::TIME;
                                                                                                        break 's_425;
                                                                                                    }
                                                                                                }
                                                                                                16182 => {
                                                                                                    break 's_425;
                                                                                                }
                                                                                                16183 => {
                                                                                                    break 's_519;
                                                                                                }
                                                                                                16184 => {
                                                                                                    break 'c_2317;
                                                                                                }
                                                                                                16185 => {
                                                                                                    break 'c_2319;
                                                                                                }
                                                                                                16186 => {
                                                                                                    break 'c_2322;
                                                                                                }
                                                                                                16187 => {
                                                                                                    break 'c_2325;
                                                                                                }
                                                                                                16188 => {
                                                                                                    break 'c_2327;
                                                                                                }
                                                                                                16189 => {
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh10 = next;
                                                                                                        next = next.offset(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (*c2rust_fresh10 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    (*state).check = (hold >> 24 as ::core::ffi::c_int
                                                                                                        & 0xff as ::core::ffi::c_ulong)
                                                                                                        .wrapping_add(
                                                                                                            hold >> 8 as ::core::ffi::c_int
                                                                                                                & 0xff00 as ::core::ffi::c_ulong,
                                                                                                        )
                                                                                                        .wrapping_add(
                                                                                                            (hold & 0xff00 as ::core::ffi::c_ulong)
                                                                                                                << 8 as ::core::ffi::c_int,
                                                                                                        )
                                                                                                        .wrapping_add(
                                                                                                            (hold & 0xff as ::core::ffi::c_ulong)
                                                                                                                << 24 as ::core::ffi::c_int,
                                                                                                        );
                                                                                                    (*strm).adler = (*state).check as crate::stdlib::uLong;
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                    (*state).mode = crate::src::inflate::DICT;
                                                                                                    break 'c_2336;
                                                                                                }
                                                                                                16190 => {
                                                                                                    break 'c_2336;
                                                                                                }
                                                                                                16191 => {
                                                                                                    break 'c_2339;
                                                                                                }
                                                                                                16192 => {
                                                                                                    break 'c_2340;
                                                                                                }
                                                                                                16193 => {
                                                                                                    hold >>= bits & 7 as ::core::ffi::c_uint;
                                                                                                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh12 = next;
                                                                                                        next = next.offset(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (*c2rust_fresh12 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    if hold & 0xffff as ::core::ffi::c_ulong
                                                                                                        != hold >> 16 as ::core::ffi::c_int
                                                                                                            ^ 0xffff as ::core::ffi::c_ulong
                                                                                                    {
                                                                                                        crate::zlib_h::set_stream_message(&mut *strm, c"invalid stored block lengths");
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        (*state).length = hold as ::core::ffi::c_uint
                                                                                                            & 0xffff as ::core::ffi::c_uint;
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        (*state).mode = crate::src::inflate::COPY_;
                                                                                                        if flush == crate::zlib_h::Z_TREES {
                                                                                                            break '_inf_leave;
                                                                                                        } else {
                                                                                                            break 'c_2355;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                16194 => {
                                                                                                    break 'c_2355;
                                                                                                }
                                                                                                16195 => {
                                                                                                    break 'c_2356;
                                                                                                }
                                                                                                16196 => {
                                                                                                    while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh13 = next;
                                                                                                        next = next.offset(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (*c2rust_fresh13 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    (*state).nlen = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(257 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    (*state).ndist = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(1 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    (*state).ncode = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(4 as ::core::ffi::c_uint);
                                                                                                    hold >>= 4 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            4 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    if (*state).nlen > 286 as ::core::ffi::c_uint
                                                                                                        || (*state).ndist > 30 as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        crate::zlib_h::set_stream_message(&mut *strm, c"too many length or distance symbols");
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        (*state).have = 0 as ::core::ffi::c_uint;
                                                                                                        (*state).mode = crate::src::inflate::LENLENS;
                                                                                                        break 's_1582;
                                                                                                    }
                                                                                                }
                                                                                                16197 => {
                                                                                                    break 's_1582;
                                                                                                }
                                                                                                16198 => {
                                                                                                    break 's_1689;
                                                                                                }
                                                                                                16199 => {
                                                                                                    break 'c_2397;
                                                                                                }
                                                                                                16200 => {
                                                                                                    break 'c_2398;
                                                                                                }
                                                                                                16201 => {
                                                                                                    break 'c_2410;
                                                                                                }
                                                                                                16202 => {
                                                                                                    break 's_2462;
                                                                                                }
                                                                                                16203 => {
                                                                                                    break 'c_2422;
                                                                                                }
                                                                                                16204 => {
                                                                                                    break 'c_2425;
                                                                                                }
                                                                                                16205 => {
                                                                                                    if left == 0 as ::core::ffi::c_uint {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                    let c2rust_fresh32 = put;
                                                                                                    put = put.offset(1);
                                                                                                    *c2rust_fresh32 = (*state).length as ::core::ffi::c_uchar;
                                                                                                    left = left.wrapping_sub(1);
                                                                                                    (*state).mode = crate::src::inflate::LEN;
                                                                                                    continue '_inf_leave;
                                                                                                }
                                                                                                16206 => {
                                                                                                    if (*state).wrap != 0 {
                                                                                                        while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if have == 0 as ::core::ffi::c_uint {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            have = have.wrapping_sub(1);
                                                                                                            let c2rust_fresh33 = next;
                                                                                                            next = next.offset(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (*c2rust_fresh33 as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        out = out.wrapping_sub(left);
                                                                                                        (*strm).total_out = (*strm)
                                                                                                            .total_out
                                                                                                            .wrapping_add(out as crate::stdlib::uLong);
                                                                                                        (*state).total = (*state)
                                                                                                            .total
                                                                                                            .wrapping_add(out as ::core::ffi::c_ulong);
                                                                                                        if (*state).wrap & 4 as ::core::ffi::c_int != 0 && out != 0
                                                                                                        {
                                                                                                            (*state).check = (if (*state).flags != 0 {
                                                                                                                crate::src::crc32::crc32(
                                                                                                                    (*state).check as crate::stdlib::uLong,
                                                                                                                    ::core::slice::from_raw_parts(
                                                                                                                        put.offset(-(out as isize)),
                                                                                                                        out as usize,
                                                                                                                    ),
                                                                                                                )
                                                                                                            } else {
                                                                                                                crate::src::adler32::adler32(
                                                                                                                    (*state).check as crate::stdlib::uLong,
                                                                                                                    ::core::slice::from_raw_parts(
                                                                                                                        put.offset(-(out as isize)),
                                                                                                                        out as usize,
                                                                                                                    ),
                                                                                                                )
                                                                                                            }) as ::core::ffi::c_ulong;
                                                                                                            (*strm).adler = (*state).check as crate::stdlib::uLong;
                                                                                                        }
                                                                                                        out = left;
                                                                                                        if (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                            && (if (*state).flags != 0 {
                                                                                                                hold
                                                                                                            } else {
                                                                                                                (hold >> 24 as ::core::ffi::c_int
                                                                                                                    & 0xff as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(
                                                                                                                        hold >> 8 as ::core::ffi::c_int
                                                                                                                            & 0xff00 as ::core::ffi::c_ulong,
                                                                                                                    )
                                                                                                                    .wrapping_add(
                                                                                                                        (hold & 0xff00 as ::core::ffi::c_ulong)
                                                                                                                            << 8 as ::core::ffi::c_int,
                                                                                                                    )
                                                                                                                    .wrapping_add(
                                                                                                                        (hold & 0xff as ::core::ffi::c_ulong)
                                                                                                                            << 24 as ::core::ffi::c_int,
                                                                                                                    )
                                                                                                            }) != (*state).check
                                                                                                        {
                                                                                                            crate::zlib_h::set_stream_message(&mut *strm, c"incorrect data check");
                                                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                        }
                                                                                                    }
                                                                                                    (*state).mode = crate::src::inflate::LENGTH;
                                                                                                }
                                                                                                16207 => {}
                                                                                                16208 => {
                                                                                                    break 'c_2443;
                                                                                                }
                                                                                                16209 => {
                                                                                                    ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                16210 => return crate::zlib_h::Z_MEM_ERROR,
                                                                                                16211 | _ => return crate::zlib_h::Z_STREAM_ERROR,
                                                                                            }
                                                                                            if (*state).wrap != 0 && (*state).flags != 0 {
                                                                                                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                {
                                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                    have = have.wrapping_sub(1);
                                                                                                    let c2rust_fresh34 = next;
                                                                                                    next = next.offset(1);
                                                                                                    hold = hold
                                                                                                        .wrapping_add(
                                                                                                            (*c2rust_fresh34 as ::core::ffi::c_ulong) << bits,
                                                                                                        );
                                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                }
                                                                                                if (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                    && hold
                                                                                                        != (*state).total & 0xffffffff as ::core::ffi::c_ulong
                                                                                                {
                                                                                                    crate::zlib_h::set_stream_message(&mut *strm, c"incorrect length check");
                                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                                    continue '_inf_leave;
                                                                                                } else {
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                }
                                                                                            }
                                                                                            (*state).mode = crate::src::inflate::DONE;
                                                                                            break 'c_2443;
                                                                                        }
                                                                                        while (*state).have < (*state).ncode {
                                                                                            while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                            {
                                                                                                if have == 0 as ::core::ffi::c_uint {
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                have = have.wrapping_sub(1);
                                                                                                let c2rust_fresh14 = next;
                                                                                                next = next.offset(1);
                                                                                                hold = hold
                                                                                                    .wrapping_add(
                                                                                                        (*c2rust_fresh14 as ::core::ffi::c_ulong) << bits,
                                                                                                    );
                                                                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                            }
                                                                                            let c2rust_fresh15 = (*state).have;
                                                                                            (*state).have = (*state).have.wrapping_add(1);
                                                                                            (*state).lens[order[c2rust_fresh15 as usize] as usize] = (hold
                                                                                                as ::core::ffi::c_uint
                                                                                                & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                as ::core::ffi::c_ushort;
                                                                                            hold >>= 3 as ::core::ffi::c_int;
                                                                                            bits = bits
                                                                                                .wrapping_sub(
                                                                                                    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                );
                                                                                        }
                                                                                        while (*state).have < 19 as ::core::ffi::c_uint {
                                                                                            let c2rust_fresh16 = (*state).have;
                                                                                            (*state).have = (*state).have.wrapping_add(1);
                                                                                            (*state).lens[order[c2rust_fresh16 as usize] as usize] = 0
                                                                                                as ::core::ffi::c_ushort;
                                                                                        }
                                                                                        (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                                                                                        (*state).distcode = CodeTable::Dynamic(0);
                                                                                        (*state).lencode = CodeTable::Dynamic(0);
                                                                                        (*state).lenbits = 7 as ::core::ffi::c_uint;
                                                                                        ret = crate::src::inftrees::inflate_table(
                                                                                            crate::src::inftrees::CODES,
                                                                                            &raw mut (*state).lens as *mut ::core::ffi::c_ushort,
                                                                                            19 as ::core::ffi::c_uint,

                                                                                            &raw mut (*state).next as *mut _ as *mut *mut crate::src::inftrees::code,
                                                                                            &raw mut (*state).lenbits,
                                                                                            &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                                                                        );
                                                                                        if ret != 0
                                                                                        {
                                                                                            crate::zlib_h::set_stream_message(&mut *strm, c"invalid code lengths set");
                                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                                            continue '_inf_leave;
                                                                                        } else {
                                                                                            (*state).have = 0 as ::core::ffi::c_uint;
                                                                                            (*state).mode = crate::src::inflate::CODELENS;
                                                                                            break 's_1689;
                                                                                        }
                                                                                    }
                                                                                    if (*state).havedict == 0 as ::core::ffi::c_int {
                                                                                        (*strm).next_out = crate::output_cursor!(put);
                                                                                        (*strm).avail_out = left as crate::stdlib::uInt;
                                                                                        (*strm).next_in = crate::input_cursor!(next);
                                                                                        (*strm).avail_in = have as crate::stdlib::uInt;
                                                                                        (*state).hold = hold;
                                                                                        (*state).bits = bits;
                                                                                        return crate::zlib_h::Z_NEED_DICT;
                                                                                    }
                                                                                    (*state).check = crate::src::adler32::ADLER32_INITIAL
                                                                                        as ::core::ffi::c_ulong;
                                                                                    (*strm).adler =
                                                                                        (*state)
                                                                                            .check
                                                                                            as crate::stdlib::uLong;
                                                                                    (*state).mode =
                                                                                        crate::src::inflate::TYPE;
                                                                                    break 'c_2339;
                                                                                }
                                                                                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                {
                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                        break '_inf_leave;
                                                                                    }
                                                                                    have = have.wrapping_sub(1);
                                                                                    let c2rust_fresh2 = next;
                                                                                    next = next.offset(1);
                                                                                    hold = hold
                                                                                        .wrapping_add(
                                                                                            (*c2rust_fresh2 as ::core::ffi::c_ulong) << bits,
                                                                                        );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                if let Some(head) =
                                                                                    (*state)
                                                                                        .head
                                                                                        .clone()
                                                                                {
                                                                                    head.borrow_mut().time = hold as crate::stdlib::uLong;
                                                                                }
                                                                                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                                                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                {
                                                                                    hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                    hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[2 as usize] = (hold >> 16 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[3 as usize] = (hold >> 24 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    (*state).check = crate::src::crc32::crc32(
                                                                                        (*state).check as crate::stdlib::uLong,
                                                                                        &hbuf[..4],
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                }
                                                                                hold = 0 as ::core::ffi::c_ulong;
                                                                                bits = 0 as ::core::ffi::c_uint;
                                                                                (*state).mode = crate::src::inflate::OS;
                                                                                break 's_519;
                                                                            }
                                                                            (*state).mode = crate::src::inflate::COPY_1;
                                                                            break 'c_2356;
                                                                        }
                                                                        while (*state).have
                                                                            < (*state)
                                                                                .nlen
                                                                                .wrapping_add(
                                                                                    (*state).ndist,
                                                                                )
                                                                        {
                                                                            loop {
                                                                                here = (*state).lencode_at(
                                                                                    (hold as ::core::ffi::c_uint
                                                                                        & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint)) as usize,
                                                                                );
                                                                                if here.bits as ::core::ffi::c_uint <= bits {
                                                                                    break;
                                                                                }
                                                                                if have == 0 as ::core::ffi::c_uint {
                                                                                    break '_inf_leave;
                                                                                }
                                                                                have = have
                                                                                    .wrapping_sub(
                                                                                        1,
                                                                                    );
                                                                                let c2rust_fresh17 =
                                                                                    next;
                                                                                next =
                                                                                    next.offset(1);
                                                                                hold = hold
                                                                                    .wrapping_add(
                                                                                        (*c2rust_fresh17 as ::core::ffi::c_ulong) << bits,
                                                                                    );
                                                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                            }
                                                                            if (here.val as ::core::ffi::c_int)
                                                                                < 16 as ::core::ffi::c_int
                                                                            {
                                                                                hold >>= here.bits as ::core::ffi::c_int;
                                                                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                let c2rust_fresh18 = (*state).have;
                                                                                (*state).have = (*state).have.wrapping_add(1);
                                                                                (*state).lens[c2rust_fresh18 as usize] = here.val;
                                                                            } else {
                                                                                if here.val as ::core::ffi::c_int
                                                                                    == 16 as ::core::ffi::c_int
                                                                                {
                                                                                    while bits
                                                                                        < (here.bits as ::core::ffi::c_int
                                                                                            + 2 as ::core::ffi::c_int) as ::core::ffi::c_uint
                                                                                    {
                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                            break '_inf_leave;
                                                                                        }
                                                                                        have = have.wrapping_sub(1);
                                                                                        let c2rust_fresh19 = next;
                                                                                        next = next.offset(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (*c2rust_fresh19 as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    if (*state).have == 0 as ::core::ffi::c_uint {
                                                                                        crate::zlib_h::set_stream_message(&mut *strm, c"invalid bit length repeat");
                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                        break;
                                                                                    } else {
                                                                                        len = (*state)
                                                                                            .lens[(*state).have.wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                            as usize] as ::core::ffi::c_uint;
                                                                                        copy = (3 as ::core::ffi::c_uint)
                                                                                            .wrapping_add(
                                                                                                hold as ::core::ffi::c_uint
                                                                                                    & ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint),
                                                                                            );
                                                                                        hold >>= 2 as ::core::ffi::c_int;
                                                                                        bits = bits
                                                                                            .wrapping_sub(
                                                                                                2 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                            );
                                                                                    }
                                                                                } else if here.val as ::core::ffi::c_int
                                                                                    == 17 as ::core::ffi::c_int
                                                                                {
                                                                                    while bits
                                                                                        < (here.bits as ::core::ffi::c_int
                                                                                            + 3 as ::core::ffi::c_int) as ::core::ffi::c_uint
                                                                                    {
                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                            break '_inf_leave;
                                                                                        }
                                                                                        have = have.wrapping_sub(1);
                                                                                        let c2rust_fresh20 = next;
                                                                                        next = next.offset(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (*c2rust_fresh20 as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    len = 0 as ::core::ffi::c_uint;
                                                                                    copy = (3 as ::core::ffi::c_uint)
                                                                                        .wrapping_add(
                                                                                            hold as ::core::ffi::c_uint
                                                                                                & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                                                                        );
                                                                                    hold >>= 3 as ::core::ffi::c_int;
                                                                                    bits = bits
                                                                                        .wrapping_sub(
                                                                                            3 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                        );
                                                                                } else {
                                                                                    while bits
                                                                                        < (here.bits as ::core::ffi::c_int
                                                                                            + 7 as ::core::ffi::c_int) as ::core::ffi::c_uint
                                                                                    {
                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                            break '_inf_leave;
                                                                                        }
                                                                                        have = have.wrapping_sub(1);
                                                                                        let c2rust_fresh21 = next;
                                                                                        next = next.offset(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (*c2rust_fresh21 as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    len = 0 as ::core::ffi::c_uint;
                                                                                    copy = (11 as ::core::ffi::c_uint)
                                                                                        .wrapping_add(
                                                                                            hold as ::core::ffi::c_uint
                                                                                                & ((1 as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int)
                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                                                                        );
                                                                                    hold >>= 7 as ::core::ffi::c_int;
                                                                                    bits = bits
                                                                                        .wrapping_sub(
                                                                                            7 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                        );
                                                                                }
                                                                                if (*state).have.wrapping_add(copy)
                                                                                    > (*state).nlen.wrapping_add((*state).ndist)
                                                                                {
                                                                                    crate::zlib_h::set_stream_message(&mut *strm, c"invalid bit length repeat");
                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                    break;
                                                                                } else {
                                                                                    loop {
                                                                                        let c2rust_fresh22 = copy;
                                                                                        copy = copy.wrapping_sub(1);
                                                                                        if c2rust_fresh22 == 0 {
                                                                                            break;
                                                                                        }
                                                                                        let c2rust_fresh23 = (*state).have;
                                                                                        (*state).have = (*state).have.wrapping_add(1);
                                                                                        (*state).lens[c2rust_fresh23 as usize] = len
                                                                                            as ::core::ffi::c_ushort;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        if (*state).mode as ::core::ffi::c_uint
                                                                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                        {
                                                                            continue '_inf_leave;
                                                                        }
                                                                        if (*state).lens[256 as usize] as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            crate::zlib_h::set_stream_message(&mut *strm, c"invalid code -- missing end-of-block");
                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                                                                            (*state).lencode = CodeTable::Dynamic(0);
                                                                            (*state).lenbits = 9 as ::core::ffi::c_uint;
                                                                            ret = crate::src::inftrees::inflate_table(
                                                                                crate::src::inftrees::LENS,
                                                                                &raw mut (*state).lens as *mut ::core::ffi::c_ushort,
                                                                                (*state).nlen,

                                                                                &raw mut (*state).next as *mut _ as *mut *mut crate::src::inftrees::code,
                                                                                &raw mut (*state).lenbits,
                                                                                &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                                                            );
                                                                            if ret != 0 {
                                                                                crate::zlib_h::set_stream_message(&mut *strm, c"invalid literal/lengths set");
                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                (*state).distcode = CodeTable::Dynamic(
                                                                                    (*state)
                                                                                        .next
                                                                                        .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
                                                                                        as usize,
                                                                                );
                                                                                (*state).distbits = 6 as ::core::ffi::c_uint;
                                                                                ret = crate::src::inftrees::inflate_table(
                                                                                    crate::src::inftrees::DISTS,
                                                                                    (&raw mut (*state).lens as *mut ::core::ffi::c_ushort)
                                                                                        .offset((*state).nlen as isize),
                                                                                    (*state).ndist,

                                                                                    &raw mut (*state).next as *mut _ as *mut *mut crate::src::inftrees::code,
                                                                                    &raw mut (*state).distbits,
                                                                                    &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                                                                );
                                                                                if ret != 0 {
                                                                                    crate::zlib_h::set_stream_message(&mut *strm, c"invalid distances set");
                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                    continue '_inf_leave;
                                                                                } else {
                                                                                    (*state).mode = crate::src::inflate::LEN_;
                                                                                    if flush == crate::zlib_h::Z_TREES {
                                                                                        break '_inf_leave;
                                                                                    } else {
                                                                                        break 'c_2397;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    copy = (*state).length;
                                                                    if copy != 0 {
                                                                        if copy > have {
                                                                            copy = have;
                                                                        }
                                                                        if copy > left {
                                                                            copy = left;
                                                                        }
                                                                        if copy == 0
                                                                            as ::core::ffi::c_uint
                                                                        {
                                                                            break '_inf_leave;
                                                                        }
                                                                        crate::stdlib::memcpy(
                                                                            put as *mut ::core::ffi::c_void,
                                                                            next as *const ::core::ffi::c_void,
                                                                            copy as crate::__stddef_size_t_h::size_t,
                                                                        );
                                                                        have =
                                                                            have.wrapping_sub(copy);
                                                                        next = next
                                                                            .offset(copy as isize);
                                                                        left =
                                                                            left.wrapping_sub(copy);
                                                                        put = put
                                                                            .offset(copy as isize);
                                                                        (*state).length = (*state)
                                                                            .length
                                                                            .wrapping_sub(copy);
                                                                        continue '_inf_leave;
                                                                    } else {
                                                                        (*state).mode = crate::src::inflate::TYPE;
                                                                        continue '_inf_leave;
                                                                    }
                                                                }
                                                                while bits
                                                                    < 16 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_uint
                                                                {
                                                                    if have
                                                                        == 0 as ::core::ffi::c_uint
                                                                    {
                                                                        break '_inf_leave;
                                                                    }
                                                                    have = have.wrapping_sub(1);
                                                                    let c2rust_fresh3 = next;
                                                                    next = next.offset(1);
                                                                    hold = hold.wrapping_add(
                                                                        (*c2rust_fresh3
                                                                            as ::core::ffi::c_ulong)
                                                                            << bits,
                                                                    );
                                                                    bits = bits.wrapping_add(
                                                                        8 as ::core::ffi::c_uint,
                                                                    );
                                                                }
                                                                if let Some(head) =
                                                                    (*state).head.clone()
                                                                {
                                                                    let mut head =
                                                                        head.borrow_mut();
                                                                    head.xflags = (hold
                                                                        & 0xff
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int;
                                                                    head.os = (hold
                                                                        >> 8 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_int;
                                                                }
                                                                if (*state).flags
                                                                    & 0x200 as ::core::ffi::c_int
                                                                    != 0
                                                                    && (*state).wrap
                                                                        & 4 as ::core::ffi::c_int
                                                                        != 0
                                                                {
                                                                    hbuf[0 as usize] = hold
                                                                        as ::core::ffi::c_uchar;
                                                                    hbuf[1 as usize] = (hold
                                                                        >> 8 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_uchar;
                                                                    (*state).check = crate::src::crc32::crc32(
                                                                        (*state).check as crate::stdlib::uLong,
                                                                        &hbuf[..2],
                                                                    ) as ::core::ffi::c_ulong;
                                                                }
                                                                hold = 0 as ::core::ffi::c_ulong;
                                                                bits = 0 as ::core::ffi::c_uint;
                                                                (*state).mode =
                                                                    crate::src::inflate::EXLEN;
                                                                break 'c_2317;
                                                            }
                                                            if flush == crate::zlib_h::Z_BLOCK
                                                                || flush == crate::zlib_h::Z_TREES
                                                            {
                                                                break '_inf_leave;
                                                            } else {
                                                                break 'c_2340;
                                                            }
                                                        }
                                                        ret = crate::zlib_h::Z_STREAM_END;
                                                        break '_inf_leave;
                                                    }
                                                    if (*state).last != 0 {
                                                        hold >>= bits & 7 as ::core::ffi::c_uint;
                                                        bits = bits.wrapping_sub(
                                                            bits & 7 as ::core::ffi::c_uint,
                                                        );
                                                        (*state).mode = crate::src::inflate::CHECK;
                                                        continue '_inf_leave;
                                                    } else {
                                                        while bits
                                                            < 3 as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint
                                                        {
                                                            if have == 0 as ::core::ffi::c_uint {
                                                                break '_inf_leave;
                                                            }
                                                            have = have.wrapping_sub(1);
                                                            let c2rust_fresh11 = next;
                                                            next = next.offset(1);
                                                            hold = hold.wrapping_add(
                                                                (*c2rust_fresh11
                                                                    as ::core::ffi::c_ulong)
                                                                    << bits,
                                                            );
                                                            bits = bits.wrapping_add(
                                                                8 as ::core::ffi::c_uint,
                                                            );
                                                        }
                                                        (*state).last = (hold
                                                            as ::core::ffi::c_uint
                                                            & ((1 as ::core::ffi::c_uint)
                                                                << 1 as ::core::ffi::c_int)
                                                                .wrapping_sub(
                                                                    1 as ::core::ffi::c_uint,
                                                                ))
                                                            as ::core::ffi::c_int;
                                                        hold >>= 1 as ::core::ffi::c_int;
                                                        bits = bits.wrapping_sub(
                                                            1 as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint,
                                                        );
                                                        match hold as ::core::ffi::c_uint
                                                            & ((1 as ::core::ffi::c_uint)
                                                                << 2 as ::core::ffi::c_int)
                                                                .wrapping_sub(
                                                                    1 as ::core::ffi::c_uint,
                                                                ) {
                                                            0 => {
                                                                (*state).mode =
                                                                    crate::src::inflate::STORED;
                                                            }
                                                            1 => {
                                                                crate::src::inftrees::inflate_fixed(
                                                                    &mut *state,
                                                                );
                                                                (*state).mode =
                                                                    crate::src::inflate::LEN_;
                                                                if flush == crate::zlib_h::Z_TREES {
                                                                    hold >>=
                                                                        2 as ::core::ffi::c_int;
                                                                    bits = bits.wrapping_sub(
                                                                        2 as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint,
                                                                    );
                                                                    break '_inf_leave;
                                                                }
                                                            }
                                                            2 => {
                                                                (*state).mode =
                                                                    crate::src::inflate::TABLE;
                                                            }
                                                            _ => {
                                                                crate::zlib_h::set_stream_message(
                                                                    &mut *strm,
                                                                    c"invalid block type",
                                                                );
                                                                (*state).mode =
                                                                    crate::src::inflate::BAD;
                                                            }
                                                        }
                                                        hold >>= 2 as ::core::ffi::c_int;
                                                        bits = bits.wrapping_sub(
                                                            2 as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint,
                                                        );
                                                        continue '_inf_leave;
                                                    }
                                                }
                                                if (*state).flags & 0x400 as ::core::ffi::c_int != 0
                                                {
                                                    while bits
                                                        < 16 as ::core::ffi::c_int
                                                            as ::core::ffi::c_uint
                                                    {
                                                        if have == 0 as ::core::ffi::c_uint {
                                                            break '_inf_leave;
                                                        }
                                                        have = have.wrapping_sub(1);
                                                        let c2rust_fresh4 = next;
                                                        next = next.offset(1);
                                                        hold = hold.wrapping_add(
                                                            (*c2rust_fresh4
                                                                as ::core::ffi::c_ulong)
                                                                << bits,
                                                        );
                                                        bits = bits
                                                            .wrapping_add(8 as ::core::ffi::c_uint);
                                                    }
                                                    (*state).length = hold as ::core::ffi::c_uint;
                                                    if let Some(head) = (*state).head.clone() {
                                                        head.borrow_mut().extra_len = hold
                                                            as ::core::ffi::c_uint
                                                            as crate::stdlib::uInt;
                                                    }
                                                    if (*state).flags & 0x200 as ::core::ffi::c_int
                                                        != 0
                                                        && (*state).wrap & 4 as ::core::ffi::c_int
                                                            != 0
                                                    {
                                                        hbuf[0 as usize] =
                                                            hold as ::core::ffi::c_uchar;
                                                        hbuf[1 as usize] = (hold
                                                            >> 8 as ::core::ffi::c_int)
                                                            as ::core::ffi::c_uchar;
                                                        (*state).check = crate::src::crc32::crc32(
                                                            (*state).check as crate::stdlib::uLong,
                                                            &hbuf[..2],
                                                        )
                                                            as ::core::ffi::c_ulong;
                                                    }
                                                    hold = 0 as ::core::ffi::c_ulong;
                                                    bits = 0 as ::core::ffi::c_uint;
                                                } else if let Some(head) = (*state).head.clone() {
                                                    head.borrow_mut().extra = None;
                                                }
                                                (*state).mode = crate::src::inflate::EXTRA;
                                                break 'c_2319;
                                            }
                                            (*state).mode = crate::src::inflate::LEN;
                                        }
                                        if have >= 6 as ::core::ffi::c_uint
                                            && left >= 258 as ::core::ffi::c_uint
                                        {
                                            (*strm).next_out = crate::output_cursor!(put);
                                            (*strm).avail_out = left as crate::stdlib::uInt;
                                            (*strm).next_in = crate::input_cursor!(next);
                                            (*strm).avail_in = have as crate::stdlib::uInt;
                                            (*state).hold = hold;
                                            (*state).bits = bits;
                                            crate::src::inffast::inflate_fast(
                                                strm as *mut crate::zlib_h::z_stream_s,
                                                &mut *state,
                                                out,
                                            );
                                            put = crate::output_pointer!((*strm).next_out)
                                                as *mut ::core::ffi::c_uchar;
                                            left = (*strm).avail_out as ::core::ffi::c_uint;
                                            let input_cursor = (*strm).next_in;
                                            next = match input_cursor.0 {
                                                Some(address) => {
                                                    ::core::ptr::with_exposed_provenance_mut(
                                                        address.get(),
                                                    )
                                                }
                                                None => ::core::ptr::null_mut(),
                                            }
                                                as *mut ::core::ffi::c_uchar;
                                            have = (*strm).avail_in as ::core::ffi::c_uint;
                                            hold = (*state).hold;
                                            bits = (*state).bits;
                                            if (*state).mode as ::core::ffi::c_uint
                                                == crate::src::inflate::TYPE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                (*state).back = -1 as ::core::ffi::c_int;
                                            }
                                            continue '_inf_leave;
                                        } else {
                                            (*state).back = 0 as ::core::ffi::c_int;
                                            loop {
                                                here = (*state).lencode_at(
                                                    (hold as ::core::ffi::c_uint
                                                        & ((1 as ::core::ffi::c_uint)
                                                            << (*state).lenbits)
                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                        as usize,
                                                );
                                                if here.bits as ::core::ffi::c_uint <= bits {
                                                    break;
                                                }
                                                if have == 0 as ::core::ffi::c_uint {
                                                    break '_inf_leave;
                                                }
                                                have = have.wrapping_sub(1);
                                                let c2rust_fresh24 = next;
                                                next = next.offset(1);
                                                hold = hold.wrapping_add(
                                                    (*c2rust_fresh24 as ::core::ffi::c_ulong)
                                                        << bits,
                                                );
                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                            }
                                            if here.op as ::core::ffi::c_int != 0
                                                && here.op as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                            {
                                                last = here;
                                                loop {
                                                    here = (*state).lencode_at(
                                                        (last.val as ::core::ffi::c_uint)
                                                            .wrapping_add(
                                                            (hold as ::core::ffi::c_uint
                                                                & ((1 as ::core::ffi::c_uint)
                                                                    << last.bits
                                                                        as ::core::ffi::c_int
                                                                        + last.op
                                                                            as ::core::ffi::c_int)
                                                                    .wrapping_sub(
                                                                        1 as ::core::ffi::c_uint,
                                                                    ))
                                                                >> last.bits as ::core::ffi::c_int,
                                                        )
                                                            as usize,
                                                    );
                                                    if (last.bits as ::core::ffi::c_int
                                                        + here.bits as ::core::ffi::c_int)
                                                        as ::core::ffi::c_uint
                                                        <= bits
                                                    {
                                                        break;
                                                    }
                                                    if have == 0 as ::core::ffi::c_uint {
                                                        break '_inf_leave;
                                                    }
                                                    have = have.wrapping_sub(1);
                                                    let c2rust_fresh25 = next;
                                                    next = next.offset(1);
                                                    hold = hold.wrapping_add(
                                                        (*c2rust_fresh25 as ::core::ffi::c_ulong)
                                                            << bits,
                                                    );
                                                    bits =
                                                        bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                }
                                                hold >>= last.bits as ::core::ffi::c_int;
                                                bits = bits
                                                    .wrapping_sub(last.bits as ::core::ffi::c_uint);
                                                (*state).back += last.bits as ::core::ffi::c_int;
                                            }
                                            hold >>= here.bits as ::core::ffi::c_int;
                                            bits =
                                                bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                            (*state).back += here.bits as ::core::ffi::c_int;
                                            (*state).length = here.val as ::core::ffi::c_uint;
                                            if here.op as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                (*state).mode = crate::src::inflate::LIT;
                                                continue '_inf_leave;
                                            } else if here.op as ::core::ffi::c_int
                                                & 32 as ::core::ffi::c_int
                                                != 0
                                            {
                                                (*state).back = -1 as ::core::ffi::c_int;
                                                (*state).mode = crate::src::inflate::TYPE;
                                                continue '_inf_leave;
                                            } else if here.op as ::core::ffi::c_int
                                                & 64 as ::core::ffi::c_int
                                                != 0
                                            {
                                                crate::zlib_h::set_stream_message(
                                                    &mut *strm,
                                                    c"invalid literal/length code",
                                                );
                                                (*state).mode = crate::src::inflate::BAD;
                                                continue '_inf_leave;
                                            } else {
                                                (*state).extra = here.op as ::core::ffi::c_uint
                                                    & 15 as ::core::ffi::c_uint;
                                                (*state).mode = crate::src::inflate::LENEXT;
                                                break 'c_2410;
                                            }
                                        }
                                    }
                                    if (*state).flags & 0x400 as ::core::ffi::c_int != 0 {
                                        copy = (*state).length;
                                        if copy > have {
                                            copy = have;
                                        }
                                        if copy != 0 {
                                            if let Some(head) = (*state).head.clone() {
                                                let mut head = head.borrow_mut();
                                                let extra_len = head.extra_len;
                                                let extra_max = head.extra_max;
                                                if let Some(extra) = head.extra.as_mut() {
                                                    len = (extra_len as ::core::ffi::c_uint)
                                                        .wrapping_sub((*state).length);
                                                    let max = (extra_max as usize).min(extra.len());
                                                    if (len as usize) < max {
                                                        let copied =
                                                            (copy as usize).min(max - len as usize);
                                                        extra[len as usize..len as usize + copied]
                                                            .copy_from_slice(
                                                                ::core::slice::from_raw_parts(
                                                                    next, copied,
                                                                ),
                                                            );
                                                    }
                                                }
                                            }
                                            if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                            {
                                                (*state).check = crate::src::crc32::crc32(
                                                    (*state).check as crate::stdlib::uLong,
                                                    ::core::slice::from_raw_parts(
                                                        next,
                                                        copy as usize,
                                                    ),
                                                )
                                                    as ::core::ffi::c_ulong;
                                            }
                                            have = have.wrapping_sub(copy);
                                            next = next.offset(copy as isize);
                                            (*state).length = (*state).length.wrapping_sub(copy);
                                        }
                                        if (*state).length != 0 {
                                            break '_inf_leave;
                                        }
                                    }
                                    (*state).length = 0 as ::core::ffi::c_uint;
                                    (*state).mode = crate::src::inflate::NAME;
                                    break 'c_2322;
                                }
                                if (*state).extra != 0 {
                                    while bits < (*state).extra {
                                        if have == 0 as ::core::ffi::c_uint {
                                            break '_inf_leave;
                                        }
                                        have = have.wrapping_sub(1);
                                        let c2rust_fresh26 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*c2rust_fresh26 as ::core::ffi::c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    (*state).length = (*state).length.wrapping_add(
                                        hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                                .wrapping_sub(1 as ::core::ffi::c_uint),
                                    );
                                    hold >>= (*state).extra;
                                    bits = bits.wrapping_sub((*state).extra);
                                    (*state).back = ((*state).back as ::core::ffi::c_uint)
                                        .wrapping_add((*state).extra)
                                        as ::core::ffi::c_int;
                                }
                                (*state).was = (*state).length;
                                (*state).mode = crate::src::inflate::DIST;
                                break 's_2462;
                            }
                            if (*state).flags & 0x800 as ::core::ffi::c_int != 0 {
                                if have == 0 as ::core::ffi::c_uint {
                                    break '_inf_leave;
                                }
                                copy = 0 as ::core::ffi::c_uint;
                                loop {
                                    let c2rust_fresh5 = copy;
                                    copy = copy.wrapping_add(1);
                                    len =
                                        *next.offset(c2rust_fresh5 as isize) as ::core::ffi::c_uint;
                                    if let Some(head) = (*state).head.clone() {
                                        let mut head = head.borrow_mut();
                                        let name_max = head.name_max;
                                        if let Some(name) = head.name.as_mut() {
                                            let max = (name_max as usize).min(name.len());
                                            if ((*state).length as usize) < max {
                                                let c2rust_fresh6 = (*state).length as usize;
                                                (*state).length = (*state).length.wrapping_add(1);
                                                name[c2rust_fresh6] = len as crate::stdlib::Bytef;
                                            }
                                        }
                                    }
                                    if !(len != 0 && copy < have) {
                                        break;
                                    }
                                }
                                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                {
                                    (*state).check = crate::src::crc32::crc32(
                                        (*state).check as crate::stdlib::uLong,
                                        ::core::slice::from_raw_parts(next, copy as usize),
                                    )
                                        as ::core::ffi::c_ulong;
                                }
                                have = have.wrapping_sub(copy);
                                next = next.offset(copy as isize);
                                if len != 0 {
                                    break '_inf_leave;
                                }
                            } else if let Some(head) = (*state).head.clone() {
                                head.borrow_mut().name = None;
                            }
                            (*state).length = 0 as ::core::ffi::c_uint;
                            (*state).mode = crate::src::inflate::COMMENT;
                            break 'c_2325;
                        }
                        loop {
                            here = (*state).distcode_at(
                                (hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint) << (*state).distbits)
                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                    as usize,
                            );
                            if here.bits as ::core::ffi::c_uint <= bits {
                                break;
                            }
                            if have == 0 as ::core::ffi::c_uint {
                                break '_inf_leave;
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh27 = next;
                            next = next.offset(1);
                            hold = hold
                                .wrapping_add((*c2rust_fresh27 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            last = here;
                            loop {
                                here = (*state).distcode_at(
                                    (last.val as ::core::ffi::c_uint).wrapping_add(
                                        (hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint)
                                                << last.bits as ::core::ffi::c_int
                                                    + last.op as ::core::ffi::c_int)
                                                .wrapping_sub(1 as ::core::ffi::c_uint))
                                            >> last.bits as ::core::ffi::c_int,
                                    ) as usize,
                                );
                                if (last.bits as ::core::ffi::c_int
                                    + here.bits as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint
                                    <= bits
                                {
                                    break;
                                }
                                if have == 0 as ::core::ffi::c_uint {
                                    break '_inf_leave;
                                }
                                have = have.wrapping_sub(1);
                                let c2rust_fresh28 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add(
                                    (*c2rust_fresh28 as ::core::ffi::c_ulong) << bits,
                                );
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            hold >>= last.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                            (*state).back += last.bits as ::core::ffi::c_int;
                        }
                        hold >>= here.bits as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                        (*state).back += here.bits as ::core::ffi::c_int;
                        if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                            crate::zlib_h::set_stream_message(&mut *strm, c"invalid distance code");
                            (*state).mode = crate::src::inflate::BAD;
                            continue '_inf_leave;
                        } else {
                            (*state).offset = here.val as ::core::ffi::c_uint;
                            (*state).extra =
                                here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                            (*state).mode = crate::src::inflate::DISTEXT;
                            break 'c_2422;
                        }
                    }
                    if (*state).flags & 0x1000 as ::core::ffi::c_int != 0 {
                        if have == 0 as ::core::ffi::c_uint {
                            break '_inf_leave;
                        }
                        copy = 0 as ::core::ffi::c_uint;
                        loop {
                            let c2rust_fresh7 = copy;
                            copy = copy.wrapping_add(1);
                            len = *next.offset(c2rust_fresh7 as isize) as ::core::ffi::c_uint;
                            if let Some(head) = (*state).head.clone() {
                                let mut head = head.borrow_mut();
                                let comm_max = head.comm_max;
                                if let Some(comment) = head.comment.as_mut() {
                                    let max = (comm_max as usize).min(comment.len());
                                    if ((*state).length as usize) < max {
                                        let c2rust_fresh8 = (*state).length as usize;
                                        (*state).length = (*state).length.wrapping_add(1);
                                        comment[c2rust_fresh8] = len as crate::stdlib::Bytef;
                                    }
                                }
                            }
                            if !(len != 0 && copy < have) {
                                break;
                            }
                        }
                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
                        {
                            (*state).check = crate::src::crc32::crc32(
                                (*state).check as crate::stdlib::uLong,
                                ::core::slice::from_raw_parts(next, copy as usize),
                            ) as ::core::ffi::c_ulong;
                        }
                        have = have.wrapping_sub(copy);
                        next = next.offset(copy as isize);
                        if len != 0 {
                            break '_inf_leave;
                        }
                    } else if let Some(head) = (*state).head.clone() {
                        head.borrow_mut().comment = None;
                    }
                    (*state).mode = crate::src::inflate::HCRC;
                    break 'c_2327;
                }
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as ::core::ffi::c_uint {
                            break '_inf_leave;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh29 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh29 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    (*state).offset = (*state).offset.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                    (*state).back = ((*state).back as ::core::ffi::c_uint)
                        .wrapping_add((*state).extra)
                        as ::core::ffi::c_int;
                }
                (*state).mode = crate::src::inflate::MATCH;
                break 'c_2425;
            }
            if (*state).flags & 0x200 as ::core::ffi::c_int != 0 {
                while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break '_inf_leave;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh9 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh9 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if (*state).wrap & 4 as ::core::ffi::c_int != 0
                    && hold != (*state).check & 0xffff as ::core::ffi::c_ulong
                {
                    crate::zlib_h::set_stream_message(&mut *strm, c"header crc mismatch");
                    (*state).mode = crate::src::inflate::BAD;
                    continue '_inf_leave;
                } else {
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                }
            }
            if let Some(head) = (*state).head.clone() {
                let mut head = head.borrow_mut();
                head.hcrc = (*state).flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
                head.done = 1 as ::core::ffi::c_int;
            }
            (*state).check =
                crate::src::crc32::crc32(0 as crate::stdlib::uLong, &[]) as ::core::ffi::c_ulong;
            (*strm).adler = (*state).check as crate::stdlib::uLong;
            (*state).mode = crate::src::inflate::TYPE;
            continue '_inf_leave;
        }
        if left == 0 as ::core::ffi::c_uint {
            break;
        }
        copy = out.wrapping_sub(left);
        if (*state).offset > copy {
            copy = (*state).offset.wrapping_sub(copy);
            if copy > (*state).whave {
                if (*state).sane != 0 {
                    crate::zlib_h::set_stream_message(&mut *strm, c"invalid distance too far back");
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                }
            }
            if copy > (*state).wnext {
                copy = copy.wrapping_sub((*state).wnext);
                window_index = Some((*state).wsize.wrapping_sub(copy) as usize);
            } else {
                window_index = Some((*state).wnext.wrapping_sub(copy) as usize);
            }
            if copy > (*state).length {
                copy = (*state).length;
            }
        } else {
            from = put.offset(-((*state).offset as isize));
            window_index = None;
            copy = (*state).length;
        }
        if copy > left {
            copy = left;
        }
        left = left.wrapping_sub(copy);
        (*state).length = (*state).length.wrapping_sub(copy);
        loop {
            let byte = if let Some(index) = window_index {
                let window = (*state)
                    .window
                    .as_ref()
                    .expect("a referenced history has a window");
                window_index = Some((index + 1) % (*state).wsize as usize);
                window[index]
            } else {
                let byte = *from;
                from = from.offset(1);
                byte
            };
            let c2rust_fresh31 = put;
            put = put.offset(1);
            *c2rust_fresh31 = byte;
            copy = copy.wrapping_sub(1);
            if copy == 0 {
                break;
            }
        }
        if (*state).length == 0 as ::core::ffi::c_uint {
            (*state).mode = crate::src::inflate::LEN;
        }
    }
    (*strm).next_out = crate::output_cursor!(put);
    (*strm).avail_out = left as crate::stdlib::uInt;
    (*strm).next_in = crate::input_cursor!(next);
    (*strm).avail_in = have as crate::stdlib::uInt;
    (*state).hold = hold;
    (*state).bits = bits;
    if (*state).wsize != 0
        || out != (*strm).avail_out
            && ((*state).mode as ::core::ffi::c_uint)
                < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
            && (((*state).mode as ::core::ffi::c_uint)
                < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                || flush != crate::zlib_h::Z_FINISH)
    {
        if updatewindow(
            strm,
            &mut *state,
            crate::output_pointer!((*strm).next_out),
            out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint),
        ) != 0
        {
            (*state).mode = crate::src::inflate::MEM;
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in as ::core::ffi::c_uint);
    out = out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
    (*strm).total_in = (*strm).total_in.wrapping_add(in_0 as crate::stdlib::uLong);
    (*strm).total_out = (*strm).total_out.wrapping_add(out as crate::stdlib::uLong);
    (*state).total = (*state).total.wrapping_add(out as ::core::ffi::c_ulong);
    if (*state).wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
        (*state).check = (if (*state).flags != 0 {
            crate::src::crc32::crc32(
                (*state).check as crate::stdlib::uLong,
                ::core::slice::from_raw_parts(
                    crate::output_pointer!((*strm).next_out).sub(out as usize),
                    out as usize,
                ),
            )
        } else {
            crate::src::adler32::adler32(
                (*state).check as crate::stdlib::uLong,
                ::core::slice::from_raw_parts(
                    crate::output_pointer!((*strm).next_out).sub(out as usize),
                    out as usize,
                ),
            )
        }) as ::core::ffi::c_ulong;
        (*strm).adler = (*state).check as crate::stdlib::uLong;
    }
    (*strm).data_type = (*state).bits as ::core::ffi::c_int
        + (if (*state).last != 0 {
            64 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
        + (if (*state).mode as ::core::ffi::c_uint
            == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            128 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
        + (if (*state).mode as ::core::ffi::c_uint
            == crate::src::inflate::LEN_ as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*state).mode as ::core::ffi::c_uint
                == crate::src::inflate::COPY_ as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            256 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
    if (in_0 == 0 as ::core::ffi::c_uint && out == 0 as ::core::ffi::c_uint
        || flush == crate::zlib_h::Z_FINISH)
        && ret == crate::zlib_h::Z_OK
    {
        ret = crate::zlib_h::Z_BUF_ERROR;
    }
    return ret;
}
#[export_name = "inflate"]

pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate(strm, flush)
}
pub fn inflateEnd(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    if inflate_state_invalid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = strm
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    state.window = None;
    drop(state);
    strm.state = None;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateEnd(strm)
}
pub fn inflate_get_dictionary(
    strm: &mut crate::zlib_h::z_stream,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: &mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if inflate_state_invalid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = strm
        .inflate_state()
        .expect("inflate state was checked above");
    let state = state_handle.borrow();
    let whave = state.whave as usize;
    if whave != 0 {
        let Some(dictionary) = dictionary else {
            *dict_length = state.whave as crate::stdlib::uInt;
            return crate::zlib_h::Z_OK;
        };
        if dictionary.len() < whave {
            return crate::zlib_h::Z_BUF_ERROR;
        }
        let window = state
            .window
            .as_ref()
            .expect("a non-empty history has a window");
        let tail = state.whave.wrapping_sub(state.wnext) as usize;
        dictionary[..tail].copy_from_slice(&window[state.wnext as usize..state.whave as usize]);
        dictionary[tail..whave].copy_from_slice(&window[..state.wnext as usize]);
    }
    *dict_length = state.whave as crate::stdlib::uInt;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateGetDictionary"]

pub unsafe extern "C" fn inflateGetDictionary_ffi(
    strm: crate::zlib_h::z_streamp,
    dictionary: *mut crate::stdlib::Bytef,
    dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if inflate_state_invalid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dictionary = if dictionary.is_null() {
        None
    } else {
        let dictionary_len = strm
            .inflate_state()
            .expect("inflate state was checked above")
            .borrow()
            .whave as usize;
        Some(unsafe { ::core::slice::from_raw_parts_mut(dictionary, dictionary_len) })
    };
    let mut dictionary_len = 0;
    let ret = inflate_get_dictionary(strm, dictionary, &mut dictionary_len);
    if ret == crate::zlib_h::Z_OK {
        if let Some(dict_length) = unsafe { dictLength.as_mut() } {
            *dict_length = dictionary_len;
        }
    }
    ret
}
pub fn inflate_set_dictionary(
    strm: &mut crate::zlib_h::z_stream,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if inflate_state_invalid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = strm
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    if state.wrap != 0 as ::core::ffi::c_int
        && state.mode as ::core::ffi::c_uint
            != crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.mode as ::core::ffi::c_uint
        == crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let dictid = crate::src::adler32::adler32(
            crate::src::adler32::ADLER32_INITIAL as crate::stdlib::uLong,
            dictionary,
        ) as ::core::ffi::c_ulong;
        if dictid != state.check {
            return crate::zlib_h::Z_DATA_ERROR;
        }
    }
    if updatewindow_from_slice(&mut state, dictionary) != 0 {
        state.mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    state.havedict = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateSetDictionary"]

pub unsafe extern "C" fn inflateSetDictionary_ffi(
    strm: crate::zlib_h::z_streamp,
    dictionary: *const crate::stdlib::Bytef,
    dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let dictionary = if dictLength == 0 {
        &[]
    } else {
        if dictionary.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        unsafe { ::core::slice::from_raw_parts(dictionary, dictLength as usize) }
    };
    inflate_set_dictionary(strm, dictionary)
}
pub fn inflate_get_header(
    strm: &mut crate::zlib_h::z_stream,
    head: ::std::rc::Rc<::std::cell::RefCell<crate::zlib_h::gz_header>>,
) -> ::core::ffi::c_int {
    if inflate_state_invalid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = strm
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    if (*state).wrap & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    head.borrow_mut().done = 0 as ::core::ffi::c_int;
    (*state).head = Some(head);
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateGetHeader"]

pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(head) = head.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_get_header(
        strm,
        ::std::rc::Rc::new(::std::cell::RefCell::new(head.clone())),
    )
}
unsafe extern "C" fn syncsearch(
    mut have: *mut ::core::ffi::c_uint,
    mut buf: *const ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut got: ::core::ffi::c_uint = 0;
    let mut next: ::core::ffi::c_uint = 0;
    got = *have;
    next = 0 as ::core::ffi::c_uint;
    while next < len && got < 4 as ::core::ffi::c_uint {
        if *buf.offset(next as isize) as ::core::ffi::c_int
            == (if got < 2 as ::core::ffi::c_uint {
                0 as ::core::ffi::c_int
            } else {
                0xff as ::core::ffi::c_int
            })
        {
            got = got.wrapping_add(1);
        } else if *buf.offset(next as isize) != 0 {
            got = 0 as ::core::ffi::c_uint;
        } else {
            got = (4 as ::core::ffi::c_uint).wrapping_sub(got);
        }
        next = next.wrapping_add(1);
    }
    *have = got;
    return next;
}
pub unsafe extern "C" fn inflateSync(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_uint = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut in_0: ::core::ffi::c_ulong = 0;
    let mut out: ::core::ffi::c_ulong = 0;
    let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = (&*strm)
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    if (*strm).avail_in == 0 as crate::stdlib::uInt && (*state).bits < 8 as ::core::ffi::c_uint {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    if (*state).mode as ::core::ffi::c_uint
        != crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*state).mode = crate::src::inflate::SYNC;
        (*state).hold >>= (*state).bits & 7 as ::core::ffi::c_uint;
        (*state).bits = (*state)
            .bits
            .wrapping_sub((*state).bits & 7 as ::core::ffi::c_uint);
        len = 0 as ::core::ffi::c_uint;
        while (*state).bits >= 8 as ::core::ffi::c_uint {
            let c2rust_fresh35 = len;
            len = len.wrapping_add(1);
            buf[c2rust_fresh35 as usize] = (*state).hold as ::core::ffi::c_uchar;
            (*state).hold >>= 8 as ::core::ffi::c_int;
            (*state).bits = (*state).bits.wrapping_sub(8 as ::core::ffi::c_uint);
        }
        (*state).have = 0 as ::core::ffi::c_uint;
        syncsearch(
            &raw mut (*state).have,
            &raw mut buf as *mut ::core::ffi::c_uchar,
            len,
        );
    }
    len = syncsearch(
        &raw mut (*state).have,
        crate::input_pointer!((*strm).next_in),
        (*strm).avail_in as ::core::ffi::c_uint,
    );
    (*strm).avail_in = (*strm).avail_in.wrapping_sub(len);
    (*strm).next_in = crate::zlib_h::InputBuffer(::core::num::NonZeroUsize::new(
        (*strm)
            .next_in
            .0
            .expect("non-null input cursor")
            .get()
            .wrapping_add(len as usize),
    ));
    (*strm).total_in = (*strm).total_in.wrapping_add(len as crate::stdlib::uLong);
    if (*state).have != 4 as ::core::ffi::c_uint {
        return crate::zlib_h::Z_DATA_ERROR;
    }
    if (*state).flags == -1 as ::core::ffi::c_int {
        (*state).wrap = 0 as ::core::ffi::c_int;
    } else {
        (*state).wrap &= !(4 as ::core::ffi::c_int);
    }
    flags = (*state).flags;
    in_0 = (*strm).total_in as ::core::ffi::c_ulong;
    out = (*strm).total_out as ::core::ffi::c_ulong;
    drop(state);
    inflateReset(strm);
    let state_handle = (&*strm)
        .inflate_state()
        .expect("inflate state was reset above");
    let mut state = state_handle.borrow_mut();
    (*strm).total_in = in_0 as crate::stdlib::uLong;
    (*strm).total_out = out as crate::stdlib::uLong;
    (*state).flags = flags;
    (*state).mode = crate::src::inflate::TYPE;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    inflateSync(strm)
}
pub unsafe extern "C" fn inflateSyncPoint(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = (&*strm)
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    return ((*state).mode as ::core::ffi::c_uint
        == crate::src::inflate::STORED as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*state).bits == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[export_name = "inflateSyncPoint"]

pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateSyncPoint(strm)
}
pub unsafe extern "C" fn inflateCopy(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if inflateStateCheck(source) != 0 || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = &mut *dest;
    let state_handle = (&*source)
        .inflate_state()
        .expect("source inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    let Some(copy_state) = state.copy_for_inflate_copy() else {
        return crate::zlib_h::Z_MEM_ERROR;
    };
    let mut copy = Box::new(copy_state);
    *dest = (*source).clone();
    (*copy).next = (&raw mut (*copy).codes as *mut crate::src::inftrees::code).offset(
        (*state)
            .next
            .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
            as isize,
    );
    dest.set_inflate_state(*copy);
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateCopy"]

pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateCopy(dest, source)
}
pub fn inflateUndermine(
    strm: &mut crate::zlib_h::z_stream,
    _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflate_state_invalid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = strm
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    state.sane = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_DATA_ERROR;
}
#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    strm: crate::zlib_h::z_streamp,
    subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateUndermine(strm, subvert)
}
pub fn inflateValidate(
    strm: &mut crate::zlib_h::z_stream,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflate_state_invalid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_handle = strm
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    if check != 0 && state.wrap != 0 {
        state.wrap |= 4 as ::core::ffi::c_int;
    } else {
        state.wrap &= !(4 as ::core::ffi::c_int);
    }
    crate::zlib_h::Z_OK
}
#[export_name = "inflateValidate"]

pub unsafe extern "C" fn inflateValidate_ffi(
    strm: crate::zlib_h::z_streamp,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateValidate(strm, check)
}
pub fn inflateMark(strm: &crate::zlib_h::z_stream) -> ::core::ffi::c_long {
    if inflate_state_invalid(strm) {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    }
    let state_handle = strm
        .inflate_state()
        .expect("inflate state was checked above");
    let state = state_handle.borrow();
    ((state.back as ::core::ffi::c_long as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int)
        as ::core::ffi::c_long
        + (if state.mode as ::core::ffi::c_uint
            == crate::src::inflate::COPY_1 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            state.length
        } else {
            if state.mode as ::core::ffi::c_uint
                == crate::src::inflate::MATCH as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                state.was.wrapping_sub(state.length)
            } else {
                0 as ::core::ffi::c_uint
            }
        }) as ::core::ffi::c_long
}
#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_long {
    let Some(strm) = (unsafe { strm.as_ref() }) else {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    };
    inflateMark(strm)
}
pub unsafe extern "C" fn inflateCodesUsed(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    if inflateStateCheck(strm) != 0 {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    }
    let state_handle = (&*strm)
        .inflate_state()
        .expect("inflate state was checked above");
    let mut state = state_handle.borrow_mut();
    return (*state)
        .next
        .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
        as ::core::ffi::c_ulong;
}
#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    inflateCodesUsed(strm)
}
