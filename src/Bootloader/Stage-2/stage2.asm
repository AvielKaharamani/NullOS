org 0x7e00

jmp enter_protected_mode

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
    jmp code_segment:move_to_32_protected_mode


[bits 32]

%include "Stage-2/identical_paging.asm"
%include "Stage-2/cpuid.asm"

move_to_32_protected_mode:

    ; setting up all the data segments the the data 
    mov ax, data_segment
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    jmp enter_long_mode

enter_long_mode:
    call detect_cpuid
    call setup_identical_paging
    call fit_gdt_long_mode

    ; enable physical address extension bit (in cr4)
    mov eax, cr4
    or eax, 1 << 5 ; pae bit
    mov cr4, eax

    ; enable long mode bit inside the efer msr
    mov ecx, 0xC0000080 ; EFER (Extended Feature Enable Register) msr
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging bit (in cr4)
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    
    jmp code_segment:move_to_64_bit_long_mode

[bits 64]

move_to_64_bit_long_mode:

    call _start ; call to our kernel

    jmp $

times 2048 - ($ - $$) db 0
