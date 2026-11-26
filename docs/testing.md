# MARU Kernel Testing Strategy

MARU tests externally observable contracts rather than implementation details. The same contract should hold across supported architectures unless the platform specification explicitly says otherwise.

## Test layers

### 1. Architecture-independent unit tests

Run on the host and cover deterministic shared logic. The initial suite tests HHDM physical-to-virtual address conversion, including zero values, ordinary offsets, and integer overflow.

These tests do not execute privileged instructions or access hardware.

```sh
cargo test --manifest-path kernel/Cargo.toml --lib
```

### 2. Cross-architecture build and ELF contract

Both `x86_64-unknown-none` and `aarch64-unknown-none` must compile. The ELF verifier checks that each output is ELF64, has the expected machine type, contains loadable segments, and exports the kernel entry point.

### 3. QEMU boot contract

Each architecture is booted independently under QEMU through the same Limine boot protocol. The test requires the kernel to report:

- Kernel identity
- Boot protocol
- Expected architecture
- Successful completion of early initialization

The kernel intentionally idles after this point, so the harness uses a bounded timeout. A timeout alone is not success: every required log marker must be present.

### 4. Static quality checks

Rust formatting and Clippy run on the shared kernel library. Architecture-specific code is additionally compiled for both bare-metal targets.

## Design rules

- Test shared contracts once at the host level; test platform behavior on the corresponding emulator.
- Keep x86_64 and AArch64 checks symmetrical.
- Include boundary and failure cases, not only happy paths.
- Never count compilation as proof of runtime correctness.
- Do not add placeholder tests for subsystems that do not exist yet.

## Not covered yet

There are no scheduler, address-space, capability, IPC, interrupt-controller, filesystem, or userspace tests because those subsystems are not implemented. When introduced, each should have isolated unit tests plus integration tests at its actual trust boundary.
