ENTRY(_start)

SECTIONS {
    . = 1M;
    
    .boot : ALIGN(4K) {
        KEEP(*(.multiboot))
    }

    .gdt ALIGN(8) : {
        KEEP(*(.gdt))
    }
    
    .text : {
        *(.text)
    }
    
    .rodata : {
        *(.rodata)
    }
    
    .data : {
        *(.data)
    }
    
    .bss : {
        *(.bss)
    }
}
