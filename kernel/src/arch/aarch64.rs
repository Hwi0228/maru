use core::ptr::{read_volatile, write_volatile};

// QEMU virt exposes its PL011 UART at this physical address.
const UART0_PHYS: u64 = 0x0900_0000;
const UART_DR: u64 = 0x00;
const UART_FR: u64 = 0x18;
const UART_IBRD: u64 = 0x24;
const UART_FBRD: u64 = 0x28;
const UART_LCR_H: u64 = 0x2C;
const UART_CR: u64 = 0x30;
const UART_FR_TXFF: u32 = 1 << 5;

fn uart_base(hhdm_offset: u64) -> *mut u32 {
    let address = maru_kernel::hhdm_address(hhdm_offset, UART0_PHYS)
        .expect("PL011 virtual address is outside the supported address range");
    address as *mut u32
}

pub fn init(hhdm_offset: u64) {
    let base = uart_base(hhdm_offset);

    // SAFETY: The QEMU virt platform maps the PL011 UART in the HHDM.
    unsafe {
        write_volatile(base.add((UART_CR / 4) as usize), 0);
        write_volatile(base.add((UART_IBRD / 4) as usize), 13);
        write_volatile(base.add((UART_FBRD / 4) as usize), 1);
        write_volatile(base.add((UART_LCR_H / 4) as usize), 0x70);
        write_volatile(base.add((UART_CR / 4) as usize), 0x301);
    }
}

pub fn write_byte(hhdm_offset: u64, byte: u8) {
    let base = uart_base(hhdm_offset);

    // SAFETY: The address is the PL011 UART register window on QEMU virt.
    unsafe {
        while read_volatile(base.add((UART_FR / 4) as usize)) & UART_FR_TXFF != 0 {
            core::hint::spin_loop();
        }
        write_volatile(base.add((UART_DR / 4) as usize), byte as u32);
    }
}

pub fn halt() {
    // SAFETY: WFI is used in the kernel idle loop.
    unsafe { core::arch::asm!("wfi", options(nomem, nostack)) };
}
