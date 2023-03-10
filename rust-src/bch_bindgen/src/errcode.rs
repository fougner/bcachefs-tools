use crate::bcachefs;
use std::ffi::CStr;
use std::fmt;

pub use crate::c::bch_errcode;

impl fmt::Display for bch_errcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe { CStr::from_ptr(bcachefs::bch2_err_str(*self as i32)) };
        write!(f, "{:?}", s)
    }
}

/* Can we make a function generic over ptr constness? */

pub fn errptr_to_result<T>(p: *mut T) -> Result<*mut T, bch_errcode> {
    let addr = p as usize;
    let max_err: isize = -4096;
    if addr > max_err as usize {
        let addr = addr as i32;
        let err: bch_errcode = unsafe { std::mem::transmute(-addr) };
        Err(err)
    } else {
        Ok(p)
    }
}

pub fn errptr_to_result_c<T>(p: *const T) -> Result<*const T, bch_errcode> {
    let addr = p as usize;
    let max_err: isize = -4096;
    if addr > max_err as usize {
        let addr = addr as i32;
        let err: bch_errcode = unsafe { std::mem::transmute(-addr) };
        Err(err)
    } else {
        Ok(p)
    }
}

impl std::error::Error for bch_errcode {}
