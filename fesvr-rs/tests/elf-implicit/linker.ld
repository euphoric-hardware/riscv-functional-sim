SECTIONS
{
    . = 0x80000000;
    tohost = .;
    . = ALIGN(8);
    fromhost = .;

    . = 0x80000100;
    .text :
    {
        *(.text*)
    }

    . = ALIGN(8);
    .rodata :
    {
        *(.rodata*)
    }

    . = ALIGN(8);
    .data :
    {
        *(.data*)
    }

    . = ALIGN(8);
    .bss :
    {
        *(.bss*)
    }

    /DISCARD/ :
    {
        *(.note*)
        *(.comment*)
        *(.gnu*)
    }
}
