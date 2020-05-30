#ifndef _CAPI_PREDEFINES_H_
#define _CAPI_PREDEFINES_H_

/// special types
#ifdef __cplusplus
#include <cstddef>
using nullptr_t = std::nullptr_t;
#define _Bool bool
#define CAPI_EXTERN extern "C"
#else
#include <stdbool.h>
#define CAPI_EXTERN
#endif

#ifdef _WIN32
#define CAPI_EXPORT CAPI_EXTERN __declspec(dllexport)
#define CAPI_IMPORT CAPI_EXTERN __declspec(dllimport)
#else
#define CAPI_EXPORT CAPI_EXTERN
#define CAPI_IMPORT CAPI_EXTERN
#endif

#ifndef CAPI
#ifdef CAPI_DLL
#define CAPI CAPI_IMPORT
#else
#define CAPI CAPI_EXTERN
#endif // CAPI_DLL
#endif // CAPI

#endif // _CAPI_PREDEFINES_H_
