org 0x7c00

; setting up stack
mov bp, 0x7c00
mov sp, bp

mov bx, testStr
call printString

testStr:
    db "Aviel the king!", 0

%include "string.asm"

times 510 - ($ - $$) db 0

dw 0xaa55