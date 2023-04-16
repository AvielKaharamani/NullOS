PML4_ENTRY equ 0x1000
PDPT_ENTRY equ 0x2000
PDT_ENTRY  equ 0x3000
PAGE_TABLE_ENTRIES equ 512
PAGE_SIZE equ 0x1000 ; 4096
HUGE_PAGE_SIZE equ 0x200000 ; 2 MiB
PAGING_ENTRY_SIZE equ 8

FIRST_MAPPED_PAGE equ 0

setup_identical_paging:
    mov al, 0
    mov edi, 0x1000
    mov ecx, 0x3000
    
    cld
    rep stosb

    mov edi, PML4_ENTRY
    mov cr3, edi
    mov dword [PML4_ENTRY], PDPT_ENTRY + 0b11 ; (present bit, write bit)
    mov dword [PDPT_ENTRY], PDT_ENTRY + 0b11 ; (present bit, write bit)
    
    mov edi, PDT_ENTRY
    mov ebx, FIRST_MAPPED_PAGE + 0b10000011 ; (present bit, write bit, huge paging)
    mov ecx, PAGE_TABLE_ENTRIES

    ; mapped 512 HUGE pages from 0 to 1 GiB
    .set_entry:
    mov dword [edi], ebx
    add ebx, HUGE_PAGE_SIZE
    add edi, PAGING_ENTRY_SIZE
    loop .set_entry

    ret