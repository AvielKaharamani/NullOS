
gdt_null_descriptor:
    dd 0
    dd 0
gdt_code_descriptor:
    dw 0xffff       ; limit
    dw 0x0000       ; base (low)
    db 0x00         ; base (medium)
    db 0b10011010   ; access flags
    db 0b11001111   ; flags + upper limit
    db 0x00         ; base (high)
; the only difference is the executable access bit
gdt_data_descriptor:
    dw 0xffff       ; limit
    dw 0x0000       ; base (low)
    db 0x00         ; base (medium)
    db 0b10010010   ; access flags
    db 0b11001111   ; flags + upper limit
    db 0x00         ; base (high)
    gdt_end:

gdt_descriptor:
    gdt_size: dw gdt_end - gdt_null_descriptor - 1
    gdt_offset: dq gdt_null_descriptor

code_segment equ gdt_code_descriptor - gdt_null_descriptor
data_segment equ gdt_data_descriptor - gdt_null_descriptor

[bits 32]

fit_gdt_long_mode:
    ; enable long mode flag and disable size flag (because long mode enforce the size flag to be cleared)

    mov byte [gdt_code_descriptor + 6], 0b10101111

    mov byte [gdt_data_descriptor + 6], 0b10101111
    ret

[bits 16]
