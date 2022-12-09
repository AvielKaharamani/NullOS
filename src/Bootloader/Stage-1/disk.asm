
STAGE2_ADDR equ 0x7e00

load_stage2_from_disk:
    ; Read disk function from (int 0x13)
    mov ah, 0x2

    mov dl, [CURR_DISK] ; the disk we read from
    mov al, 4 ; the amount of sector to read
    mov ch, 0 ; the cylinder we read from
    mov dh, 0 ; the head we read from
    mov cl, 2 ; the sector we read from (starting right after our bootloader) 


    ; Ram address for stage2 code
    mov bx, 0
    mov es, bx
    mov bx, STAGE2_ADDR
    
    ; https://stanislavs.org/helppc/int_13.html
    int 0x13

    ; if int 0x13 reading function fails carry flag turns on
    jnc .success

    ; if failed print the error message and freeze
    mov bx, DISK_ERROR_MSG
    call print_string
    jmp $

    .success:
    ret

CURR_DISK db 0
DISK_ERROR_MSG db "Failed reading stage2 from the disk", 0
