#include "type.h"

extern u32 _STACK;

__attribute__((naked, noreturn))
void start()
{
    asm("ldr sp, =_STACK");
    asm("b rust_start");
}
