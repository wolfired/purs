#ifndef LIB_H
#define LIB_H

#ifdef _WIN32
#ifdef BUILD_DLL
#define PURC_API __declspec(dllexport) __stdcall
#else
#define PURC_API __declspec(dllimport) __stdcall
#endif // BUILD_DLL
#else
#define PURC_API
#endif // _WIN32

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include <stdint.h>

uint32_t PURC_API purc_max(uint32_t x, uint32_t y);
void PURC_API     purc_call_rs(void (*cb)());

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // LIB_H
