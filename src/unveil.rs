use std::ffi::CString;
use std::io;
use std::ptr;
use libc;
use crate::oserror::OSError;

pub fn unveil(path: Option<&str>, permissions: Option<&str>) -> std::result::Result<(), OSError> {
    let mut owned = Vec::new();

    let path_ptr = path.map_or_else(
        || ptr::null(),
        |p| {
            let path_cstr = CString::new(p).unwrap();
            owned.push(path_cstr);
            owned.last().unwrap().as_ptr()
        },
    );

    let permissions_ptr = permissions.map_or_else(
        || ptr::null(),
        |p| {
            let permissions_cstr = CString::new(p).unwrap();
            owned.push(permissions_cstr);
            owned.last().unwrap().as_ptr()
        },
    );

    unsafe {
        match libc::unveil(path_ptr, permissions_ptr) {
            0 => Ok(()),
            _ => Err(OSError::new(
                io::Error::last_os_error().raw_os_error().unwrap(),
            )),
        }
    }
}
