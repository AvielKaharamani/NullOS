stage2_entry:
    mov bx, STAGE2_MSG
    call print_string_with_new_line

    call enable_a20_line
    call enter_unreal_mode


    mov edi, KERNEL_MEMORY_ADDRESS
    call load_kernel

    mov bx, LOAD_KERNEL_MSG
    call print_string_with_new_line

    jmp enter_protected_mode

%include "Bootloader/Stage-2/unreal_mode.asm"
%include "Bootloader/Stage-2/a20_line.asm"
%include "Bootloader/Stage-2/gdt.asm"
%include "Bootloader/Stage-2/load_kernel.asm"

enter_protected_mode:

    ; disable interupts before entering protected mode
    cli
    
    ; enable the protected mode bit in the cr0 (tell the cpu to enter protected mode)
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    ; far jump (to update the code segment 'cs')
    jmp code_segment:move_to_32_protected_mode

[bits 32]

%include "Bootloader/Stage-2/identical_paging.asm"
%include "Bootloader/Stage-2/cpuid.asm"

move_to_32_protected_mode:
    ; setting up all the data segments to data 
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
    mov rbp, 0x9fc00
	mov rsp, rbp

    mov rax, KERNEL_MEMORY_ADDRESS
    jmp rax ; jump to the kernel entry point

STAGE2_MSG db "On stage2!", 0
LOAD_KERNEL_MSG db "Kernel loaded to memory", 0
