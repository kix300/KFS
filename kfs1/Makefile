
.PHONY: all clean run test

all: run

run: 
	cd src/kernel && cargo run
test: 
	cd src/kernel && cargo test

# Clean build artifacts
clean:
	rm -f src/kernel/boot.o src/kernel/kernel.iso src/kernel/iso/boot/grub/grub.cfg src/kernel/iso/boot/kiki
	rm -rf isodir
	cd src/kernel && cargo clean
