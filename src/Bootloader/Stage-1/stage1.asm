org 0x7c00

; save the current disk index for later use
mov [CURR_DISK], dl
; setting up stack
mov bp, 0x7c00
mov sp, bp

call load_stage2_from_disk

jmp STAGE2_ADDR

%include "Utils/strings.asm"
%include "Utils/disk.asm"

times 510 - ($ - $$) db 0

dw 0xaa55