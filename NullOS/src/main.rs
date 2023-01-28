#![feature(rustc_private)]
#![allow(non_snake_case)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points (main isnt needed because our entry point is _start)
use core::panic::PanicInfo;
extern crate compiler_builtins;

static WELCOME_MSG: &[u8] = b"Aviel king pro noder nader!";

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
    let vga_memory = 0xb8000 as *mut u8;

    unsafe {
        *vga_memory = 'P' as u8;
        *vga_memory.add(2) = 'a' as u8;
        *vga_memory.add(4) = 'n' as u8;
        *vga_memory.add(6) = 'i' as u8;
        *vga_memory.add(8) = 'c' as u8;
        *vga_memory.add(10) = '!' as u8;
        *vga_memory.add(12) = ' ' as u8;
    }
    loop {}
}
