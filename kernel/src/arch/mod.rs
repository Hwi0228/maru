#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "aarch64")]
mod aarch64;

pub fn init(_hhdm_offset: u64) {
    #[cfg(target_arch = "x86_64")]
    x86_64::init();

    #[cfg(target_arch = "aarch64")]
    aarch64::init(_hhdm_offset);
}

pub fn write_str(_hhdm_offset: u64, text: &str) {
    for byte in text.bytes() {
        #[cfg(target_arch = "x86_64")]
        x86_64::write_byte(byte);

        #[cfg(target_arch = "aarch64")]
        aarch64::write_byte(_hhdm_offset, byte);
    }
}

pub fn halt() {
    #[cfg(target_arch = "x86_64")]
    x86_64::halt();

    #[cfg(target_arch = "aarch64")]
    aarch64::halt();
}
