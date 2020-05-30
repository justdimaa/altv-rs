use crate::natives::*;
use std::ffi::{CStr, CString};

pub struct String {
    data: *mut i8,
    size: u64,
}

impl String {
    pub fn new(str: &str) -> String {
        let cstr = CString::new(str).unwrap();
        String {
            data: cstr.into_raw(),
            size: str.len() as u64,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get_data(&self) -> std::string::String {
        unsafe { CStr::from_ptr(self.data).to_str().unwrap().to_owned() }
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }
}

impl From<alt_String> for String {
    fn from(s: alt_String) -> Self {
        String {
            data: s.data,
            size: s.size,
        }
    }
}

impl From<String> for alt_String {
    fn from(s: String) -> Self {
        alt_String {
            data: s.data,
            size: s.size,
        }
    }
}
