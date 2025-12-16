
.PHONY: all clean run test
export ARCH ?= i386

all: run

run:
	cd src/kernel && cargo clean && cargo run

test:
	cd src/kernel && cargo clean && RUSTFLAGS="--cfg kfs_test" cargo run

clean:
	rm -f src/kernel/boot.o src/kernel/kernel.iso src/kernel/iso/boot/grub/grub.cfg src/kernel/iso/boot/kiki
	rm -rf isodir
	cd src/kernel && cargo clean
