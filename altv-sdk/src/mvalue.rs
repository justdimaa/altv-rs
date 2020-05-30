use crate::natives::*;
use crate::rgba::Rgba;
use crate::string;
use crate::string_view::StringView;
use crate::vector::Vector3;
use std::fmt;

// #[derive(FromPrimitive, ToPrimitive)]
#[derive(Clone)]
pub enum MValue {
    None,
    Nil,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Double(f64),
    String(String),
    List(Vec<MValue>),
    // Dict(HashMap<String, MValue>),
    // Entity(BaseObject),
    // Function,
    Vector3(Vector3),
    Rgba(Rgba),
    // ByteArray(Vec<u8>),
}

impl MValue {
    pub fn new(ptr: *mut alt_IMValue) -> MValue {
        unsafe {
            dbg!(alt_IMValue_GetType(ptr));

            match alt_IMValue_GetType(ptr) {
                alt_IMValue_Type::ALT_IMVALUE_TYPE_NONE => MValue::None,
                alt_IMValue_Type::ALT_IMVALUE_TYPE_NIL => MValue::Nil,
                alt_IMValue_Type::ALT_IMVALUE_TYPE_BOOL => {
                    let ptr = alt_IMValue_to_alt_IMValueBool(ptr);
                    MValue::Bool(alt_IMValueBool_Value(ptr))
                }
                alt_IMValue_Type::ALT_IMVALUE_TYPE_INT => {
                    let ptr = alt_IMValue_to_alt_IMValueInt(ptr);
                    MValue::Int(alt_IMValueInt_Value(ptr))
                }
                alt_IMValue_Type::ALT_IMVALUE_TYPE_UINT => {
                    let ptr = alt_IMValue_to_alt_IMValueUInt(ptr);
                    MValue::Uint(alt_IMValueUInt_Value(ptr))
                }
                alt_IMValue_Type::ALT_IMVALUE_TYPE_DOUBLE => {
                    let ptr = alt_IMValue_to_alt_IMValueDouble(ptr);
                    MValue::Double(alt_IMValueDouble_Value(ptr))
                }
                alt_IMValue_Type::ALT_IMVALUE_TYPE_STRING => {
                    let ptr = alt_IMValue_to_alt_IMValueString(ptr);
                    let val = alt_IMValueString_Value_CAPI_Heap(ptr);
                    MValue::String(StringView::from(*val).get_data())
                }
                alt_IMValue_Type::ALT_IMVALUE_TYPE_LIST => {
                    let ptr = alt_IMValue_to_alt_IMValueList(ptr);

                    let mut vec = Vec::new();

                    for n in 0..alt_IMValueList_GetSize(ptr) {
                        let val = alt_IMValueList_Get_1_CAPI_Heap(ptr, n);
                        vec.push(MValue::new(alt_RefBase_RefStore_constIMValue_Get(val)));
                    }

                    MValue::List(vec)
                }
                // alt_IMValue_Type::ALT_IMVALUE_TYPE_DICT => {
                //     let ptr = alt_IMValue_to_alt_IMValueDict(ptr);
                //
                //     let mut map = HashMap::new();
                //
                //     let iter = alt_IMValueDict_Begin(ptr);
                //
                //     for _ in 0..alt_IMValueDict_GetSize(ptr) {
                //         let key = alt_IMValueDict_Iterator_GetKey_CAPI_Heap(iter);
                //         let val = alt_IMValueDict_Iterator_GetValue_CAPI_Heap(iter);
                //         map.insert()
                //     }
                // }
                alt_IMValue_Type::ALT_IMVALUE_TYPE_VECTOR3 => {
                    let ptr = alt_IMValue_to_alt_IMValueVector3(ptr);
                    let val = alt_IMValueVector3_Value_CAPI_Heap(ptr);
                    MValue::Vector3(Vector3::from(*val))
                }
                alt_IMValue_Type::ALT_IMVALUE_TYPE_RGBA => {
                    let ptr = alt_IMValue_to_alt_IMValueRGBA(ptr);
                    let val = alt_IMValueRGBA_Value_CAPI_Heap(ptr);
                    MValue::Rgba(Rgba::from(*val))
                }
                _ => unimplemented!(),
            }
        }
    }

    pub fn test() {
        unsafe {
            dbg!();
            let core = alt_ICore_Instance();
            dbg!();

            let val = alt_ICore_CreateMValueRGBA_CAPI_Heap(
                core,
                Box::into_raw(Box::new(Rgba::new(53, 1, 12, 255).into())),
            );
            dbg!();
            let val = alt_RefBase_RefStore_constIMValue_Get(
                val as *mut alt_RefBase_RefStore_constIMValue,
            );
            dbg!(*val);
            let val = alt_IMValue_to_alt_IMValueRGBA(val);
            dbg!(alt_IMValueRGBA_GetType(val));
            dbg!();
            dbg!(*alt_IMValueRGBA_Value_CAPI_Heap(val));
            dbg!();

            // let val = dbg!(alt_ICore_CreateMValueNone(core));
            // let val = dbg!(val as *mut alt_RefBase_RefStore_constIMValue);
            // dbg!((*val).ptr.as_ref());

            // let val = alt_ICore_CreateMValueString(core, Box::into_raw(Box::new(string::String::new("hii").into())));
            // let val = alt_RefBase_RefStore_constIMValue_Get(val as *mut alt_RefBase_RefStore_constIMValue);
            // dbg!(alt_IMValue_GetType(val));
            // let val = alt_IMValue_to_alt_IMValueString(val);
            // dbg!(StringView::from(*alt_IMValueString_Value(val)).get_data());

            // let val = alt_ICore_CreateMValueInt_CAPI_Heap(core, 33);
            // let val = alt_RefBase_RefStore_constIMValue_Get(
            //     val as *mut alt_RefBase_RefStore_constIMValue,
            // );
            // dbg!(alt_IMValue_GetType(val));
            // let val = alt_IMValue_to_alt_IMValueInt(val);
            // dbg!(alt_IMValueInt_Value(val));

            // let val = dbg!(alt_IMValue_to_alt_IMValueNone(val));
        }
    }
}

impl fmt::Debug for MValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MValue::None => write!(f, ""),
            MValue::Nil => write!(f, ""),
            MValue::Bool(v) => write!(f, "{}", v),
            MValue::Int(v) => write!(f, "{}L", v),
            MValue::Uint(v) => write!(f, "{}uL", v),
            MValue::Double(v) => write!(f, "{:.1}", v),
            MValue::String(ref v) => write!(f, "\"{}\"", *v),
            MValue::List(ref v) => write!(f, "{:?}", *v),
            MValue::Vector3(ref v) => write!(f, "{}", *v),
            MValue::Rgba(ref v) => write!(f, "{}", *v),
        }
    }
}

impl fmt::Display for MValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MValue::None => write!(f, ""),
            MValue::Nil => write!(f, ""),
            MValue::Bool(v) => write!(f, "{}", v),
            MValue::Int(v) => write!(f, "{}L", v),
            MValue::Uint(v) => write!(f, "{}uL", v),
            MValue::Double(v) => write!(f, "{:.1}", v),
            MValue::String(ref v) => write!(f, "\"{}\"", *v),
            MValue::List(ref v) => write!(f, "{:?}", *v),
            MValue::Vector3(ref v) => write!(f, "{}", *v),
            MValue::Rgba(ref v) => write!(f, "{}", *v),
        }
    }
}

impl From<MValue> for Option<()> {
    fn from(v: MValue) -> Self {
        match v {
            MValue::None => Some(()),
            MValue::Nil => Some(()),
            _ => None,
        }
    }
}

impl From<MValue> for Option<bool> {
    fn from(v: MValue) -> Self {
        match v {
            MValue::Bool(v) => Some(v),
            _ => None,
        }
    }
}

impl From<MValue> for *mut alt_RefBase_RefStore_IMValue {
    fn from(v: MValue) -> Self {
        unsafe {
            let core = alt_ICore_Instance();

            match v {
                MValue::None => {
                    let val = alt_ICore_CreateMValueNone_CAPI_Heap(core);
                    val as *mut alt_RefBase_RefStore_IMValue
                }
                MValue::Nil => {
                    let val = alt_ICore_CreateMValueNil_CAPI_Heap(core);
                    val as *mut alt_RefBase_RefStore_IMValue
                }
                MValue::Bool(v) => {
                    let val = alt_ICore_CreateMValueBool_CAPI_Heap(core, v);
                    val as *mut alt_RefBase_RefStore_IMValue
                }
                MValue::Int(v) => {
                    let val = alt_ICore_CreateMValueInt_CAPI_Heap(core, v);
                    val as *mut alt_RefBase_RefStore_IMValue
                }
                MValue::Uint(v) => {
                    let val = alt_ICore_CreateMValueUInt_CAPI_Heap(core, v);
                    val as *mut alt_RefBase_RefStore_IMValue
                }
                MValue::Double(v) => {
                    let val = alt_ICore_CreateMValueDouble_CAPI_Heap(core, v);
                    val as *mut alt_RefBase_RefStore_IMValue
                }
                MValue::String(v) => {
                    let val = alt_ICore_CreateMValueString_CAPI_Heap(
                        core,
                        Box::into_raw(Box::new(string::String::new(v.as_str()).into())),
                    );
                    val as *mut alt_RefBase_RefStore_IMValue
                }
                MValue::List(v) => {
                    let val = alt_ICore_CreateMValueList_CAPI_Heap(core, 0);

                    // for val in v.iter() {
                    //
                    // }

                    val as *mut alt_RefBase_RefStore_IMValue
                }
                // MValue::BaseObject(v) => {
                //     let val = alt_ICore_CreateMValueBaseObject_CAPI_Heap(core, );
                // }
                MValue::Vector3(v) => {
                    let val = alt_ICore_CreateMValueVector3_CAPI_Heap(
                        core,
                        Box::into_raw(Box::new(v.into())),
                    );
                    val as *mut alt_RefBase_RefStore_IMValue
                }
                MValue::Rgba(v) => {
                    let val = alt_ICore_CreateMValueRGBA_CAPI_Heap(
                        core,
                        Box::into_raw(Box::new(v.into())),
                    );
                    val as *mut alt_RefBase_RefStore_IMValue
                }
            }
        }
    }
}

impl From<bool> for MValue {
    fn from(v: bool) -> Self {
        MValue::Bool(v)
    }
}

impl From<i64> for MValue {
    fn from(v: i64) -> Self {
        MValue::Int(v)
    }
}

impl From<u64> for MValue {
    fn from(v: u64) -> Self {
        MValue::Uint(v)
    }
}

impl From<f64> for MValue {
    fn from(v: f64) -> Self {
        MValue::Double(v)
    }
}

impl From<String> for MValue {
    fn from(v: String) -> Self {
        MValue::String(v)
    }
}

impl From<Vector3> for MValue {
    fn from(v: Vector3) -> Self {
        MValue::Vector3(v)
    }
}

impl From<Rgba> for MValue {
    fn from(v: Rgba) -> Self {
        MValue::Rgba(v)
    }
}
