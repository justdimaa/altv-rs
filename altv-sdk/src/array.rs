use crate::mvalue::MValue;
use crate::natives::*;
use crate::string::String;
use crate::string_view::StringView;
use std::slice::Iter;

// struct Array<T> {
//
// }

impl From<alt_Array_StringView> for Vec<StringView> {
    fn from(v: alt_Array_StringView) -> Self {
        let mut vec = Vec::new();

        for n in 0..v.size {
            vec.push(StringView::from(unsafe { *v.data.offset(n as isize) }));
        }

        vec
    }
}

impl From<alt_Array_String> for Vec<String> {
    fn from(v: alt_Array_String) -> Self {
        let mut vec = Vec::new();

        for n in 0..v.size {
            vec.push(String::from(unsafe { *v.data.offset(n as isize) }));
        }

        vec
    }
}

impl From<alt_Array_RefBase_RefStore_constIMValue> for Vec<MValue> {
    fn from(v: alt_Array_RefBase_RefStore_constIMValue) -> Self {
        let mut vec = Vec::new();

        for n in 0..v.size {
            vec.push(MValue::new(unsafe { (*v.data.offset(n as isize)).ptr }));
        }

        vec
    }
}

pub fn convert_iter_to_array_mvalue(
    v: Iter<MValue>,
) -> *mut alt_Array_RefBase_RefStore_constIMValue {
    unsafe {
        let arr = alt_Array_RefBase_RefStore_constIMValue_Create_CAPI_Heap();

        for val in v {
            let ptr: *mut alt_RefBase_RefStore_IMValue = val.clone().into();
            alt_Array_RefBase_RefStore_constIMValue_Push(
                arr,
                ptr as *mut alt_RefBase_RefStore_constIMValue,
            );
        }

        arr
    }
}
