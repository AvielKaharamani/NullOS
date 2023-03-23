use crate::{keyboard::{self, get_string, get_password}};
use alloc::vec::Vec;
use alloc::string::String;
use crate::vga_buffer::clear_screen;

struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: String::from(username),
            password: String::from(password),
        }
    }
}

pub struct Shell {
    saved_tick_counter: u64,
    users: Vec<User>,
    curr_user_index: usize
}

impl Shell {
    pub fn new() -> Shell {
        let root_user = User::new("root", "root");
        let mut users = Vec::new();
        users.push(root_user);

        Shell {saved_tick_counter: 0, users: users, curr_user_index: 0}
    }

    pub fn start_shell(&mut self) {
        self.login();
        
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
                "help" => self.help(),
                "timer" => self.timer(args),
                "echo" => self.echo(args),
                "clear" | "cls" => self.clear(),
                "ls" => self.ls(args),
                "cat" => self.cat(args),
                "mkdir" => self.mkdir(args),
                "touch" => self.touch(args),
                "edit" => self.edit(args),
                "rm" => self.rm(args),
                "cd" => self.cd(args),
                "pwd" => self.pwd(),
                "adduser" => self.adduser(args),
                "deluser" => self.deluser(args),
                "lsusers" => self.lsusers(),
                "logout" => self.login(),
                _ => println!("Unsupported command! type 'help' for more explanation")
            }
        }
    }
    
    fn help(&self) {
        println!("Available commands:\n\thelp\n\ttimer\n\techo\n\tclear\n\tls\n\tcat\n\tmkdir\n\ttouch\n\tedit\n\trm\n\tcd\n\tpwd\n\tsnake\n\tadduser\n\tdeluser\n\tlsusers\n\tlogout");
    }
    
    fn timer(&mut self, args: Vec<&str>) {
        use crate::interrupts;
        match args[0] {
            "start" => unsafe {
                self.saved_tick_counter = interrupts::TICK_COUNTER;
                println!("Timer started!");
            },
            "stop" => unsafe {
                if self.saved_tick_counter == 0 {
                    println!("You must first start the timer!");
                    return;
                }
                let delta = interrupts::TICK_COUNTER - self.saved_tick_counter;
                println!("Time taken is: {} seconds!", delta / 20);
                self.saved_tick_counter = 0;
            },
            _ => println!("Usage: timer [start/stop]")
        }
    }
    
    fn echo(&self, args: Vec<&str>) {
        args.iter().for_each(|arg| print!("{} ", arg));
        println!("");
    }
    
    fn clear(&self) {
        clear_screen();
    }
    
    fn ls(&self, args: Vec<&str>) {
        println!("ls function");
    }
    
    fn cat(&self, args: Vec<&str>) {
        println!("cat function");
    }
    
    fn mkdir(&self, args: Vec<&str>) {
        println!("mkdir function");
    }
    
    fn touch(&self, args: Vec<&str>) {
        println!("touch function");
    }
    
    fn edit(&self, args: Vec<&str>) {
        println!("edit function");
    }

    fn rm(&self, args: Vec<&str>) {
        println!("rm function");
    }
    
    fn cd(&self, args: Vec<&str>) {
        println!("cd function");
    }
    
    fn pwd(&self) {
        println!("pwd function");
    }

    fn adduser(&mut self, args: Vec<&str>) {
        if args.len() != 2 {
            println!("Usage: adduser [username] [password]");
            return;
        }

        let username = args[0];
        let password = args[1];
        let mut user_exists = false;

        for u in self.users.iter() {
            if u.username == username {
                user_exists = true;
                break;
            }
        }

        if user_exists {
            println!("The user: '{}' already exists!", username);
            return;
        }

        self.users.push(User::new(username, password));
        println!("Created user!");
    }

    fn deluser(&mut self, args:Vec<&str>) {
        if args.len() != 1 {
            println!("Usage: deluser [username]");
            return;
        }

        let username = args[0];
        let mut user_index = 0;
        let mut user_exists = false;
        
        if username == self.users[0].username {
            println!("Cannot delete root user!");
            return;
        }

        if username == self.users[self.curr_user_index].username {
            println!("Cannot delete yourself!");
            return;
        }

        for (i, u) in self.users.iter().enumerate() {
            if u.username == username {
                user_exists = true;
                user_index = i;
                break;
            }
        }

        if !user_exists {
            println!("The user: '{}' does not exist!", username);
            return;
        }
        
        print!("root password: ");
        let root_password = get_password();

        if root_password != self.users[0].password {
            println!("root password is incorrect!");
            return;
        }

        self.users.remove(user_index);
        println!("Deleted user!");
    }

    fn lsusers(&self) {
        println!("Users({}):",self.users.len());

        for user in self.users.iter() {
            println!("\t- {}", user.username);
        }
    }

    fn login(&mut self) {
        let mut is_logged = false;
        let mut username = String::from("");

        clear_screen();

        while !is_logged {
            print!("Username: ");
            username = get_string();
            print!("Password: ");
            let password = get_password();
            
            for (i, user) in self.users.iter().enumerate() {
                if user.username == username && user.password == password {
                    is_logged = true;
                    self.curr_user_index = i;
                    break;
                }
            }

            if !is_logged {
                println!("Incorrect Username or Password, please try again\n");
            }
        }
        println!("Welcome to NullOS!");
        println!("Logged as: '{}'\n", username);
    }
}
