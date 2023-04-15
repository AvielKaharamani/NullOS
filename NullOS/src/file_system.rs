use crate::interrupts::sleep;
use crate::reader_writer::{ReaderWriter, SeekFrom};
use crate::inode::{Inode, InodeType};
use alloc::vec::Vec;
use alloc::vec;
use core::cmp::min;
use core::mem;
use crate::alloc::borrow::ToOwned;
use alloc::string::String;
use hashbrown::HashMap;
use crate::shell::update_percentage;


const BLOCK_SIZE: usize = 512;
const INODE_SIZE: usize = mem::size_of::<Inode>();
#[allow(dead_code)]
const DISK_SIZE: usize = 1024 * 1024;
#[allow(dead_code)]
const INODE_TABLE_OFFSET: usize = 0;
const NUMBER_OF_INODES: usize = 512;

const BYTE_MAP_OFFSET: usize = NUMBER_OF_INODES * INODE_SIZE;
const BLOCKS_OFFSET: usize = BYTE_MAP_OFFSET + NUMBER_OF_BLOCKS;
const NUMBER_OF_BLOCKS: usize = 512 * 2;

pub struct FileSystem {
    reader_writer: ReaderWriter
}

pub enum BlockState {
    Free, Taken
}

// | inode table | byte map of blocks | blocks |

impl FileSystem {
    pub fn new(reader_writer: ReaderWriter) -> FileSystem {
        let mut file_system = FileSystem {reader_writer};

        let first_dir = Inode{inode_type: InodeType::Dir, content_length: 0, blocks_indexes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]};
        
        file_system.set_inode(first_dir, 0);

        println!("");
        update_percentage(0);

        
        for i in 1..NUMBER_OF_INODES {
            let default_inode = Inode{inode_type: InodeType::Free, content_length: 0, blocks_indexes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]};
            file_system.set_inode(default_inode, i);

            if i == NUMBER_OF_INODES / 2 {
                update_percentage(17);
            }
        }

        update_percentage(34);

        for i in 0..NUMBER_OF_BLOCKS {
            file_system.set_block_state(BlockState::Free, i);

            if i == NUMBER_OF_BLOCKS / 2 {
                update_percentage(67);
            }
        }

        update_percentage(100);
        sleep(100);

        file_system
    }

    // Inodes
    pub fn get_inode(&mut self, inode_index: usize) -> Inode {
        let mut buff = [0u8; INODE_SIZE];

        let offset = self.get_inode_offset_by_index(inode_index);
        self.reader_writer.seek(SeekFrom::Start(offset)).expect("failed to seek to block");
        
        self.reader_writer.read(&mut buff).expect("failed to read from block");
        let inode: Inode = unsafe { mem::transmute::<[u8; INODE_SIZE], Inode>(buff) };
        
        inode
    }

    pub fn set_inode(&mut self, inode: Inode, inode_index: usize) {
        let buff: [u8; INODE_SIZE] = unsafe { mem::transmute(inode) };
        let offset = self.get_inode_offset_by_index(inode_index);

        self.reader_writer.seek(SeekFrom::Start(offset)).expect("failed to seek to block");
        
        self.reader_writer.write(&buff).expect("failed to write to block");
    }

    pub fn get_content_by_inode_index(&mut self, inode_index: usize) -> Vec<u8> {
        let mut content = Vec::<u8>::new();
        let mut buff = [0u8; 1];
        let inode: Inode = self.get_inode(inode_index);
        let length = inode.content_length;
        let num_of_blocks = ceil(length as f32 / BLOCK_SIZE as f32);
        let mut counter = 0;

        for i in 0..num_of_blocks {
            let block_index = inode.blocks_indexes[i];
            let offset = self.get_block_offset_by_index(block_index);
            self.reader_writer.seek(SeekFrom::Start(offset)).expect("failed to seek to block");
            
            for _ in 0..BLOCK_SIZE {
                counter += 1;
                self.reader_writer.read(&mut buff).expect("failed to read from block");
                content.push(buff[0]);

                if counter == length {
                    break;
                }
            }      
        }

        content
    }

    pub fn set_content_by_inode_index(&mut self, inode_index: usize, content: Vec<u8>) {
        let mut inode = self.get_inode(inode_index);
        let length = content.len();
        let num_of_blocks = ceil(inode.content_length as f32 / BLOCK_SIZE as f32);
        
        for i in 0..num_of_blocks {
            self.set_block_state(BlockState::Free, inode.blocks_indexes[i]);
        }

        let blocks_needed = ceil(length as f32 / BLOCK_SIZE as f32);

        for i in 0..blocks_needed {
            let usizeo_write = min(BLOCK_SIZE, length - BLOCK_SIZE * i);
            let block_index = self.get_first_free_block_index();
            let offset = self.get_block_offset_by_index(block_index);
            self.set_block_state(BlockState::Taken, block_index);
            self.reader_writer.seek(SeekFrom::Start(offset)).expect("failed to seek to block");
            self.reader_writer.write(&content[BLOCK_SIZE * i..BLOCK_SIZE * i + usizeo_write]).expect("failed to write to block");
            inode.blocks_indexes[i] = block_index;
        }

        inode.content_length = length;
        self.set_inode(inode, inode_index);
    }

    pub fn get_inode_offset_by_index(&mut self, inode_index: usize) -> usize {
        inode_index * INODE_SIZE
    }

    pub fn get_inode_index_by_offset(&mut self, inode_offset: usize) -> usize {
        inode_offset / INODE_SIZE
    }

    pub fn get_first_free_inode_index(&mut self) -> usize {
        for i in 0..NUMBER_OF_INODES {
            let inode = self.get_inode(i);
            match inode.inode_type {
                InodeType::Free => return i,
                _ => continue,
            };
        }

        panic!("There is no free inode!");
    }

    // bit map of blocks
    pub fn get_block_state(&mut self, block_index: usize) -> BlockState {
        let mut buff = [0u8; 1];

        self.reader_writer.seek(SeekFrom::Start(BYTE_MAP_OFFSET + block_index)).expect("failed to seek to block");

        self.reader_writer.read(&mut buff).expect("failed to read from block");
        
        match buff[0] {
            0 => BlockState::Free,
            1 => BlockState::Taken,
            _ => panic!("Invalid block state!")
        }
    }
    
    pub fn set_block_state(&mut self, block_state: BlockState, block_index: usize) {
        let mut buff = [0u8; 1];
        self.reader_writer.seek(SeekFrom::Start(BYTE_MAP_OFFSET + block_index)).expect("failed to seek to block");

        buff[0] = match block_state {
            BlockState::Free => 0,
            BlockState::Taken => 1
        };

        self.reader_writer.write(&buff).expect("failed to write to block");
    }

    pub fn get_first_free_block_index(&mut self) -> usize {
        for i in 0..NUMBER_OF_BLOCKS {
            match self.get_block_state(i) {
                BlockState::Free => return i,
                _ => continue,
            };
        }
        
        panic!("There is no free block")
    }

    // Blocks 
    pub fn get_block(&mut self, block_index: usize) -> Vec<u8> {
        let mut buff = vec![0u8; BLOCK_SIZE];
        let offset = self.get_block_offset_by_index(block_index);
        self.reader_writer.seek(SeekFrom::Start(offset)).expect("failed to seek to block");

        self.reader_writer.read(&mut buff).expect("failed to read from block");

        buff.to_vec()
    }

    pub fn set_block(&mut self, block_index: usize, content: Vec<u8>) {
        let offset = self.get_block_offset_by_index(block_index);
        self.reader_writer.seek(SeekFrom::Start(offset)).expect("failed to seek to block");

        self.reader_writer.write(&content).expect("the write operation failed");
    }

    pub fn get_block_offset_by_index(&mut self, block_index: usize) -> usize {
        BLOCKS_OFFSET + block_index * BLOCK_SIZE
    }
    
    pub fn get_block_index_by_offset(&mut self, block_offset: usize) -> usize {
        (BLOCKS_OFFSET - block_offset) / BLOCK_SIZE
    }

    pub fn create_dir_entry(&mut self, entry_name: &str, is_dir: bool, index_of_dir_inode: usize) {
        // read the dir that should contain the file and search for one with the same name
        let mut dir_entries = self.get_entries_from_dir(index_of_dir_inode);

        if dir_entries.contains_key(entry_name) {
            println!("{} with this name is already exist.", if is_dir {"Dir"} else {"File"});
            return;
        }

        // update the dir that should contain the file by adding the new file name with his suitable inode
        let new_inode_index = self.get_first_free_inode_index();

        let mut new_inode = self.get_inode(new_inode_index);

        new_inode.inode_type = if is_dir { InodeType::Dir } else { InodeType::File };
        self.set_inode(new_inode, new_inode_index);
        
        let entry_name_owned = entry_name.to_owned();

        dir_entries.insert(entry_name_owned, new_inode_index);

        self.set_files_to_dir(&dir_entries, index_of_dir_inode);
    }

    pub fn set_files_to_dir(&mut self, dir_entries: &HashMap<String, usize>, dir_inode_index: usize) {
        match self.get_inode(dir_inode_index).inode_type {
            InodeType::Dir => {},
            _ => panic!("Index of the the inode isn't a directory inode."),
        };

        let mut dir_raw_content: Vec<u8> = Vec::new();
        for (name, inode_index) in dir_entries {
            dir_raw_content.extend_from_slice(name.as_bytes());
            dir_raw_content.push(b'\0');
            dir_raw_content.extend_from_slice(&(inode_index.to_le_bytes()));
        }

        self.set_content_by_inode_index(dir_inode_index, dir_raw_content);
    }

    pub fn get_entries_from_dir(&mut self, index_of_dir_inode: usize) -> HashMap<String, usize> {
        let inode = self.get_inode(index_of_dir_inode);

        match inode.inode_type {
            InodeType::Dir => {},
            _ => panic!("Index of the the inode isn't a directory inode."),
        };

        let content = self.get_content_by_inode_index(index_of_dir_inode);
        let mut dir_entries = HashMap::new();

        let mut i = 0;
        let mut name = String::new();
        let mut curr_byte;

        while i < content.len() {
            curr_byte = content[i];

            if curr_byte != b'\0' {
                name.push(curr_byte as char);
            } else {
                let mut inode_entry_buf = [0u8; mem::size_of::<usize>()];
                inode_entry_buf.copy_from_slice(&content[i + 1..i + 1 + mem::size_of::<usize>()]);
                let inode_entry = usize::from_le_bytes(inode_entry_buf);
                dir_entries.insert(name.clone(), inode_entry);
                name.clear();
                i += mem::size_of::<usize>();
            }

            i += 1;
        }

        dir_entries
    }
    
    pub fn get_type_by_path(&mut self, path: &str) -> InodeType {
        match self.get_inode_index_from_path(path, 0) {
            Ok(inode_index) => self.get_inode(inode_index).inode_type,
            Err(_) => InodeType::Free,
        }
    }

    pub fn get_inode_index_from_path(&mut self, path: &str, inode_index: usize) -> Result<usize, &'static str> {
        if path.is_empty() {
            return Err("Path can't be empty");
        }
    
        if path == "/" {
            return Ok(0);
        }

        let sep = '/';
        let sep_index = path.find(sep);

        let before_sep: &str;
        let after_sep: &str;

        match sep_index {
            Some(index) => {
                if index == 0 {
                    return self.get_inode_index_from_path(path, inode_index);
                }

                before_sep = &path[..index];
                after_sep = &path[index + 1..];
            },
            None => {
                before_sep = path;
                after_sep = "";
            }
        };
    
        let dir_entries = self.get_entries_from_dir(inode_index);
    
        if !dir_entries.contains_key(before_sep) {
            return Err("Unknown entry");
        }
    
        let entry_inode_index = dir_entries[before_sep];
    
        if after_sep.is_empty() {
            return Ok(entry_inode_index);
        }
    
        self.get_inode_index_from_path(after_sep, entry_inode_index)
    }    

    pub fn path_to_base_dir_and_entry<'a>(&self, path: &'a str) -> (&'a str, &'a str) {
        let last_sep_index = path.rfind('/'); // Find the last occurrence of '/'
        let last_sep_index = match last_sep_index {
            Some(index) => index,
            None => {
                return ("/", path);
            }
        };

        (&path[0..last_sep_index], &path[last_sep_index + 1..])
    }

}

fn ceil(x: f32) -> usize {
    let xi = x as i32;
    if x > 0.0 && x != xi as f32 {
        (xi + 1) as usize
    } else {
        xi as usize
    }
}

