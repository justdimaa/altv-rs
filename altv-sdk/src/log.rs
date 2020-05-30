use crate::natives::*;
use crate::string_view::StringView;

pub fn info(msg: &str) {
    unsafe {
        let core = alt_ICore_Instance();
        alt_ICore_LogInfo(core, Box::into_raw(Box::new(StringView::new(msg).into())));
    }
}
#[macro_export]
macro_rules! logi {
    () => ($crate::log::info(""));

    ($($arg:tt)*) => ({
        $crate::log::info(&format!($($arg)*).to_owned());
    })
}

pub fn error(msg: &str) {
    unsafe {
        let core = alt_ICore_Instance();
        alt_ICore_LogError(core, Box::into_raw(Box::new(StringView::new(msg).into())));
    }
}
#[macro_export]
macro_rules! loge {
    () => ($crate::log::info(""));

    ($($arg:tt)*) => ({
        $crate::log::error(&format!($($arg)*).to_owned());
    })
}

pub fn warning(msg: &str) {
    unsafe {
        let core = alt_ICore_Instance();
        alt_ICore_LogWarning(core, Box::into_raw(Box::new(StringView::new(msg).into())));
    }
}
#[macro_export]
macro_rules! logw {
    () => ($crate::log::info(""));

    ($($arg:tt)*) => ({
        $crate::log::warning(&format!($($arg)*).to_owned());
    })
}

pub fn debug(msg: &str) {
    unsafe {
        let core = alt_ICore_Instance();
        alt_ICore_LogDebug(core, Box::into_raw(Box::new(StringView::new(msg).into())));
    }
}
#[macro_export]
macro_rules! logd {
    () => ($crate::log::info(""));

    ($($arg:tt)*) => ({
        $crate::log::debug(&format!($($arg)*).to_owned());
    })
}

pub fn colored(msg: &str) {
    unsafe {
        let core = alt_ICore_Instance();
        alt_ICore_LogColored(core, Box::into_raw(Box::new(StringView::new(msg).into())));
    }
}
#[macro_export]
macro_rules! logc {
    () => ($crate::log::info(""));

    ($($arg:tt)*) => ({
        $crate::log::colored(&format!($($arg)*).to_owned());
    })
}
