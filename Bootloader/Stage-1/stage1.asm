org 0x7c00

; save the current disk index for later use
mov [CURR_DISK], dl
; setting up stack
mov bp, 0x7c00
mov sp, bp

call load_stage2_from_disk

call clear_screen

jmp STAGE2_ADDR

%include "Stage-1/strings.asm"
%include "Stage-1/disk.asm"

times 510 - ($ - $$) db 0

dw 0xaa55