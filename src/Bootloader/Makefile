
build:
	nasm Stage-1/stage1.asm -f bin -o stage1.bin
	nasm Stage-2/stage2.asm -f bin -o stage2.bin
	cat stage1.bin stage2.bin > disk.flp

run:
	qemu-system-x86_64 -drive file=disk.flp,format=raw

clean:
	rm -f *.bin *.flp
