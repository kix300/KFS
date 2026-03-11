%macro isr_err_stub 1
isr_stub_%+%1:
	add esp, 4
	pusha
	call exception_handler
	popa
	iretd
%endmacro

%macro isr_no_err_stub 1
isr_stub_%+%1:
	pusha
	call exception_handler
	popa
	iretd
%endmacro

%macro irq_stub 1
irq_stub_%+%1:
	pusha
	push %1
    call irq_handler
	add esp, 4
	popa
    iretd
%endmacro

extern exception_handler
extern irq_handler 

isr_no_err_stub 0
isr_no_err_stub 1
isr_no_err_stub 2
isr_no_err_stub 3
isr_no_err_stub 4
isr_no_err_stub 5
isr_no_err_stub 6
isr_no_err_stub 7
isr_err_stub    8
isr_no_err_stub 9
isr_err_stub    10
isr_err_stub    11
isr_err_stub    12
isr_err_stub    13
isr_err_stub    14
isr_no_err_stub 15
isr_no_err_stub 16
isr_err_stub    17
isr_no_err_stub 18
isr_no_err_stub 19
isr_no_err_stub 20
isr_no_err_stub 21
isr_no_err_stub 22
isr_no_err_stub 23
isr_no_err_stub 24
isr_no_err_stub 25
isr_no_err_stub 26
isr_no_err_stub 27
isr_no_err_stub 28
isr_no_err_stub 29
isr_err_stub    30
isr_no_err_stub 31


irq_stub 0   ; IRQ0 - Timer
irq_stub 1   ; IRQ1 - Clavier
irq_stub 2   ; IRQ2 - Cascade PIC2
irq_stub 3   ; IRQ3 - COM2
irq_stub 4   ; IRQ4 - COM1
irq_stub 5   ; IRQ5 - LPT2
irq_stub 6   ; IRQ6 - Floppy
irq_stub 7   ; IRQ7 - LPT1
irq_stub 8   ; IRQ8 - RTC
irq_stub 9   ; IRQ9
irq_stub 10  ; IRQ10
irq_stub 11  ; IRQ11
irq_stub 12  ; IRQ12 - PS/2 Mouse
irq_stub 13  ; IRQ13 - FPU
irq_stub 14  ; IRQ14 - ATA primaire
irq_stub 15  ; IRQ15 - ATA secondaire

global isr_stub_table
isr_stub_table:
%assign i 0
%rep	32
	dd isr_stub_%+i
%assign i i+1
%endrep

global irq_stub_table
irq_stub_table:
%assign i 0
%rep 16
    dd irq_stub_%+i
%assign i i+1
%endrep
