MEMORY {
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 512M
}

EXTERN(start)

SECTIONS {
    .text : ALIGN(4K) {
        KEEP(*(.text.start))
        *(.text*)
        *(.rodata)
    } > RAM

    .data : ALIGN(4K) {
        *(.data*)
        *(.bss*)
        _IMAGE_END = ALIGN(4K);
    } > RAM
}

_STACK = _IMAGE_END + 16K;
_HEAP_END = _STACK + 4M;
