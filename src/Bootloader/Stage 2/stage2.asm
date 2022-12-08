org 0x7e00

mov bx, STAGE2_SUCCESS_MSG
call print_string

jmp $

%include "Utils/strings.asm"

STAGE2_SUCCESS_MSG db "stage2 loaded successfully!", 0

times 2048 - ($ - $$) db 0
