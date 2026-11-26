.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    // Limine base revision 6 leaves FP/SIMD disabled. Enable it before
    // entering Rust, which may use Advanced SIMD as part of its target ABI.
    mrs x9, CPACR_EL1
    mov x10, #3
    lsl x10, x10, #20
    orr x9, x9, x10
    msr CPACR_EL1, x9
    isb

    // Limine supplies the initial stack and higher-half page tables.
    b kmain

.size _start, . - _start
