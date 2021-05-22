use std::ffi::CStr;
use std::fmt;

#[derive(Debug)]
pub struct OSError {
    details: String,
}

impl OSError {
    pub fn new(errnum: libc::c_int) -> Self {
        const NL_TEXTMAX: libc::size_t = 255; /* /usr/include/limits.h */
        let err_cstr: &CStr;
        let mut buf = [0 as libc::c_char; NL_TEXTMAX];
        unsafe {
            libc::strerror_r(errnum, buf.as_mut_ptr(), buf.len());
            err_cstr = CStr::from_ptr(buf.as_ptr());
        }
        let err_str = err_cstr.to_str().unwrap();
        OSError {
            details: err_str.to_owned(),
        }
    }
}

impl fmt::Display for OSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
