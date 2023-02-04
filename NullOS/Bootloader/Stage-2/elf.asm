ELF_HEADER_OFFSET equ 0x20
ELF_ENTRY_POINT_OFFSET equ 0x18
PROGRAM_SIZE_HEADER_OFFSET equ 0x20
SEGMENT_OFFSET_HEADER_OFFSET equ 0x08
MEMORY_ADDRESS_HEADER_OFFSET equ 0x10

; rbx - base address of elf
; return: rax - The entry point of the program
parse_elf:
    mov rdx, [rbx + ELF_HEADER_OFFSET] ; get the header start offset
    add rdx, rbx ; get the header start address

    mov rcx, [rdx + PROGRAM_SIZE_HEADER_OFFSET] ; p_filesz - size of the text segment
    mov rsi, [rdx + SEGMENT_OFFSET_HEADER_OFFSET] ;p_offset - offset of the text segment
    mov rdi, [rdx + MEMORY_ADDRESS_HEADER_OFFSET] ;p_vaddr - address of the program in memory

    cld
    rep movsb ; load program text to memory

    mov rax, [rbx + ELF_ENTRY_POINT_OFFSET] ; return the entry point to the program
    
    ret

; ELF_HEADER_OFFSET equ 0x20
; ELF_ENTRY_POINT_OFFSET equ 0x8

; ; rbx - base address of elf
; ; return: The entry point of the program
; get_elf_entry_point:
;     mov rdx, [rbx + ELF_HEADER_OFFSET]
;     add rdx, rbx
;     mov rax, rbx
;     add rax, [rdx + ELF_ENTRY_POINT_OFFSET]
    
;     ret