buildf:
	cargo build
	
	nasm Bootloader/bootloader.asm -f bin -o disk.flp

run:
	qemu-system-x86_64 -drive file=disk.flp,format=raw

clean:
	rm -f *.bin *.flp
