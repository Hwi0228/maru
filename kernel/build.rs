fn main() {
    let arch =
        std::env::var("CARGO_CFG_TARGET_ARCH").expect("Cargo must provide CARGO_CFG_TARGET_ARCH");
    let target_os =
        std::env::var("CARGO_CFG_TARGET_OS").expect("Cargo must provide CARGO_CFG_TARGET_OS");

    match arch.as_str() {
        "x86_64" | "aarch64" => {}
        other => panic!("unsupported MARU target architecture: {other}"),
    }

    // Host-side unit tests must use the host linker. The MARU linker scripts
    // are only valid for bare-metal targets.
    if target_os == "none" {
        println!("cargo:rustc-link-arg=-Tlinker-{arch}.ld");
        println!("cargo:rerun-if-changed=linker-{arch}.ld");
    }
}
