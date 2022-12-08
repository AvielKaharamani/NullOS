
VGA_MEMORY_BUFFER equ 0xb8000

print_string:
    push ebx
    push esi
    push eax

    mov esi, MEMORY_INDEX

.repeat:
    mov al, [ebx]
    cmp al, 0
    je .exit
    mov [VGA_MEMORY_BUFFER + esi], al
    inc ebx
    add esi, 2
    jmp .repeat
.exit:
    mov MEMORY_INDEX, esi

    pop eax
    pop esi
    pop ebx
    ret

MEMORY_INDEX dw 0
