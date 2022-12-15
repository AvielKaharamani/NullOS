org 0x7c00

stage1_start:
    %include "Bootloader/Stage-1/stage1.asm"
stage1_end:

stage2_start:
    %include "Bootloader/Stage-2/stage2.asm"
    align 512, db 0
stage2_end:

kernel_start:
    incbin "NullOS.bin"
    align 512, db 0
kernel_end:
