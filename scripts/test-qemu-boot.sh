#!/usr/bin/env bash
set -euo pipefail

architecture="${1:?usage: test-qemu-boot.sh <x86_64|aarch64>}"
log_file="$(mktemp)"
clean_log_file="$(mktemp)"
trap 'rm -f "$log_file" "$clean_log_file"' EXIT

case "$architecture" in
  x86_64)
    image="yeoul-x86_64.iso"
    qemu=(
      qemu-system-x86_64
      -machine q35
      -m 512M
      -cdrom "$image"
      -display none
      -serial stdio
      -monitor none
      -no-reboot
    )
    expected_arch="Architecture: x86_64"
    ;;
  aarch64)
    image="yeoul-aarch64.iso"
    firmware="/usr/share/qemu-efi-aarch64/QEMU_EFI.fd"
    test -s "$firmware" || {
      echo "AArch64 UEFI firmware not found: $firmware" >&2
      exit 2
    }
    qemu=(
      qemu-system-aarch64
      -machine virt
      -cpu cortex-a72
      -m 512M
      -bios "$firmware"
      -cdrom "$image"
      -display none
      -serial stdio
      -monitor none
      -no-reboot
    )
    expected_arch="Architecture: AArch64"
    ;;
  *)
    echo "Unsupported architecture: $architecture" >&2
    exit 2
    ;;
esac

test -s "$image"

# The kernel intentionally idles after initialization. A timeout is therefore
# expected; the test passes only when the complete boot contract was observed.
set +e
timeout --signal=TERM --kill-after=3s 25s "${qemu[@]}" >"$log_file" 2>&1
qemu_status=$?
set -e

# UEFI firmware and Limine may emit ANSI cursor-control sequences on serial.
# Remove those controls before checking the stable kernel log contract.
sed -E 's/\x1B\[[0-9;?]*[ -/]*[@-~]//g' "$log_file" >"$clean_log_file"
cat "$clean_log_file"

grep -Fq "YEOUL OS / MARU Kernel" "$clean_log_file"
grep -Fq "Boot protocol: Limine" "$clean_log_file"
grep -Fq "$expected_arch" "$clean_log_file"
grep -Fq "Kernel initialization complete." "$clean_log_file"

echo "QEMU boot contract passed: $architecture (QEMU exit: $qemu_status)"
