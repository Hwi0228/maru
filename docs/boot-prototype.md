# MARU Boot Prototype

**Status:** Initial prototype  
**Boot protocol:** Limine  
**Kernel language:** Rust (`no_std`)  
**Targets:** x86_64 and AArch64

## What is implemented

- A shared Rust kernel entry point.
- Architecture-specific early console output:
  - x86_64: COM1 serial UART.
  - AArch64: PL011 UART on the QEMU `virt` platform, accessed through the Limine HHDM.
- Architecture-specific Limine base revisions: revision 0 for the current x86_64 path and revision 6 for AArch64.
- Limine HHDM request.
- Architecture-specific linker scripts and static ELF output.
- Cross-compilation CI for both target triples.
- A basic Limine ISO build path.

The prototype prints a short boot banner and enters an architecture-specific idle loop.

## Build

Install a nightly Rust toolchain, QEMU, GNU Make, Git, and (for ISO creation) xorriso.

Build only:

```sh
make kernel KARCH=x86_64
make kernel KARCH=aarch64
```

Create bootable images:

```sh
make iso KARCH=x86_64
make iso KARCH=aarch64
```

Run under QEMU:

```sh
make run KARCH=x86_64
make run KARCH=aarch64
```

The AArch64 image is intended for QEMU's `virt` machine and its PL011 UART. Physical ARM hardware is not yet supported.

## Current limitations

This is a boot-and-console prototype, not yet a functioning microkernel. It does not implement user mode, address-space management, scheduling, IPC, capabilities, a filesystem, or POSIX services. The AArch64 PL011 mapping and both runtime boot paths still require QEMU validation.
