
PML4_ENTRY equ 0x1000
PDPT_ENTRY equ 0x2000
PDT_ENTRY  equ 0x3000
PT_ENTRY   equ 0x4000

FIRST_MAPPED_PAGE equ 0x00000000

setup_identical_paging:
    mov edi, PML4_ENTRY
    mov cr3, edi
    mov dword [PML4_ENTRY], 0x2003
    mov dword [PDPT_ENTRY], 0x3003
    mov dword [PDT_ENTRY], 0x4003
    
    mov edi, PT_ENTRY
    lea ebx, [FIRST_MAPPED_PAGE + 0b11] ; present bit and read write bit
    mov ecx, 512

    ; mapped 512 pages from 0 to 2 mega
    .set_entry:
    mov dword [edi], ebx
    add ebx, 0x1000
    add edi, 8
    loop .set_entry

    ret
