#![no_std]
#![no_main]

#[macro_use]
pub mod vga_buffer;

use null_os::shell::Shell;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    null_os::init(&boot_info);
    println!("NullOS is now loading...");

    let mut shell = Shell::new();

    shell.start_shell();
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    null_os::hlt_loop();
}
