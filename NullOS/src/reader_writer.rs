use core2;
use alloc::vec;
use crate::{ata, keyboard};

/// A readable and writable "stateful" I/O stream that keeps track 
/// of its current offset within its internal stateless I/O stream.
///
/// ## Trait implementations
/// * This implements the [`core2::io::Read`] and [`core2::io::Write`] traits for read and write access.
/// * This implements the [`core2::io::Seek`] trait if the underlying I/O stream implements [`KnownLength`].
/// * This also forwards all other I/O-related traits implemented by the underlying I/O stream.
/// * This derefs into the inner `IO` type, via both [`Deref`] and [`DerefMut`].
pub struct ReaderWriter<'a> {
    disk: &'a dyn ata::Disk,
    offset: usize,
}

impl<'a> ReaderWriter<'a> {
    /// Creates a new `ReaderWriter<'a>` with an initial offset of 0.
    pub fn new(disk: &'a dyn ata::Disk) -> ReaderWriter<'a> {
        ReaderWriter { disk: disk, offset: 0 }
    }
}

// impl<'a> Deref for ReaderWriter<'a> {
//     fn deref(&self) -> &'a dyn ata::Disk {
//         &self.disk
//     }
// }

// impl<'a> DerefMut for ReaderWriter<'a> {
//     fn deref_mut(&mut self) -> &'a mut dyn ata::Disk {
//         &mut self.disk
//     }
// }

impl<'a> core2::io::Read for ReaderWriter<'a> {
    fn read(&mut self, buf: &mut [u8]) -> core2::io::Result<usize> {
        println!("Read: ");
        let start_block: usize = floor(self.offset as f32 / (ata::BLOCK_SIZE as f32));
        let end_block_excluded: usize = ceil((self.offset + buf.len()) as f32 / (ata::BLOCK_SIZE as f32));
        let new_buff_size: usize = (end_block_excluded - start_block) * ata::BLOCK_SIZE;
        let mut buffer = vec![0u8; new_buff_size];

        
        let slice_start = self.offset as usize - start_block * ata::BLOCK_SIZE;
        let slice_end = slice_start + buf.len();

        println!("self.offset: {}", self.offset);
        println!("buf.len(): {}", buf.len());
        println!("start_block: {}", start_block);
        println!("end_block_excluded: {}", end_block_excluded);
        println!("new_buff_size: {}", new_buff_size);
        println!("slice_start: {}", slice_start);
        println!("slice_end: {}", slice_end);

        unsafe { self.disk.read(start_block, &mut buffer).expect("Failed to read from the disk"); }


        for (i, &byte) in buffer[slice_start..slice_end].iter().enumerate() {
            print!("{:#02x} ", byte);
            buf[i] = byte;
        }

        println!("\n");
        // keyboard::get_string();

        self.offset += buf.len();
        Ok(buf.len())
    }
}

impl<'a> core2::io::Write for ReaderWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> core2::io::Result<usize> {
        println!("Write");

        let start_block: usize = floor(self.offset as f32 / (ata::BLOCK_SIZE as f32));
        let end_block_excluded: usize = ceil((self.offset + buf.len()) as f32 / (ata::BLOCK_SIZE as f32));
        let new_buff_size: usize = (end_block_excluded - start_block) * ata::BLOCK_SIZE;
        let mut buffer = vec![0u8; new_buff_size];
        
        unsafe { self.disk.read(start_block, &mut buffer).expect("Failed to read from the disk"); }

        let slice_start = self.offset as usize - start_block * ata::BLOCK_SIZE;

        for (i, &byte) in buf.iter().enumerate() {
            buffer[slice_start + i] = byte;
        }

        unsafe { self.disk.write(start_block, &mut buffer).expect("Failed to write to the disk"); }
        // keyboard::get_string();    
        
        self.offset += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> core2::io::Result<()> {
        Ok(())
    }    
}

impl<'a> core2::io::Seek for ReaderWriter<'a> {
    fn seek(&mut self, position: core2::io::SeekFrom) -> core2::io::Result<u64> {
        let (base_pos, offset) = match position {
            core2::io::SeekFrom::Start(n) => {
                self.offset = n as usize;
                return Ok(n);
            }
            core2::io::SeekFrom::Current(n) => (self.offset, n),
            core2::io::SeekFrom::End(n) => (unsafe {self.disk.len()}, n),
        };
        let new_pos = if offset >= 0 {
            base_pos.checked_add(offset as usize)
        } else {
            base_pos.checked_sub(offset.wrapping_neg() as usize)
        };
        if let Some(n) = new_pos {
            self.offset = n;
            Ok(self.offset as u64)
        } else {
            Err(core2::io::Error::new(
                core2::io::ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            ))
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