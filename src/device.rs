use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

pub struct Device {
    file: Mutex<File>,
}

impl Device {
    const BLOCK_SIZE: u64 = 1024;

    pub fn mount(path: &str) -> std::io::Result<Device> {
        Ok(Device {
            file: Mutex::new(OpenOptions::new().read(true).write(true).open(path)?),
        })
    }

    pub fn umount(&self) -> std::io::Result<()> {
        drop(self.file.lock().unwrap());
        Ok(())
    }

    pub fn write_block(&self, block: u64, buf: &[u8]) -> std::io::Result<()> {
        let mut f = self.file.lock().unwrap();
        f.seek(SeekFrom::Start(block * Device::BLOCK_SIZE))?;
        f.write_all(buf)?;
        Ok(())
    }

    pub fn read_block(&self, block: u64, buf: &mut [u8]) -> std::io::Result<()> {
        let mut f = self.file.lock().unwrap();
        f.seek(SeekFrom::Start(block * Device::BLOCK_SIZE))?;
        f.read_exact(buf)?;
        Ok(())
    }
}
