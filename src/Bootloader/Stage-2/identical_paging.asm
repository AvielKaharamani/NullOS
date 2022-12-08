
PAGING_ENTRY 0x1000

setup_identical_paging:
    mov edi, PAGING_ENTRY
    mov cr3, edi
    mov dword [edi], 0x2003
    add edi, 0x1000
    mov dword [edi], 0x3003
    add edi, 0x1000
    mov dword [edi], 0x4003
    add edi, 0x1000

    mov ebx, 0x00000003
    mov ecx. 512

    .set_entry:
    mov dword [edi], ebx
    add ebx, 0x1000
    add edi, 8
    loop .set_entry

    mov

