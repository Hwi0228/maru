#![no_std]
#![no_main]

mod arch;
mod boot;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(boot::base_revision_supported());

    let hhdm_offset = boot::hhdm_offset();
    arch::init(hhdm_offset);

    arch::write_str(hhdm_offset, "\r\n");
    arch::write_str(hhdm_offset, "  __  __    _    ____  _   _\r\n");
    arch::write_str(hhdm_offset, " |  \/  |  / \\  |  _ \\| | | |\r\n");
    arch::write_str(hhdm_offset, " | |\\/| | / _ \\ | |_) | |_| |\r\n");
    arch::write_str(hhdm_offset, " |_|  |_|/_/ \\_\\|____/ \\___/\r\n");
    arch::write_str(hhdm_offset, "\r\n");
    arch::write_str(hhdm_offset, "YEOUL OS / MARU Kernel\r\n");
    arch::write_str(hhdm_offset, "Boot protocol: Limine\r\n");

    #[cfg(target_arch = "x86_64")]
    arch::write_str(hhdm_offset, "Architecture: x86_64\r\n");
    #[cfg(target_arch = "aarch64")]
    arch::write_str(hhdm_offset, "Architecture: AArch64\r\n");

    arch::write_str(hhdm_offset, "Kernel initialization complete.\r\n");

    loop {
        arch::halt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        arch::halt();
    }
}
