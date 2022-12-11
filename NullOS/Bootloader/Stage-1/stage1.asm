org 0x7c00

; save the current disk index for later use
mov [CURR_DISK], dl
; setting up stack
mov bp, 0x7c00
mov sp, bp

call clear_screen

; Load stage 2 from disk into memory
mov bx, STAGE2_ADDR
mov si, 0
mov eax, 0
mov cx, (stage2_end - stage2_start) / SECTOR_SIZE
call disk_load

jmp STAGE2_ADDR

%include "Bootloader/Stage-1/strings.asm"
%include "Bootloader/Stage-1/disk.asm"

times 510 - ($ - $$) db 0

dw 0xaa55