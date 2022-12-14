; using fast a20 gate
enable_a20_line:
    in al, 0x92
    or al, 2
    out 0x92, al
    ret
