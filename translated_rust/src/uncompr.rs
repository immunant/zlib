pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate;
pub use crate::src::inflate::inflateEnd;
pub use crate::src::inflate::inflateInit_;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::uLongf;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_NEED_DICT;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ChunkedProgress {
    total: crate::stdlib::z_size_t,
    unassigned: crate::stdlib::z_size_t,
}

impl ChunkedProgress {
    fn new(total: crate::stdlib::z_size_t) -> Self {
        Self {
            total,
            unassigned: total,
        }
    }

    fn replenish(&mut self, available: &mut crate::stdlib::uInt) {
        if *available != 0 {
            return;
        }

        *available = self
            .unassigned
            .min(crate::stdlib::uInt::MAX as crate::stdlib::z_size_t)
            as crate::stdlib::uInt;
        self.unassigned = self
            .unassigned
            .wrapping_sub(*available as crate::stdlib::z_size_t);
    }

    fn remaining(self, available: crate::stdlib::uInt) -> crate::stdlib::z_size_t {
        self.unassigned
            .wrapping_add(available as crate::stdlib::z_size_t)
    }

    fn consumed(self, available: crate::stdlib::uInt) -> crate::stdlib::z_size_t {
        self.total.wrapping_sub(self.remaining(available))
    }
}

fn normalize_uncompress_status(
    err: ::core::ffi::c_int,
    input_remaining: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if err == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else if err == crate::zlib_h::Z_NEED_DICT {
        crate::zlib_h::Z_DATA_ERROR
    } else if err == crate::zlib_h::Z_BUF_ERROR && input_remaining == 0 {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        err
    }
}

#[export_name = "uncompress2_z"]
pub unsafe extern "C" fn uncompress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if sourceLen.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source_len = *sourceLen;
    let dest_len = *destLen;
    if source_len > 0 && source.is_null() || dest_len > 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let mut stream: crate::zlib_h::z_stream = crate::zlib_h::z_stream_s {
        next_in: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        state: ::core::ptr::null_mut::<crate::src::deflate::internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut dummy = 0 as crate::stdlib::Bytef;
    if dest_len == 0 && dest.is_null() {
        dest = &raw mut dummy;
    }

    stream.next_in = source as *mut crate::stdlib::Bytef;
    stream.next_out = dest;
    let err = crate::src::inflate::inflateInit_(
        &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
        crate::zlib_h::ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
    );
    if err != crate::zlib_h::Z_OK {
        return err;
    }

    let mut input_progress = ChunkedProgress::new(source_len);
    let mut output_progress = ChunkedProgress::new(dest_len);
    let err = loop {
        output_progress.replenish(&mut stream.avail_out);
        input_progress.replenish(&mut stream.avail_in);
        let err = crate::src::inflate::inflate(
            &raw mut stream as *mut crate::zlib_h::z_stream_s,
            crate::zlib_h::Z_NO_FLUSH,
        );
        if err != crate::zlib_h::Z_OK {
            break err;
        }
    };
    let input_remaining = input_progress.remaining(stream.avail_in);
    let status = normalize_uncompress_status(err, input_remaining);
    *sourceLen = input_progress.consumed(stream.avail_in);
    *destLen = output_progress.consumed(stream.avail_out);
    crate::src::inflate::inflateEnd(&raw mut stream as *mut crate::zlib_h::z_stream_s);
    status
}

#[export_name = "uncompress2"]
pub unsafe extern "C" fn uncompress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if destLen.is_null() || sourceLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mut got = *destLen as crate::stdlib::z_size_t;
    let mut used = *sourceLen as crate::stdlib::z_size_t;
    let ret = uncompress2_z_ffi(dest, &raw mut got, source, &raw mut used);
    *sourceLen = used as crate::stdlib::uLong;
    *destLen = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    ret
}

#[export_name = "uncompress_z"]
pub unsafe extern "C" fn uncompress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    let mut used = sourceLen;
    uncompress2_z_ffi(dest, destLen, source, &raw mut used)
}

#[export_name = "uncompress"]
pub unsafe extern "C" fn uncompress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let mut used = sourceLen;
    uncompress2_ffi(dest, destLen, source, &raw mut used)
}

#[cfg(test)]
mod tests {
    use super::ChunkedProgress;

    #[test]
    fn progress_refills_in_uint_sized_chunks() {
        let Some(total) = (crate::stdlib::uInt::MAX as crate::stdlib::z_size_t).checked_add(3)
        else {
            return;
        };
        let mut progress = ChunkedProgress::new(total);
        let mut available = 0;
        progress.replenish(&mut available);
        assert_eq!(available, crate::stdlib::uInt::MAX);
        assert_eq!(progress.unassigned, 3);
        available = 0;
        progress.replenish(&mut available);
        assert_eq!(available, 3);
        assert_eq!(progress.unassigned, 0);
    }

    #[test]
    fn progress_keeps_scheduled_availability_and_accounts_consumption() {
        let mut progress = ChunkedProgress::new(10);
        let mut available = 0;
        progress.replenish(&mut available);
        assert_eq!(available, 10);
        available = 7;
        progress.replenish(&mut available);
        assert_eq!(available, 7);
        assert_eq!(progress.remaining(available), 7);
        assert_eq!(progress.consumed(available), 3);
    }

    #[test]
    fn progress_with_no_work_stays_empty() {
        let mut progress = ChunkedProgress::new(0);
        let mut available = 0;
        progress.replenish(&mut available);
        assert_eq!(available, 0);
        assert_eq!(progress.remaining(available), 0);
        assert_eq!(progress.consumed(available), 0);
    }
}
