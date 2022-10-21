
SECTIONS
{
    .text : {
        *(.text.boot)
        *(.text._start) 
        *(.text.*)) 
    }
    .rodata : { *(.rodata .rodata.* .gnu.linkonce.r*) }
    PROVIDE(_data = .);
    .data : { *(.data .data.* .gnu.linkonce.d*) }
    .bss (NOLOAD) : {
        . = ALIGN(16);
        __bss_start__ = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(16);
        __bss_end__ = .;
        __bss_size__ = (__bss_end__ - __bss_start__);
    }
    .cpu1Stack :
    {
        . = ALIGN(16);
        __stack_start  = .;
        . = . + 512;
        __cpu1_stack  = .;
    }
    .cpu2Stack :
    {
        . = . + 512;
        __cpu2_stack  = .;
    }
    .cpu3Stack :
    {
        . = . + 512;
        __cpu3_stack  = .;
    }
    _end = .;

   /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*)  }
}
