#![allow(non_snake_case)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points (main isnt needed because our entry point is _start)


use core::panic::PanicInfo;

static WELCOME_MSG: &[u8] = b"Hello, from NullOS!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_memory = 0xb8000 as *mut u8;

    // printing welcome message
    for (i, &ch) in WELCOME_MSG.iter().enumerate() {
        unsafe {
            *vga_memory.add(i * 2) = ch;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
