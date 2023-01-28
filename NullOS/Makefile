release: build_release only_run
debug: build_debug only_run

gdb: build_debug only_gdb

build_release: build_kernel_release build_disk
build_debug: build_kernel_debug build_disk

build_kernel_release:
	@RUSTFLAGS="-C link-arg=-T/Users/orenope/Documents/ashdod-604-nullos/NullOS/layout.ld" RUST_TARGET_PATH=$(shell pwd) xargo build --release --target x86_64-NullOS
	x86_64-pc-elf-objcopy -O binary target/x86_64-NullOS/release/NullOS NullOS.bin
	x86_64-pc-elf-readelf -h target/x86_64-NullOS/release/NullOS | grep "Entry point address" | sed 's/Entry point address: //' | sed 's/^[ \t]*//' | python3 -c 'import struct; val = struct.pack("<Q", int(input(), 16)); f = open("entry_point", "wb"); f.write(val); f.close()'

build_kernel_debug:
	@RUSTFLAGS="-C link-arg=-T/Users/orenope/Documents/ashdod-604-nullos/NullOS/layout.ld" RUST_TARGET_PATH=$(shell pwd) xargo build --target x86_64-NullOS
	x86_64-pc-elf-objcopy -O binary target/x86_64-NullOS/debug/NullOS NullOS.bin
	x86_64-pc-elf-readelf -h target/x86_64-NullOS/debug/NullOS | grep "Entry point address" | sed 's/Entry point address: //' | sed 's/^[ \t]*//' | python3 -c 'import struct; val = struct.pack("<Q", int(input(), 16)); f = open("entry_point", "wb"); f.write(val); f.close()'


build_disk:
	nasm Bootloader/bootloader.asm -f bin -o disk.flp

only_run:
	qemu-system-x86_64 -drive file=disk.flp,format=raw

only_debug:
	qemu-system-x86_64 -drive file=disk.flp,format=raw -monitor stdio

only_gdb:
	qemu-system-x86_64 -S -s -drive file=disk.flp,format=raw

clean:
	rm -f NullOS.bin NullOS.elf *.flp
	xargo clean
