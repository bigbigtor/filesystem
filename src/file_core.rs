use crate::bitmap::Bitmap;
use crate::device::Device;
use crate::superblock::Superblock;

pub struct FileCore;

impl FileCore {
    pub fn init(device: &Device, total_blocks: u16) -> std::io::Result<()> {
        Superblock::init(device, total_blocks)?;
        Bitmap::init(device, total_blocks)?;
        Ok(())
    }
}
