#![feature(rustc_private)]
#![allow(non_snake_case)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points (main isnt needed because our entry point is _start)
use core::panic::PanicInfo;

#[macro_use] // vec! macro
pub mod vga_buffer;

static WELCOME_MSG: &str = "Welcome to NullOS!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome msg: {}", WELCOME_MSG);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panic!");

    loop {}
}
