mov bx, STAGE2_MSG
call print_string

; Load Kernel from disk into memory
mov bx, kernel_start
mov dx, cs
mov eax, (kernel_start - stage1_start) / SECTOR_SIZE
mov cx, (kernel_end - kernel_start) / SECTOR_SIZE
mov ax, cx
call disk_load

jmp enter_protected_mode

%include "Bootloader/Stage-2/gdt.asm"

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

; using fast a20 gate
enable_a20_line:
    in al, 0x92
    or al, 2
    out 0x92, al
    ret

[bits 32]

%include "Bootloader/Stage-2/identical_paging.asm"
%include "Bootloader/Stage-2/cpuid.asm"

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

%include "Bootloader/Stage-2/elf.asm"

move_to_64_bit_long_mode:
    mov rbx, kernel_start
    call parse_elf
    
    jmp rax ; jump to the kernel entry point
    
    jmp $

STAGE2_MSG db "On stage2!", 0
