#!/usr/bin/env bash
set -euo pipefail

architecture="${1:?usage: test-qemu-boot.sh <x86_64|aarch64>}"
log_file="$(mktemp)"
clean_log_file="$(mktemp)"
debug_file="$(mktemp)"
trap 'rm -f "$log_file" "$clean_log_file" "$debug_file"' EXIT

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
      -d guest_errors,unimp,int
      -D "$debug_file"
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

for marker in   "YEOUL OS / MARU Kernel"   "Boot protocol: Limine"   "$expected_arch"   "Kernel initialization complete."; do
  if ! grep -Fq "$marker" "$clean_log_file"; then
    echo "QEMU boot contract failed: missing marker: $marker" >&2
    if [[ "$architecture" == "aarch64" && -s "$debug_file" ]]; then
      echo "--- QEMU AArch64 debug tail ---" >&2
      tail -n 120 "$debug_file" >&2
    fi
    exit 1
  fi
done

echo "QEMU boot contract passed: $architecture (QEMU exit: $qemu_status)"
