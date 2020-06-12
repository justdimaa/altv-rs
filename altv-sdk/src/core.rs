use crate::elements::CPlayer;
use crate::mvalue::MValue;
use crate::natives::*;
use crate::string_view::StringView;

pub const DEFAULT_DIMENSION: i32 = 0;
pub const GLOBAL_DIMENSION: i32 = -2147483648;
pub const SDK_VERSION: u32 = 36;

pub fn emit_client(cplayer: Option<&mut CPlayer>, event_name: &str, args: &[MValue]) {
    match cplayer {
        Some(cplayer) => cplayer.emit(event_name, args),
        None => unsafe {
            let core = alt_ICore_Instance();
            alt_ICore_TriggerClientEvent(
                core,
                alt_RefBase_RefStore_IPlayer_Create_3_CAPI_Heap(),
                Box::into_raw(Box::new(StringView::new(event_name).into())),
                crate::array::convert_iter_to_array_mvalue(args.iter()),
            )
        },
    }
}
