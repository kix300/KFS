ENTRY(_start)

SECTIONS {
    . = 1M;
    
    .boot : ALIGN(4K) {
        KEEP(*(.multiboot))
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
