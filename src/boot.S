.equ LOCAL_CONTROL,   0xff800000
.equ LOCAL_PRESCALER, 0xff800008
.equ OSC_FREQ,        54000000
.equ MAIN_STACK,      0x400000

.section ".text.boot"  // Make sure the linker puts this at the start of the kernel image

.global _start  // Execution starts here

_start:
    // LOAD 64-bit Register 0 With LOCAL_CONTROL Address
    ldr x0, =LOCAL_CONTROL
    // STORE ZERO REGISTER value INTO LOCAL_CONTROL Address
    str wzr, [x0]
    // Move into 32 bit Register 1, Address 0x8000_0000
    mov w1, 0x80000000
    // STORE w1 INTO 64 bit register x0 shifted by 8 bytes
    str w1, [x0, #(LOCAL_PRESCALER - LOCAL_CONTROL)]

    // LOAD 64 bit register with OSC_FREQ = 54mhz = 54,000,000hz
    ldr x0, =OSC_FREQ
    // move x0 value into special register (CONTROL FREQUENCY REGISTER)
    msr cntfrq_el0, x0
    // move ZERO value into CONTROL VIRTUAL FREQUENCY OFFSET
    msr cntvoff_el2, xzr

    // Check Processor ID is zero (executin on main core), else hang
    // load into x1 the value from Multiprocessor affinity register
    // MPIDR_el1 is a 32 bit register that returns 0x0;0x1;0x2;0x3
    // Depends on Core 0-3, so if 0 then it jumps to label 2
    mrs x1, mpidr_el1
    // load into x1, the product of x1 && 0b0011
    and x1, x1, #3
    // if x1 = 0 then go to label 2 forward
    cbz x1, 2f

    // If we are not on the main core then ->
    // Load the address of spin_cpu0 into 64bit x5 register
    adr x5, spin_cpu0
    // WAIT FOR EVENT
1:  wfe
    // Load 64bit register 4 with sum of x5 and x1 shifted left #3 bits 
    ldr x4, [x5, x1, lsl #3]
    // Compare and Branch if x4 = 0, to 1b
    // Basically, if x4 is 0 then go back to 1 infinite loop
    cbz x4, 1b

    // Else load x2 with Stack Start
    // __stack_start is defined in the linker and it's aligned 16 byte
    // Each stack is separated by 512 bytes
    ldr x2, =__stack_start
    // shift left x1 by 9 bits (multiply core by 512)
    lsl x1, x1, #9
    // Add x2 and x1 store into x3
    add x3, x2, x1
    // Move the stack pointer to where x3 calculated
    mov sp, x3

    // Set all 64bit registers to 0
    mov x0, #0
    mov x1, #0
    mov x2, #0
    mov x3, #0
    // branch to the address in x4, it should be 0x1-3
    br  x4
    // Or branch to 1b? why this?
    b   1b
2:  // We're on the main core!

    // Set stack to start somewhere safe
    // Move StackPointer to 0x40_0000;
    mov sp, #MAIN_STACK

    // Clean the BSS Section
    ldr x1, =__bss_start__
    ldr w2, =__bss_size__

    // Quit Loop if Zero
3:  cbz w2, 4f
    // Store 0 into value of address in X1 THEN add 8 to the register x1
    // 8x8 = 64 bit, each address is 64 bits
    str xzr, [x1], #8
    // Substract 1 from the address 
    sub w2, w2, #1
    // if it's not 0 then go back to label 3
    cbnz w2, 3b
    // Jump to our main() routine in C (make sure it doesn't return)
4:  bl main
    // In case it does return, halt the master core too
    b 1b

.ltorg

// .quad is a 64 bit quad word address
// Set Pointer to 0x0000_00d8
.org 0xd8
// extern the spin_cpu0
.global spin_cpu0
spin_cpu0:
    .quad 0

// Set Pointer to 0x0000_00e0
.org 0xe0
.global spin_cpu1
spin_cpu1:
    .quad 0

// Set Pointer to 0x0000_00e8
.org 0xe8
.global spin_cpu2
spin_cpu2:
    .quad 0

// Set Pointer to 0x0000_00f0
.org 0xf0
.global spin_cpu3
spin_cpu3:
    .quad 0