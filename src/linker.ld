ENTRY(_start)

SECTIONS {
    . = 1M;
    
    .boot : ALIGN(4K) {
        KEEP(*(.multiboot))
    }

    .gdt 0x00000800 : ALIGN(8) {
        KEEP(*(.gdt))
    }
	
	. = 1M + SIZEOF(.boot);

    
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
