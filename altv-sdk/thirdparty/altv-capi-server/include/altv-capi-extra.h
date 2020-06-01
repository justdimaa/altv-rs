
/**
 * CAPI Extra features
 * This file is managed manually
 */

#ifndef _CAPI_EXTRA_H_
#define _CAPI_EXTRA_H_

#ifdef __cplusplus
using nullptr_t = std::nullptr_t;
#endif

#ifdef ALTV_SERVER // Not available in the altv-server executable
CAPI unsigned int alt_GetSDKVersion();
#define CAPI_EXPORT_SDK_VERSION CAPI_EXPORT unsigned int GetSDKVersion() { return alt_GetSDKVersion(); }
#endif
#define CAPI_SKIP_VERSION_CHECK CAPI_EXPORT void SkipSDKVersionCheck() { }

typedef void(*OnCreateBaseObjectFnType)(alt_IResource*, alt_RefBase_RefStore_IBaseObject*);
typedef void(*OnRemoveBaseObjectFnType)(alt_IResource*, alt_RefBase_RefStore_IBaseObject*);

CAPI alt_IScriptRuntime* alt_CAPIScriptRuntime_Create(
    alt_IResource_Impl*(*CreateImplFn)(alt_IScriptRuntime*, alt_IResource*),
    void(*DestroyImplFn)(alt_IScriptRuntime*, alt_IResource_Impl*),
    void(*OnTickFn)(alt_IScriptRuntime*)
);

CAPI alt_IResource_Impl* alt_CAPIResource_Impl_Create(
    alt_IResource* resource,
#ifdef ALT_SERVER_API
    _Bool(*MakeClientFn)(alt_IResource*, alt_IResource_CreationInfo*, alt_Array_String*),
#endif
    _Bool(*StartFn)(alt_IResource*),
    _Bool(*StopFn)(alt_IResource*),
    _Bool(*OnEventFn)(alt_IResource*, alt_CEvent*),
    void(*OnTickFn)(alt_IResource*),
    OnCreateBaseObjectFnType OnCreateBaseObjectFn,
    OnRemoveBaseObjectFnType OnRemoveBaseObjectFn
);

CAPI void alt_CAPIResource_Impl_SetExtra(alt_IResource_Impl* resource, void* extra);
CAPI void* alt_CAPIResource_Impl_GetExtra(alt_IResource_Impl* resource);

#endif // _CAPI_EXTRA_H_
