use crate::natives::*;
use std::ffi::{CStr, CString};

pub struct StringView {
    data: *mut i8,
    size: u64,
}

impl StringView {
    pub fn new(str: &str) -> StringView {
        let cstr = CString::new(str).unwrap();
        StringView {
            data: cstr.into_raw(),
            size: str.len() as u64,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get_data(&self) -> String {
        unsafe { CStr::from_ptr(self.data).to_str().unwrap().to_owned() }
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }
}

impl From<alt_StringView> for StringView {
    fn from(s: alt_StringView) -> Self {
        StringView {
            data: s.data,
            size: s.size,
        }
    }
}

impl From<StringView> for alt_StringView {
    fn from(s: StringView) -> Self {
        alt_StringView {
            data: s.data,
            size: s.size,
        }
    }
}
