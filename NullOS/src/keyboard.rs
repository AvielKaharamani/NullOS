use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

pub fn read_scancode_value() -> u8 {
    use x86_64::instructions::port::Port;
    use x86_64::instructions::port::ReadWriteAccess;
    use x86_64::instructions::port::PortGeneric;
    let mut keyboard_state_port: PortGeneric<u8, ReadWriteAccess> = Port::new(0x64);
    let mut key_port: PortGeneric<u8, ReadWriteAccess> = Port::new(0x60);

    unsafe {
        while keyboard_state_port.read() & 1 != 1 {}
        key_port.read()
    }
}

pub fn get_char() -> Option<DecodedKey> {
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let scancode: u8 = read_scancode_value();


    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            return Some(key);
        }
    }
    None
}

pub fn get_string(buff: &mut [char; 80]) {
    set_boundery!();
    #[allow(unused_assignments)]
    let mut i = 0;
    loop {
        match get_char() {
            Some(DecodedKey::Unicode(character)) => {
                print!("{}", character);
                if character == '\n' {
                    break;
                } else if character == '\x08' {
                    if i > 0 {
                        i -= 1;
                    }
                } else {
                    buff[i] = character;
                    i += 1;
                }
            },
            _ => print!("")
        }
    }
    buff[i] = '\0';
}