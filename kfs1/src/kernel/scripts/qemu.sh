#!/bin/sh

# Runs the kernel in QEMU. This script is meant to be used through `cargo`

if [ -z "$ARCH" ]; then
  ARCH="i386"
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
KERNEL_ROOT="$SCRIPT_DIR/.."
PROJECT_ROOT="$KERNEL_ROOT/../.."

case $ARCH in
    "x86"|"i386")
        QEMU=qemu-system-i386
        AS=nasm
        ASFLAGS="-f elf32"
        LD=ld
        LDFLAGS="-m elf_i386 -T $PROJECT_ROOT/src/linker.ld"
        RUST_TARGET="i386-unknown-none"
        ;;
    "x86_64")
        QEMU=qemu-system-x86_64
        AS=nasm
        ASFLAGS="-f elf64"
        LD=ld
        LDFLAGS="-m elf_x86_64 -T $PROJECT_ROOT/src/linker.ld"
        RUST_TARGET="x86_64-unknown-none"
        ;;
    *)
        >&2 echo "Invalid architecture '$ARCH'"
        exit 1
        ;;
esac

$AS $ASFLAGS "$PROJECT_ROOT/src/boot.asm" -o boot.o

RUST_LIB="target/$RUST_TARGET/debug/libkernel.a"

if [ ! -f "$RUST_LIB" ]; then
    >&2 echo "Error: Rust library not found at $RUST_LIB"
    exit 1
fi

$LD $LDFLAGS boot.o "$RUST_LIB" -o kiki

# Build ISO
mkdir -p iso/boot/grub
mv kiki iso/boot/kiki
cp grub.cfg iso/boot/grub
grub-mkrescue -o kernel.iso iso

# Run the kernel
export QEMUDISK=qemu_disk
export QEMUFLAGS="-device isa-debug-exit,iobase=0xf4,iosize=0x04 $QEMUFLAGS"
if [ -f $QEMUDISK ]; then
  QEMUFLAGS="-drive file=$QEMUDISK,format=raw $QEMUFLAGS"
fi

${QEMU} -cdrom kernel.iso $QEMUFLAGS
EXIT=$?

if [ "$EXIT" -ne 33 ]; then
    exit 1
fi
