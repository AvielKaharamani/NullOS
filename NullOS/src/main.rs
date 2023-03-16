#![feature(rustc_private)]
#![feature(alloc_error_handler)]
#![allow(non_snake_case)]
#![feature(abi_x86_interrupt)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points (main isnt needed because our entry point is _start)

extern crate alloc;

use core::panic::PanicInfo;
use crate::strings::print_string;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

#[macro_use]
pub mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod keyboard;
pub mod strings;
pub mod shell;
pub mod allocator;

static OS_NAME: &str = "NullOS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to {}!", OS_NAME);

    // gdt::init();
    interrupts::init_idt();
    allocator::init_heap();
        
    // Check for Breakpoint Exception
    // x86_64::instructions::interrupts::int3();

    // Check for Page Fault -> Double Fault Exception at 1Gib address space
    // unsafe { *(0x40000000 as *mut u64) = 42; }; 

    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();


    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);   

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);   

    println!("It did not crash!");


    shell::start_shell();


    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
