
detect_cpuid:
; Check if CPUID is supported by attempting to flip the ID bit (bit 21) in
    ; the FLAGS register. If we can flip it, CPUID is available.
 
    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax
 
    ; Copy to ECX as well for comparing later on
    mov ecx, eax
 
    ; Flip the ID bit
    xor eax, 1 << 21
 
    ; Copy EAX to FLAGS via the stack
    push eax
    popfd
 
    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax
 
    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the ID bit
    ; back if it was ever flipped).
    push ecx
    popfd
 
    ; Compare EAX and ECX. If they are equal then that means the bit wasn't
    ; flipped, and CPUID isn't supported.
    xor eax, ecx
    jz .no_cpuid
    ret

    .no_cpuid:
    mov ebx, CPUID_ISNT_SUPPORTED_MSG
    call print_string
    jmp $

detect_long_mode:

    ; detect if the 0x80000001 function is supported
    mov eax, 0x80000000
    cpuid  
    cmp eax, 0x80000001
    jb .no_long_mode

    ; detect if the long mode bit is enable (long mode supported)
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    ret

    .no_long_mode:
    mov ebx, LONG_MODE_ISNT_SUPPORTED_MSG
    call print_string
    jmp $


CPUID_ISNT_SUPPORTED_MSG db "cpuid isn't supported!", 0
LONG_MODE_ISNT_SUPPORTED_MSG db "long mode isn't supported!", 0
