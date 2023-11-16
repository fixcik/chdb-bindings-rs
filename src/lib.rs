use std::{os::raw::{c_char, c_int}, ffi::c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LocalResult {
    pub buf: *const c_char,
    pub size: usize,
    _vec: *mut c_void,
    pub elapsed: f64,
    pub rows_read: u64,
    pub bytes_read: u64,
}

#[link(name = "chdb")]
extern "C" {
    pub fn query_stable(argc: c_int, argv: *const *const c_char) -> *mut LocalResult;
    pub fn free_result(result: *mut LocalResult);
}
