fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("Cargo must provide CARGO_CFG_TARGET_ARCH");

    match arch.as_str() {
        "x86_64" | "aarch64" => {}
        other => panic!("unsupported MARU target architecture: {other}"),
    }

    println!("cargo:rustc-link-arg=-Tlinker-{arch}.ld");
    println!("cargo:rerun-if-changed=linker-{arch}.ld");
}
