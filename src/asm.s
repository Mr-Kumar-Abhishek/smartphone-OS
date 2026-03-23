/* ARMv8 assembly startup for Redme-9A OS */

.section .text.start
.global _start
_start:
    /* Set stack pointer to top of stack (provided by linker) */
    ldr x0, =_stack_top
    mov sp, x0

    /* Clear .bss section */
    ldr x0, =_bss_start
    ldr x1, =_bss_end
    cmp x0, x1
    beq .Lbss_clear_done
.Lbss_clear_loop:
    str xzr, [x0], #8
    cmp x0, x1
    blt .Lbss_clear_loop
.Lbss_clear_done:

    /* Branch to Rust main */
    bl rust_main

    /* If rust_main returns, loop forever */
.Lhalt:
    wfe
    b .Lhalt

/* Exception vector table (simple) */
.section .vectors, "ax"
.global vectors
vectors:
    /* Current EL with SP0 */
    b .   /* Synchronous */
    b .   /* IRQ */
    b .   /* FIQ */
    b .   /* SError */

    /* Current EL with SPx */
    b .
    b .
    b .
    b .

    /* Lower EL using AArch64 */
    b .
    b .
    b .
    b .

    /* Lower EL using AArch32 */
    b .
    b .
    b .
    b .