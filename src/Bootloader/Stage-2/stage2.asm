org 0x7e00

mov bx, STAGE2_SUCCESS_MSG
call print_string

jmp enter_protected_mode

%include "Utils/strings.asm"
%include "Stage-2/gdt.asm"

; using fast a20 gate
enable_a20_line:
    in al, 0x92
    or al, 2
    out 0x92, al
    ret

enter_protected_mode:
    call enable_a20_line

    lgdt [gdt_descriptor]

    ; disable interupts before entering protected mode (becuase protected mode change the segmention)
    cli

    ; enable the protected mode bit in the cr0 (tell the cpu to enter protected mode)
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    
    ; far jump (to update the code segment 'cs')
    jmp code_segment:start_protected_mode


[bits 32]

start_protected_mode:

    ; setting up all the data segments the the data 
    mov ax, data_segment
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    jmp $

STAGE2_SUCCESS_MSG db "stage2 loaded successfully!", 0

times 2048 - ($ - $$) db 0
