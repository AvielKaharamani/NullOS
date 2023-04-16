use alloc::vec;
use crate::{ata, interrupts::sleep};
use alloc::fmt::Error;

#[derive(PartialEq)]
pub enum SeekFrom {
    Start(usize),
    Current(usize),
    End(usize)
}

pub struct ReaderWriter {
    disk: ata::Disk,
    offset: usize,
}

impl ReaderWriter {
    pub const fn new(disk: ata::Disk) -> ReaderWriter {
        ReaderWriter { disk: disk, offset: 0 }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        sleep(3);

        let start_block: usize = floor(self.offset as f32 / (ata::BLOCK_SIZE as f32));
        let end_block_excluded: usize = ceil((self.offset + buf.len()) as f32 / (ata::BLOCK_SIZE as f32));
        let new_buff_size: usize = (end_block_excluded - start_block) * ata::BLOCK_SIZE;
        let mut buffer = vec![0u8; new_buff_size];

        
        let slice_start = self.offset as usize - start_block * ata::BLOCK_SIZE;
        let slice_end = slice_start + buf.len();

        unsafe { self.disk.read(start_block, &mut buffer).expect("Failed to read from the disk"); }


        for (i, &byte) in buffer[slice_start..slice_end].iter().enumerate() {
            buf[i] = byte;
        }

        self.offset += buf.len();
        Ok(buf.len())
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        sleep(3);

        let start_block: usize = floor(self.offset as f32 / (ata::BLOCK_SIZE as f32));
        let end_block_excluded: usize = ceil((self.offset + buf.len()) as f32 / (ata::BLOCK_SIZE as f32));
        let new_buff_size: usize = (end_block_excluded - start_block) * ata::BLOCK_SIZE;
        let mut buffer = vec![0u8; new_buff_size];
        
        let slice_start = self.offset as usize - start_block * ata::BLOCK_SIZE;

        unsafe { self.disk.read(start_block, &mut buffer).expect("Failed to read from the disk"); }


        for (i, &byte) in buf.iter().enumerate() {
            buffer[slice_start + i] = byte;
        }

        unsafe { self.disk.write(start_block, &mut buffer).expect("Failed to write to the disk"); }  
        
        self.offset += buf.len();
        Ok(buf.len())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
    
    pub fn seek(&mut self, position: SeekFrom) -> Result<usize, Error> {
        let (base_pos, offset) = match position {
            SeekFrom::Start(n) => {
                self.offset = n;
                return Ok(n);
            }
            SeekFrom::Current(n) => (self.offset, n),
            SeekFrom::End(n) => (unsafe {self.disk.len()}, n),
        };
        let new_pos = base_pos.checked_add(offset);

        if let Some(n) = new_pos {
            self.offset = n;
            Ok(self.offset)
        } else {
            panic!("invalid seek to a negative or overflowing position");
        }
    }
}

fn floor(x: f32) -> usize {
    let xi = x as i32;
    if x < 0.0 && x != xi as f32 {
        (xi - 1) as usize
    } else {
        xi as usize
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
