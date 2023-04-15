use crate::{gdt, hlt_loop};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    AtaPrimary = PIC_1_OFFSET + 0xE,
    AtaSecondary
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt[InterruptIndex::AtaPrimary.as_usize()].set_handler_fn(primary_ata_handler);
        idt[InterruptIndex::AtaSecondary.as_usize()].set_handler_fn(secondary_ata_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

pub const TIMER_FREQUENCY: u64 = 1000;

pub fn init_timer() {
    use x86_64::instructions::port::Port;
    use x86_64::instructions::port::ReadWriteAccess;
    use x86_64::instructions::port::PortGeneric;
    let divisor = 1193180 / TIMER_FREQUENCY;

    let mut port: PortGeneric<u8, ReadWriteAccess>  = Port::new(0x43);
    
    // Send the command byte.
    unsafe { port.write(0x36); }

    // Divisor has to be sent byte-wise, so split here into upper/lower bytes.
    let divisor_low = (divisor & 0xFF) as u8;
    let divisor_high = ((divisor >> 8) & 0xFF) as u8;

    // Send the frequency divisor.
    let mut frequency_port: PortGeneric<u8, ReadWriteAccess> = Port::new(0x40);

    unsafe {
        frequency_port.write(divisor_low);
        frequency_port.write(divisor_high);
    }
}

pub static mut TICK_COUNTER: u64 = 0;

pub fn sleep(mil: u64) {
    unsafe {
        let saved_tick_counter = TICK_COUNTER;
        while saved_tick_counter + mil >= TICK_COUNTER {}
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        TICK_COUNTER += 1;

        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn primary_ata_handler(_stack_frame: InterruptStackFrame ) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::AtaPrimary.as_u8());
    }
}

extern "x86-interrupt" fn secondary_ata_handler(_stack_frame: InterruptStackFrame ) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::AtaSecondary.as_u8());
    }
}
