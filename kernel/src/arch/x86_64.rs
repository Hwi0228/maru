use core::arch::asm;

const COM1: u16 = 0x3F8;

unsafe fn outb(port: u16, value: u8) {
    // SAFETY: Caller supplies the intended legacy UART I/O port.
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    // SAFETY: Caller supplies the intended legacy UART I/O port.
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

pub fn init() {
    // Configure COM1 for 115200 baud, 8 data bits, no parity, one stop bit.
    unsafe {
        outb(COM1 + 1, 0x00);
        outb(COM1 + 3, 0x80);
        outb(COM1, 0x01);
        outb(COM1 + 1, 0x00);
        outb(COM1 + 3, 0x03);
        outb(COM1 + 2, 0xC7);
        outb(COM1 + 4, 0x0B);
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        while inb(COM1 + 5) & 0x20 == 0 {
            core::hint::spin_loop();
        }
        outb(COM1, byte);
    }
}

pub fn halt() {
    // SAFETY: HLT is used only after boot-time initialization, with interrupts masked.
    unsafe { asm!("hlt", options(nomem, nostack)) };
}
