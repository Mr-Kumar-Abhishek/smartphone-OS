/* Linker script for Redme-9A OS kernel (ARMv8-A QEMU virt) */
ENTRY(_start)

/* Memory layout for QEMU virt machine (DRAM starts at 0x40000000) */
MEMORY
{
    RAM (rwx) : ORIGIN = 0x40080000, LENGTH = 128M
}

_stack_size = 0x10000; /* 64 KB stack */

SECTIONS
{
    . = 0x40080000;

    .text : {
        *(.text .text.*)
    } > RAM

    .rodata : {
        *(.rodata .rodata.*)
    } > RAM

    .data : {
        *(.data .data.*)
    } > RAM

    .bss : {
        _bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        _bss_end = .;
    } > RAM

    /* Stack region */
    .stack (NOLOAD) : {
        . = ALIGN(16);
        _stack_bottom = .;
        . += _stack_size;
        . = ALIGN(16);
        _stack_top = .;
    } > RAM

    /DISCARD/ : {
        *(.comment)
        *(.gnu*)
        *(.note*)
        *(.eh_frame*)
    }
}

/* Provide symbols for startup code */
_stack_pointer = _stack_top;
_bss_size = _bss_end - _bss_start;