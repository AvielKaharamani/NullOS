use crate::{keyboard};
use alloc::vec::Vec;
use crate::vga_buffer::clear_screen;

pub fn start_shell() {
    help(Vec::new());

    loop {
        print!(">> ");
        let input = keyboard::get_string();
        let commands: Vec<&str> = input.trim().split(' ').collect();

        
        let command_name = commands[0];
        let args = commands[1..].to_vec();

        if command_name.is_empty() {
            continue;
        }

        match command_name {
            "help" => help(args),
            "timer" => timer(args),
            "echo" => echo(args),
            "clear" | "cls" => clear(args),
            "ls" => ls(args),
            "cat" => cat(args),
            "mkdir" => mkdir(args),
            "rm" => rm(args),
            "cd" => mkdir(args),
            "pwd" => pwd(args),
            "snake" => snake(args),
            _ => println!("Unsupported command!")
        }
    }
}

fn help(args: Vec<&str>) {
    println!("Available commands:\n\thelp\n\ttimer\n\techo\n\tclear\n\tls\n\tcat\n\tmkdir\n\trm\n\tcd\n\tpwd\n\tsnake");
}

pub static mut SAVED_TICK_COUNTER: u64 = 0;

fn timer(args: Vec<&str>) {
    use crate::interrupts;
    match args[0] {
        "start" => unsafe {
            SAVED_TICK_COUNTER = interrupts::TICK_COUNTER;
            println!("Timer started!");
        },
        "stop" => unsafe {
            if SAVED_TICK_COUNTER == 0 {
                println!("You must first start the timer!");
                return;
            }
            let delta = interrupts::TICK_COUNTER - SAVED_TICK_COUNTER;
            println!("Time taken is: {} seconds!", delta / 20);
            SAVED_TICK_COUNTER = 0;
        },
        _ => println!("Unsupported parameter!")
    }
}

fn echo(args: Vec<&str>) {
    args.iter().for_each(|arg| print!("{} ", arg));
    println!("");
}

fn clear(args: Vec<&str>) {
    clear_screen();
}

fn ls(args: Vec<&str>) {
    println!("ls function");
}

fn cat(args: Vec<&str>) {
    println!("cat function");
}

fn mkdir(args: Vec<&str>) {
    println!("mkdir function");
}

fn rm(args: Vec<&str>) {
    println!("rm function");
}

fn cd(args: Vec<&str>) {
    println!("cd function");
}

fn pwd(args: Vec<&str>) {
    println!("pwd function");
}

fn snake(args: Vec<&str>) {
    println!("snakeyyy function!");
}