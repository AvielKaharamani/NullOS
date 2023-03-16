use crate::{keyboard, strings};


pub fn start_shell() {
    const BUFF_SIZE: usize = 80;
    let mut buff: [char; BUFF_SIZE] = ['\0'; BUFF_SIZE];
    
    loop {
        print!("Enter your name: ");
        keyboard::get_string(&mut buff);
        
        

        strings::print_string(&buff);
        println!(" Is best!");
    }
}