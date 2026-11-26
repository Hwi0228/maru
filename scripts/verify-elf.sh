#!/usr/bin/env bash
set -euo pipefail

kernel_path="${1:?usage: verify-elf.sh <kernel-elf> <architecture>}"
architecture="${2:?usage: verify-elf.sh <kernel-elf> <architecture>}"

test -s "$kernel_path"

header="$(readelf -h "$kernel_path")"
program_headers="$(readelf -l "$kernel_path")"
symbols="$(nm "$kernel_path")"

require() {
  local pattern="$1"
  local content="$2"
  local description="$3"
  if ! grep -Eq "$pattern" <<<"$content"; then
    echo "ELF contract failed: $description" >&2
    echo "--- ELF header ---" >&2
    printf '%s\n' "$header" >&2
    echo "--- Program headers ---" >&2
    printf '%s\n' "$program_headers" >&2
    echo "--- Symbols ---" >&2
    printf '%s\n' "$symbols" >&2
    exit 1
  fi
}

require 'Class:[[:space:]]+ELF64' "$header" "expected ELF64"
require 'Type:[[:space:]]+EXEC' "$header" "expected an executable ELF"
require '[[:space:]]LOAD[[:space:]]' "$program_headers" "expected loadable segments"

case "$architecture" in
  x86_64)
    require 'Machine:[[:space:]]+Advanced Micro Devices X86-64' "$header" "expected x86_64 machine type"
    require '[[:space:]]kmain$' "$symbols" "expected exported kmain entry point"
    ;;
  aarch64)
    require 'Machine:[[:space:]]+AArch64' "$header" "expected AArch64 machine type"
    require '[[:space:]]_start$' "$symbols" "expected architecture startup entry point"
    require '[[:space:]]kmain$' "$symbols" "expected Rust kernel entry point"
    ;;
  *)
    echo "Unsupported architecture: $architecture" >&2
    exit 2
    ;;
esac

echo "ELF contract passed: $architecture"
