SECTIONS
{
    . = 0x80000100;
    .htif :
    {
        PROVIDE(fromhost = .);
        . += 8;
        PROVIDE(tohost = .);
        . += 8;
    }

    . = 0x80000200;
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
