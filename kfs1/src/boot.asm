; boot.asm - Multiboot header et bootstrap
bits 32

; Constantes multiboot
MBALIGN   equ 1 << 0
MEMINFO   equ 1 << 1
FLAGS     equ MBALIGN | MEMINFO
MAGIC     equ 0x1BADB002
CHECKSUM  equ -(MAGIC + FLAGS)

; Multiboot header
section .multiboot
align 4
    dd MAGIC
    dd FLAGS
    dd CHECKSUM

; Stack
section .bss
align 16
stack_bottom:
    resb 16384  ; 16 KB stack
stack_top:

; Point d'entrée
section .text
global _start
extern kernel_main

_start:
    ; Setup stack
    mov esp, stack_top
    
    ; Push multiboot info
    push ebx  ; Multiboot info structure
    push eax  ; Multiboot magic number
    
    ; Call kernel main
    call kernel_main
    
    ; Halt si kernel_main retourne
    cli
.hang:
    hlt
    jmp .hang
