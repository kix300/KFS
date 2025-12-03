AS = nasm
CARGO = cargo
LD = ld

ASFLAGS = -f elf32
LDFLAGS = -m elf_i386 -T src/linker.ld

KERNEL_DIR = src/kernel
TARGET_DIR = $(KERNEL_DIR)/target/i386-unknown-none/release
RUST_LIB = $(TARGET_DIR)/libkernel.a

KERNEL = kernel.bin
ISO = myos.iso

.PHONY: all clean run rust-kernel

all: $(ISO)

# Compile boot.asm
boot.o: src/boot.asm
	$(AS) $(ASFLAGS) src/boot.asm -o boot.o

# Build Rust kernel
rust-kernel:
	cd $(KERNEL_DIR) && $(CARGO) build --release --target i386-unknown-none.json -Z build-std=core,alloc

# Link everything
$(KERNEL): boot.o rust-kernel
	$(LD) $(LDFLAGS) boot.o $(RUST_LIB) -o $(KERNEL)

# Create bootable ISO
$(ISO): $(KERNEL)
	mkdir -p isodir/boot/grub
	cp $(KERNEL) isodir/boot/
	cp boot/grub/grub.cfg isodir/boot/grub/
	grub-mkrescue -o $(ISO) isodir

# Run with QEMU
run: $(ISO)
	qemu-system-i386 -cdrom $(ISO)

# Clean build artifacts
clean:
	rm -f boot.o $(KERNEL) $(ISO)
	rm -rf isodir
	cd $(KERNEL_DIR) && cargo clean
