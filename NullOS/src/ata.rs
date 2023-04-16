use core::slice;
use core::str;
use x86_64::instructions::port::Port;
use x86_64::instructions::port::ReadWriteAccess;
use x86_64::instructions::port::PortGeneric;

pub const BLOCK_SIZE: usize = 512;

pub struct PortRange {
    start: u16,
    end: u16,
}

impl PortRange {
    pub const fn new(start: u16, end: u16) -> Self {
        PortRange { start: start, end: end }
    }

    pub fn get(&self, index: u16) -> u16 {
        if self.end-self.start < index {
            panic!("Port out of range.");
        }
        self.start + index
    }
}

#[repr(u8)]
#[allow(dead_code)]
pub enum Drive {
    Master = 0xA0,
    Slave = 0xB0,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum DeviceType {
    Ata,
    Atapi,
    Sata,
    Satapi,
}

pub struct Disk {
    control_ports: PortRange,
    status_port: u16,
}

#[allow(dead_code)]
pub enum AtaIdentifyResponse {
    ValidDevice,
    InvalidDevice(DeviceType),
    DoesntExist,
}

#[derive(Copy, Clone)]
#[repr(u16)]
#[allow(dead_code)]
enum RegisterType {
    ErrorInformation = 1,
    SectorCount = 2,
    LbaLow = 3,
    LbaMid = 4,
    LbaHigh = 5,
    Drive = 6,
    Command = 7,
    Status,
}

impl Disk {
    pub const fn new() -> Self {
        Disk {
            control_ports: PortRange::new(0x1F0, 0x1F7),
            status_port: 0x3F6,
        }
    }

    unsafe fn write_register(&self, register: RegisterType, value: u8) {
       let mut register_port: PortGeneric<u8, ReadWriteAccess> = Port::new(self.get_port(register));
       register_port.write(value);
    }

    fn get_port(&self, register: RegisterType) -> u16 {
        match register {
            RegisterType::Status => self.status_port,
            _ => self.control_ports.get(register as u16),
        }
    }

    unsafe fn read_register(&self, register: RegisterType) -> u8 {
        let mut register_port: PortGeneric<u8, ReadWriteAccess> = Port::new(self.get_port(register));
        register_port.read()
    }

    // Reading a single value from the data port
    unsafe fn read_data(&self) -> u16 {
        let mut data_port: PortGeneric<u16, ReadWriteAccess> = Port::new(self.control_ports.get(0));
        data_port.read()
    }

    // Writing a single value from the data port
    unsafe fn write_data(&self, data: u16) {
        let mut data_port: PortGeneric<u16, ReadWriteAccess> = Port::new(self.control_ports.get(0));
        data_port.write(data);
    }

    unsafe fn poll<F>(&self, register: RegisterType, condition: F) -> u8 
        where F: Fn(u8) -> bool {
        
        let mut reg_value: u8;
        loop {
            reg_value = self.read_register(register);
            if condition(reg_value) {
                return reg_value;
            }
        }
    }

    pub unsafe fn read(&self, block: usize, buffer: &mut [u8]) -> Result<u8, &str> {
        if buffer.len() % BLOCK_SIZE != 0 {
            return Err("Size of buffer, isnt a multiplication of sector size.");
        } else if buffer.len() / BLOCK_SIZE > 127 {
            return Err("Can only read 127 sectors at a time in LBA28 mode.");
        } else if buffer.len() == 0 {
            return Err("Size of read buffer can't be 0.");
        }

        let sector_count = (buffer.len()/BLOCK_SIZE) as u8;
        let mut command: u8 = 0xE0;
        command |= ((block >> 24) & 0x0F) as u8;
        command |= 0x40; // bit 6 enabled for 28 bit LBA mode.
        
        self.write_register(RegisterType::Drive, command);
        self.write_register(RegisterType::SectorCount, sector_count);
        self.write_register(RegisterType::LbaLow, block as u8);
        self.write_register(RegisterType::LbaMid, (block >> 8) as u8);
        self.write_register(RegisterType::LbaHigh, (block >> 16) as u8);
        self.write_register(RegisterType::Command, 0x20); // READ SECTORS command

        for sector in 0..sector_count {
            // poll until (!Bussy && DataRequestReady) or Error or DriveFault
            let status = self.poll(RegisterType::Status, |x| (x & 0x80 == 0 && x & 0x8 != 0) || x & 0x1 != 0 || x & 0x20 != 0);

            if status & 1 != 0 {
                if sector == 0 {
                    return Err("No sectors read.");
                }
                // Return amount of read sectors.
                return Ok(sector);
            } else if status & 0x20 != 0 {
                return Err("Drive Fault occured.");
            }

            // Read data to buffer
            let buff = slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u16, buffer.len()/2);
            
            for i in 0..(buff.len() / sector_count as usize) {
                buff[i+(sector as usize*256)] = self.read_data();
            }
            
            // Give the drive a 400ns delay to reset its DRQ bit
            
            for _ in 0..4 {
                self.read_register(RegisterType::Status);
            }

        }

        // Return the amount of sectors read.
        Ok(sector_count)
    }

    pub unsafe fn write(&self, block: usize, buffer: &[u8]) -> Result<u8, &str> {

        if buffer.len() % BLOCK_SIZE != 0 {
            return Err("Size of buffer, isnt a multiplication of sector size.");
        } else if buffer.len() / BLOCK_SIZE > 127 {
            return Err("Can only write 127 sectors at a time in LBA28 mode.");
        } else if buffer.len() == 0 {
            return Err("Size of write buffer can't be 0.");
        }

        let sector_count = (buffer.len() / BLOCK_SIZE) as u8;
        let mut command: u8 = 0xE0;
        command |= ((block >> 24) & 0x0F) as u8;
        command |= 0x40; // bit 6 enabled for 28 bit LBA mode.

        self.write_register(RegisterType::Drive, command);
        self.write_register(RegisterType::SectorCount, sector_count);
        self.write_register(RegisterType::LbaLow, block as u8);
        self.write_register(RegisterType::LbaMid, (block >> 8) as u8);
        self.write_register(RegisterType::LbaHigh, (block >> 16) as u8);
        self.write_register(RegisterType::Command, 0x30); // WRITE SECTORS command

        for sector in 0..sector_count {
            // poll until (!Bussy && DataRequestReady) or Error or DriveFault
            let status = self.poll(RegisterType::Status, |x| (x & 0x80 == 0 && x & 0xc0 == 0x40) || x & 0x1 != 0 || x & 0x20 != 0);

            if status & 1 != 0 {
                if sector == 0 {
                    return Err("No sectors written.");
                }
                // Return amount of written sectors.
                return Ok(sector);
            } else if status & 0x20 != 0 {
                return Err("Drive Fault occured.");
            }

            // Write data from buffer
            let buff = slice::from_raw_parts(buffer.as_ptr() as *const u16, buffer.len() / 2);
            for i in 0..(buff.len()/sector_count as usize) {
                self.write_register(RegisterType::Status, 0x20); // Write to DRQ to initiate data transfer
                self.write_register(RegisterType::Command, 0x34); // Write to DRQ to initiate data transfer
                self.write_data(buff[i + (sector as usize * 256)]);
            }
            // Give the drive a 400ns delay to reset its DRQ bit
            for _ in 0..4 {
                self.read_register(RegisterType::Status);
            }
        }

        // Return the amount of sectors written.
        Ok(sector_count)
    }
    
    pub unsafe fn len(&self) -> usize {
        1024 * 1024
    }
}