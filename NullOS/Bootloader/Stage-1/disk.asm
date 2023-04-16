MAX_SECTORS_IN_ONE_TIME equ 127
SECTOR_SIZE equ 512
SEGMENT_SHIFT equ 4

; dx = address of the base segment
; cx = number of sectors to read
; bx = memory offset
; eax = begin reading address
disk_load:

    ; Read disk function from (int 0x13)
    cmp cx, MAX_SECTORS_IN_ONE_TIME
    ja .above_max_sectors_in_one_time

    mov [DAP.sectors_to_load], cx
    mov [DAP.offset], bx
    mov [DAP.segment], dx
    mov [DAP.address], eax

    mov dl, [CURR_DISK]
    mov si, DAP
    mov ah, 0x42
    ; https://en.wikipedia.org/wiki/INT_13H#INT_13h_AH=42h:_Extended_Read_Sectors_From_Drive
    int 0x13

    ; if int 0x13 reading function fails carry flag turns on
    jc .falied
    ret

.falied:
    ; if failed print the error message and freeze
    mov bx, DISK_ERROR_MSG
    call print_string
    jmp $

.above_max_sectors_in_one_time:

    pusha
    mov cx, MAX_SECTORS_IN_ONE_TIME
    call disk_load
    popa

    add eax, MAX_SECTORS_IN_ONE_TIME
    sub cx, MAX_SECTORS_IN_ONE_TIME
    add dx, (MAX_SECTORS_IN_ONE_TIME * SECTOR_SIZE) >> SEGMENT_SHIFT


    jmp disk_load


DAP:
    db 0x10
    db 0
.sectors_to_load: ; Number of sectors to load, max is 127 on most bioses
    dw 127 
.offset: ; Where to load the values
    dw 0x0
.segment: 
    dw 0x0
.address: ; Where to read from
    dq 0x0

CURR_DISK db 0
DISK_ERROR_MSG db "Failed reading from the disk", 0
