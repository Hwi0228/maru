MAKEFLAGS += -rR
.SUFFIXES:

KARCH ?= x86_64
RUST_TARGET := $(KARCH)-unknown-none
IMAGE := yeoul-$(KARCH)

.PHONY: all kernel iso run clean
all: iso

kernel:
	cd kernel && cargo build --release --target $(RUST_TARGET)
	cp kernel/target/$(RUST_TARGET)/release/maru-kernel kernel/kernel

limine/limine:
	git clone https://github.com/limine-bootloader/limine.git --branch=v11.x-binary --depth=1
	$(MAKE) -C limine

iso: kernel limine/limine
	rm -rf iso_root
	mkdir -p iso_root/boot/limine iso_root/EFI/BOOT
	cp kernel/kernel iso_root/boot/kernel
	cp limine.conf iso_root/boot/limine/limine.conf
ifeq ($(KARCH),x86_64)
	cp limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp limine/BOOTX64.EFI limine/BOOTIA32.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o $(IMAGE).iso
	./limine/limine bios-install $(IMAGE).iso
else ifeq ($(KARCH),aarch64)
	cp limine/limine-uefi-cd.bin iso_root/boot/limine/
	cp limine/BOOTAA64.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image --protective-msdos-label iso_root -o $(IMAGE).iso
else
	$(error Unsupported KARCH=$(KARCH))
endif
	rm -rf iso_root

run: iso
ifeq ($(KARCH),x86_64)
	qemu-system-x86_64 -M q35 -m 1G -cdrom $(IMAGE).iso -serial stdio
else ifeq ($(KARCH),aarch64)
	qemu-system-aarch64 -M virt -cpu cortex-a72 -m 1G -nographic -cdrom $(IMAGE).iso
endif

clean:
	cd kernel && cargo clean
	rm -rf kernel/kernel iso_root $(IMAGE).iso

.PHONY: distclean
distclean: clean
	rm -rf limine
