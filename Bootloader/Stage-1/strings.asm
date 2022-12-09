
print_string:
    push bx
    push ax
    
    mov ah, 0xe
.repeat:
    mov al, [bx]
    cmp al, 0
    je .exit
    int 0x10
    inc bx
    jmp .repeat
.exit:
    pop ax
    pop bx
    ret

clear_screen:
    push ax
    mov ah, 0x00 ; change graphics mode
    mov al, 0x03 ; selected mode (colored text mode)
    int 0x10
    pop ax
    ret