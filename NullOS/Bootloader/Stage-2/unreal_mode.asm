enter_unreal_mode:

    ; disable interupts before entering protected mode
    cli

    push ds
    push es
    
    ; load the gdt descriptor
    lgdt [gdt_descriptor]

    ; move to protected mode
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    ; update selectors
    mov ax, data_segment
    mov ds, ax
    mov es, ax
    
    ; move back to real mode
    mov eax, cr0
    and eax, ~1
    mov cr0, eax
    
    jmp 0x0:.far_jump_unreal_mode

.far_jump_unreal_mode:

    pop es
    pop ds

    sti

    ret