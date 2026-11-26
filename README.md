# YEOUL OS — MARU Kernel

MARU is the Rust `no_std` microkernel project for YEOUL OS.

## Current status

The repository currently contains an early boot prototype for:

- x86_64
- AArch64 (ARM64)

It uses the Limine boot protocol and prints a boot banner through the architecture's serial console. This is a bring-up milestone only; scheduler, address spaces, IPC, capabilities, user-space services, and POSIX support are not implemented yet.

## Build

A nightly Rust toolchain is required.

```sh
make kernel KARCH=x86_64
make kernel KARCH=aarch64
```

For ISO images, install GNU Make, Git, and xorriso:

```sh
make iso KARCH=x86_64
make iso KARCH=aarch64
```

See [the boot prototype notes](docs/boot-prototype.md) and [the architecture specification](docs/architecture.md).
