#!/usr/bin/env bash
set -euo pipefail

kernel_path="${1:?usage: verify-elf.sh <kernel-elf> <architecture>}"
architecture="${2:?usage: verify-elf.sh <kernel-elf> <architecture>}"

test -s "$kernel_path"

header="$(readelf -h "$kernel_path")"
program_headers="$(readelf -l "$kernel_path")"
symbols="$(nm "$kernel_path")"

grep -q "Class:.*ELF64" <<<"$header"
grep -q "Type:.*EXEC" <<<"$header"
grep -q "LOAD" <<<"$program_headers"
grep -Eq '[[:space:]]kmain$' <<<"$symbols"

case "$architecture" in
  x86_64)
    grep -q "Machine:.*Advanced Micro Devices X86-64" <<<"$header"
    ;;
  aarch64)
    grep -q "Machine:.*AArch64" <<<"$header"
    ;;
  *)
    echo "Unsupported architecture: $architecture" >&2
    exit 2
    ;;
esac

echo "ELF contract passed: $architecture"
