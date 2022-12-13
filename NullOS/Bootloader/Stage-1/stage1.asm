[bits 16]

bootloader_entry:
    xor ax, ax
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    jmp 0x0:.init_cs

.init_cs:

    ; save the current disk index for later use
    mov [CURR_DISK], dl
    ; setting up stack
    mov bp, 0x7c00
    mov sp, bp

    ;call clear_screen
    
    mov bx, JUMPING_MSG
    call print_string

    ; Load stage 2 from disk into memory
    mov bx, stage2_start
    mov dx, cs
    mov eax, (stage2_start - stage1_start) / SECTOR_SIZE
    mov cx, (stage2_end - stage2_start) / SECTOR_SIZE
    call disk_load
    
    jmp stage2_start

%include "Bootloader/Stage-1/strings.asm"
%include "Bootloader/Stage-1/disk.asm"

JUMPING_MSG db "Jumping to stage2!", 0

times 510 - ($ - $$) db 0

dw 0xaa55