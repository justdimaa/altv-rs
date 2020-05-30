use altv::app::CoreApplication;
use altv::app::ResourceMainFn;
use altv::sdk::natives::*;
use altv::sdk::string_view::StringView;
use libloading::Library;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

const RES_TYPE: &'static str = "rust";

thread_local! {
    static APPS: RefCell<HashMap<*mut alt_IResource, CoreApplication>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub unsafe extern "C" fn altMain(core: *mut alt_ICore) -> bool {
    match core.as_mut() {
        Some(core) => {
            alt_ICore_SetInstance(core);
            let script_rt =
                alt_CAPIScriptRuntime_Create(Some(create_impl), Some(destroy_impl), Some(tick));

            match script_rt.as_mut() {
                Some(script_rt) => alt_ICore_RegisterScriptRuntime(
                    core,
                    Box::into_raw(Box::new(StringView::new(RES_TYPE).into())),
                    script_rt,
                ),
                None => false,
            }
        }
        None => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn GetSDKVersion() -> u32 {
    altv::sdk::core::SDK_VERSION
}

unsafe extern "C" fn create_impl(
    _script_rt: *mut alt_IScriptRuntime,
    res: *mut alt_IResource,
) -> *mut alt_IResource_Impl {
    dbg!("Triggered create implementation function.");

    if _script_rt.is_null() {
        panic!("Script runtime is null.");
    }

    if res.is_null() {
        panic!("Resource is null.");
    }

    let path = StringView::from(*alt_IResource_GetPath_CAPI_Heap(res)).get_data();
    let main = StringView::from(*alt_IResource_GetMain_CAPI_Heap(res)).get_data();
    let lib = Library::new(Path::new(&path).join(&main).display().to_string()).unwrap();
    let main_fn: ResourceMainFn = *lib.get(b"main\0").unwrap();
    let main_result = main_fn();

    match main_result {
        Ok(app) => {
            // TODO: better alternative to "leak"
            Box::leak(Box::new(lib));

            APPS.with(|apps| {
                let mut apps = apps.borrow_mut();
                apps.insert(res, app);
            });

            alt_CAPIResource_Impl_Create(
                res,
                Some(res_make_client),
                Some(res_start),
                Some(res_stop),
                Some(res_on_event),
                Some(res_on_tick),
                Some(res_on_create_base_object),
                Some(res_on_remove_base_object),
            )
        }
        Err(_) => std::ptr::null_mut(),
    }
}

unsafe extern "C" fn destroy_impl(
    _script_rt: *mut alt_IScriptRuntime,
    _res_impl: *mut alt_IResource_Impl,
) {
    dbg!("Triggered destroy implementation function.");
}

unsafe extern "C" fn tick(_script_rt: *mut alt_IScriptRuntime) {}

unsafe extern "C" fn res_make_client(
    _res: *mut alt_IResource,
    res_info: *mut alt_IResource_CreationInfo,
    _str_arr: *mut alt_Array_String,
) -> bool {
    dbg!("Triggered make resource client function.");
    (*res_info).type_ = altv::sdk::string::String::new("js").into();
    true
}

unsafe extern "C" fn res_start(res: *mut alt_IResource) -> bool {
    dbg!("Triggered start resource function.");

    if res.is_null() {
        altv::sdk::log::error("[Rust] Resource is null.");
        return false;
    }

    APPS.with(|apps| {
        let mut apps = apps.borrow_mut();
        let app = apps.get_mut(&res).unwrap();
        app.start();
    });

    true
}

unsafe extern "C" fn res_stop(res: *mut alt_IResource) -> bool {
    dbg!("Triggered stop resource function.");

    if res.is_null() {
        altv::sdk::log::error("[Rust] Resource is null.");
        return false;
    }

    APPS.with(|apps| {
        let mut apps = apps.borrow_mut();
        let app = apps.get_mut(&res).unwrap();
        app.stop();
    });

    true
}

unsafe extern "C" fn res_on_event(res: *mut alt_IResource, e: *mut alt_CEvent) -> bool {
    dbg!("Triggered event function.");

    if res.is_null() {
        panic!("Resource is null.");
    }

    if e.is_null() {
        panic!("Event is null.");
    }

    dbg!(alt_CEvent_GetType(e));

    APPS.with(|apps| {
        let mut apps = apps.borrow_mut();
        let app = apps.get_mut(&res).unwrap();
        app.handle_event(e);
    });
    true
}

unsafe extern "C" fn res_on_tick(res: *mut alt_IResource) {
    // dbg!("Triggered tick resource function.");

    if res.is_null() {
        altv::sdk::log::error("[Rust] Resource is null.");
        return;
    }

    APPS.with(|apps| {
        let mut apps = apps.borrow_mut();
        let app = apps.get_mut(&res).unwrap();
        app.tick();
    });
}

unsafe extern "C" fn res_on_create_base_object(
    res: *mut alt_IResource,
    base_obj: *mut alt_RefBase_RefStore_IBaseObject,
) {
    dbg!("Triggered create base object resource function.");

    if res.is_null() {
        panic!("Resource is null.");
    }

    if base_obj.is_null() {
        panic!("Base object is null.");
    }

    dbg!(alt_IBaseObject_GetType((*base_obj).ptr));

    APPS.with(|apps| {
        let apps = apps.as_ptr();
        // let mut apps = apps.borrow_mut();
        let app = (*apps).get_mut(&res).unwrap();
        app.create_game_object((*base_obj).ptr);
    });
}

unsafe extern "C" fn res_on_remove_base_object(
    res: *mut alt_IResource,
    base_obj: *mut alt_RefBase_RefStore_IBaseObject,
) {
    dbg!("Triggered remove base object resource function.");

    if res.is_null() {
        panic!("Resource is null.");
    }

    if base_obj.is_null() {
        panic!("Base object is null.");
    }

    dbg!(alt_IBaseObject_GetType((*base_obj).ptr));

    APPS.with(|apps| {
        let apps = apps.as_ptr();
        // let mut apps = apps.borrow_mut();
        let app = (*apps).get_mut(&res).unwrap();
        app.remove_game_object((*base_obj).ptr);
    });
}
