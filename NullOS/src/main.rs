#![feature(rustc_private)]
#![allow(non_snake_case)]
#![feature(abi_x86_interrupt)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points (main isnt needed because our entry point is _start)
use core::panic::PanicInfo;
use core::primitive::str;

#[macro_use] // vec! macro
pub mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod keyboard;

static OS_NAME: &str = "NullOS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to {}!", OS_NAME);

    // gdt::init();
    interrupts::init_idt();

    // x86_64::instructions::interrupts::int3();

    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    const BUFF_SIZE: usize = 80;
    let mut buff: [char; BUFF_SIZE] = ['\0'; BUFF_SIZE];
    keyboard::get_string(&mut buff);

    let mut i = 0;
    while buff[i] != '\0' {
        print!("{}", buff[i]);
        i += 1;
    }
        
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
