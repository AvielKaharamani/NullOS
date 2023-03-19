use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
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
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

pub fn init_timer() {
    use x86_64::instructions::port::Port;
    use x86_64::instructions::port::ReadWriteAccess;
    use x86_64::instructions::port::PortGeneric;
    let frequency = 20;
    let divisor = 1193180 / frequency;

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

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Can evaluate time logic
    unsafe { TICK_COUNTER += 1; }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1, KeyCode};
    // use spin::Mutex;
    // use x86_64::instructions::port::Port;

    // lazy_static! {
    //     static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
    //         Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    //     );
    // }

    // let mut keyboard = KEYBOARD.lock();
    // let mut port = Port::new(0x60);

    // let scancode: u8 = unsafe { port.read() };
    // if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
    //     if let Some(key) = keyboard.process_keyevent(key_event) {
    //         match key {
    //             DecodedKey::Unicode(character) => {
    //                 if (character as u16 == 27) {
    //                     print!("Escape!");
    //                 } else {
    //                 print!("{}", character as u16)

    //                 }
    //             },
    //             DecodedKey::RawKey(key) => print!("{:?}", key),
    //                 }
    //             }
    //         }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
