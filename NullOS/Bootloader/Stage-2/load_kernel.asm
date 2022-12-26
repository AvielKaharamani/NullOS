KERNEL_MEMORY_ADDRESS equ 0x100000 ; 1 MiB

; edi = where to store the kernel in memory
load_kernel:
    mov bx, kernel_start ; bx = memory offset 
    mov dx, cs ; dx = address of the base segment
    mov eax, (kernel_start - stage1_start) / SECTOR_SIZE ; eax = sectors offset
    xor ecx, ecx
    mov cx, (kernel_end - kernel_start) / SECTOR_SIZE ; cx = number of sectors to read

.foreach_kernel_part:
    
    cmp cx, MAX_SECTORS_IN_ONE_TIME
    jbe .load_curr_kernel_part

    push cx
    mov cx, MAX_SECTORS_IN_ONE_TIME
    call .load_curr_kernel_part
    pop cx
    add eax, MAX_SECTORS_IN_ONE_TIME ; sectors offset
    sub cx, MAX_SECTORS_IN_ONE_TIME

    jmp .foreach_kernel_part
    
.load_curr_kernel_part:
    test cx, cx
    jz .exit

    call disk_load

    shl ecx, 9 ; ecx = ecx * 512

    xor esi, esi
    mov si, bx
    cld
    a32 rep movsb

.exit:
    ret
