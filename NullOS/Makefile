
# Linker
ifeq ($(shell uname),Darwin)
  LD = x86_64-pc-elf-ld
else
  LD = ld
endif

build: build_kernel build_disk

build_kernel:
	cargo build
	$(LD) -n -o NullOS.bin -T linker.ld target/x86_64-NullOS/debug/NullOS

build_disk:
	nasm Bootloader/bootloader.asm -f bin -o disk.flp

run:
	qemu-system-x86_64 -drive file=disk.flp,format=raw

clean:
	rm -f *.bin *.flp
