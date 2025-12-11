#!/bin/sh

# Runs the kernel in QEMU. This script is meant to be used through `cargo`

if [ -z "$ARCH" ]; then
  ARCH="i386"
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# Root of the kernel project
KERNEL_ROOT="$SCRIPT_DIR/.."
# Root of the entire project
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

# Change to kernel directory for building
cd "$KERNEL_ROOT"

# Compile boot.asm
$AS $ASFLAGS "$PROJECT_ROOT/src/boot.asm" -o boot.o

# Find the Rust static library - use the correct path relative to kernel dir
RUST_LIB="target/$RUST_TARGET/debug/libkernel.a"

if [ ! -f "$RUST_LIB" ]; then
    >&2 echo "Error: Rust library not found at $RUST_LIB"
    exit 1
fi

# Link boot.o with Rust kernel library
$LD $LDFLAGS boot.o "$RUST_LIB" -o kiki_final

# Build ISO
mkdir -p iso/boot/grub
cp kiki_final iso/boot/kiki
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