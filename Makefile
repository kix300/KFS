
.PHONY: all clean run test
export ARCH ?= i386

all: run

run:
	#cd src/kernel && cargo run
	cd src/kernel && cargo run -Zjson-target-spec #for nixos


test:
	cd src/kernel && RUSTFLAGS="--cfg kfs_test" cargo run

clean:
	rm -f src/kernel/gdt.o src/kernel/idt.o src/kernel/boot.o src/kernel/kernel.iso src/kernel/iso/boot/grub/grub.cfg src/kernel/iso/boot/kiki
	rm -rf isodir
	cd src/kernel && cargo clean

re : clean run
