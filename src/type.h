#pragma once

typedef unsigned char u8;
typedef unsigned u32;

_Static_assert(sizeof(u8) == 1, "u8 is not 1 byte long");
_Static_assert(sizeof(u32) == 4, "u32 is not 4 bytes long");

#ifndef __cplusplus
typedef u8 bool;
#define false 0
#define true 1
#endif
