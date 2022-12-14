print_char:
    push ax
    mov ah, 0eh
    int 10h
    pop ax
    ret

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

print_line:
    push ax
    mov al, 13
    call print_char
    mov al, 10
    call print_char
    pop ax
    ret

print_string_with_new_line:
    call print_string
    call print_line
    ret

print_num:
    pusha
    xor cx, cx
.for_each_digit:
    xor dx, dx
    mov bx, 10
    div bx
    push dx
    inc cx
    cmp ax, 0
    jne .for_each_digit

.print_digit:
    pop ax
    mov ah, 0xe
    add al, '0'
    int 0x10
    loop .print_digit
    popa
    ret

clear_screen:
    push ax
    mov ah, 0x00 ; change graphics mode
    mov al, 0x03 ; selected mode (colored text mode)
    int 0x10
    pop ax
    ret