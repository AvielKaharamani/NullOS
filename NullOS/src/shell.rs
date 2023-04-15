use crate::{keyboard::{self, get_string, get_password}, interrupts, inode::InodeType};
use alloc::{vec::Vec};
use alloc::string::String;
use crate::vga_buffer::{clear_screen, clear_row};
use crate::file_system::FileSystem;
use crate::reader_writer::ReaderWriter;
use crate::ata::Disk;

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
    users: Vec<User>,
    curr_user_index: usize,
    file_system: FileSystem,
}

pub fn update_percentage(percent: u32) {
    clear_row(1);
    print!("{}%", percent);
}

impl Shell {
    pub fn new() -> Shell {
        let root_user = User::new("root", "root");
        let mut users = Vec::new();
        users.push(root_user);

        Shell {users: users, curr_user_index: 0, file_system: FileSystem::new(ReaderWriter::new(Disk::new()))}
    }

    pub fn start_shell(&mut self) -> ! {
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
                "sleep" => self.sleep(args),
                "echo" => self.echo(args),
                "clear" | "cls" => self.clear(),
                "ls" => self.ls(args),
                "cat" => self.cat(args),
                "mkdir" => self.mkdir(args),
                "touch" => self.touch(args),
                "edit" => self.edit(args),
                "rm" => self.rm(args),
                "adduser" => self.adduser(args),
                "deluser" => self.deluser(args),
                "lsusers" => self.lsusers(),
                "logout" => self.login(),
                _ => println!("Unsupported command! type 'help' for more explanation")
            }
        }
    }

    fn help(&self) {
        println!("Available commands:\n\thelp\n\tsleep\n\techo\n\tclear / cls\n\tls\n\tcat\n\tmkdir\n\ttouch\n\tedit\n\trm\n\tadduser\n\tdeluser\n\tlsusers\n\tlogout");
    }
    
    fn sleep(&mut self, args: Vec<&str>) {
        if args.len() != 1 {
            println!("Usage: sleep [sec]")
        }

        let sec: u64 = args[0].parse().unwrap_or_default();

        interrupts::sleep(1000 * sec);
    }
    
    fn echo(&self, args: Vec<&str>) {
        args.iter().for_each(|arg| print!("{} ", arg));
        println!("");
    }
    
    pub fn clear(&self) {
        clear_screen();
    }
    
    fn ls(&mut self, args: Vec<&str>) {
        if args.len() > 1 {
            println!("Usage: ls [path]");
            return;
        }

        let path = if args.len() == 1 { args[0] } else { "/" };
        
        match self.file_system.get_type_by_path(path) {
            InodeType::Dir => {},
            _ => {
                println!("'{}' is not a dir", path);
                return;
            }
        };

        let inode_index = self.file_system.get_inode_index_from_path(path, 0).unwrap();
        for (file_name, _) in self.file_system.get_entries_from_dir(inode_index) {
            println!("{}", file_name);
        }
    }
    
    fn cat(&mut self, args: Vec<&str>) {
        if args.len() != 1 {
            println!("Usage: cat [file_name]");
            return;
        }

        match self.file_system.get_type_by_path(args[0]) {
            InodeType::File => {},
            _ => {
                println!("'{}' is not a file", args[0]);
                return;
            }
        };

        let file_inode_index = self.file_system.get_inode_index_from_path(args[0], 0).unwrap();

        let content_vec = self.file_system.get_content_by_inode_index(file_inode_index).to_vec();

        let content = String::from_utf8(content_vec).expect("Found invalid UTF-8");

        if !content.is_empty() {
            println!("{}", content);
        }
    }
    
    fn mkdir(&mut self, args: Vec<&str>) {
        if args.len() != 1 {
            println!("Usage: mkdir [dir_name]");
            return;
        }

        let (base_dir, entry) = self.file_system.path_to_base_dir_and_entry(args[0]);

        match self.file_system.get_type_by_path(base_dir) {
            InodeType::Dir => {},
            _ => {
                println!("'{}' is not a dir", base_dir);
                return;
            }
        };

        let base_dir_inode_index = self.file_system.get_inode_index_from_path(base_dir, 0).unwrap();

        self.file_system.create_dir_entry(entry, true, base_dir_inode_index);
    }
    
    fn touch(&mut self, args: Vec<&str>) {
        if args.len() != 1 {
            println!("Usage: touch [file_name]");
            return;
        }

        let (base_dir, entry) = self.file_system.path_to_base_dir_and_entry(args[0]);

        match self.file_system.get_type_by_path(base_dir) {
            InodeType::Dir => {},
            _ => {
                println!("'{}' is not a dir", base_dir);
                return;
            }
        };

        let base_dir_inode_index = self.file_system.get_inode_index_from_path(base_dir, 0).unwrap();

        self.file_system.create_dir_entry(entry, false, base_dir_inode_index);
    }
    
    fn edit(&mut self, args: Vec<&str>) {
        if args.len() != 1 {
            println!("Usage: edit [file_name]");
            return;
        }

        match self.file_system.get_type_by_path(args[0]) {
            InodeType::File => {},
            _ => {
                println!("'{}' is not a file", args[0]);
                return;
            }
        };

        let mut content = get_string();
        let content_vec = unsafe {content.as_mut_vec()};
        let file_inode_index = self.file_system.get_inode_index_from_path(args[0], 0).unwrap();

        self.file_system.set_content_by_inode_index(file_inode_index, content_vec.to_vec());
    }

    fn rm(&mut self, args: Vec<&str>) {
        if args.len() != 1 {
            println!("Usage: rm [file_name]");
            return;
        }

        match self.file_system.get_type_by_path(args[0]) {
            InodeType::File => {},
            _ => {
                println!("'{}' is not a file", args[0]);
                return;
            }
        };

        let (base_dir, entry) = self.file_system.path_to_base_dir_and_entry(args[0]);

        let base_dir_inode_index = self.file_system.get_inode_index_from_path(base_dir, 0).unwrap();

        let mut entries = self.file_system.get_entries_from_dir(base_dir_inode_index);
        entries.remove(entry);
        self.file_system.set_files_to_dir(&entries, base_dir_inode_index);
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
        println!("Login screen:");

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

        clear_screen();
        println!("Welcome to NullOS!");
        println!("Logged as: '{}'\n", username);
    }
}
