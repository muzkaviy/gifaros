ENTRY(_start)
SECTIONS
{
    . = 0x40000000; /* QEMU ARM 'virt' başlangıç adresi */
    .text : {
        KEEP(*(.text._start)) /* _start fonksiyonunu koru ve en başa koy */
        *(.text .text.*)
    }
    .data : { *(.data .data.*) }
    .bss : { *(.bss .bss.*) }
    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame) }
}