#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
pub mod vga_buffer;

pub mod allocator;
pub mod keyboard;
pub mod ata;
pub mod reader_writer;
pub mod inode;
pub mod file_system;
pub mod shell;
pub mod gdt;
pub mod interrupts;
pub mod memory;
use bootloader::{BootInfo};
use memory::BootInfoFrameAllocator;
use x86_64::VirtAddr;

pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::init_idt();
    interrupts::init_timer();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
