
printString:
    mov ah, 0xe
.repeat:
    mov al, [bx]
    cmp al, 0
    je .exit
    int 0x10
    inc bx
    jmp .repeat
.exit:
    ret
