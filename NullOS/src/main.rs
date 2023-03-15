#![feature(rustc_private)]
#![allow(non_snake_case)]
#![feature(abi_x86_interrupt)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points (main isnt needed because our entry point is _start)
use core::panic::PanicInfo;

use crate::strings::print_string;

#[macro_use]
pub mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod keyboard;
pub mod strings;
pub mod shell;

static OS_NAME: &str = "NullOS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to {}!", OS_NAME);

    // gdt::init();
    interrupts::init_idt();
    
    // Check for Breakpoint Exception
    // x86_64::instructions::interrupts::int3();

    // Check for Page Fault -> Double Fault Exception
    // unsafe { *(0xdeadbeef as *mut u64) = 42; };

    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    shell::start_shell();


    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
