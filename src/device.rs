use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

pub struct Device {
    file: Mutex<File>,
}

impl Device {
    pub const BLOCK_SIZE: u16 = 1024; //bytes

    pub fn mount(path: &str) -> std::io::Result<Device> {
        Ok(Device {
            file: Mutex::new(
                OpenOptions::new()
                    .create(true)
                    .read(true)
                    .write(true)
                    .open(path)?,
            ),
        })
    }

    pub fn umount(&self) -> std::io::Result<()> {
        drop(self.file.lock().unwrap());
        Ok(())
    }

    pub fn write_block(&self, block: u16, buf: &[u8]) -> std::io::Result<()> {
        let mut f = self.file.lock().unwrap();
        f.seek(SeekFrom::Start(
            (block as u32 * Device::BLOCK_SIZE as u32) as u64,
        ))?;
        f.write_all(buf)?;
        Ok(())
    }

    pub fn read_block(&self, block: u16, buf: &mut [u8]) -> std::io::Result<()> {
        let mut f = self.file.lock().unwrap();
        f.seek(SeekFrom::Start(
            (block as u32 * Device::BLOCK_SIZE as u32) as u64,
        ))?;
        f.read_exact(buf)?;
        Ok(())
    }
}
