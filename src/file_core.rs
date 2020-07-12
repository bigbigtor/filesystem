use crate::bitmap::Bitmap;
use crate::device::Device;
use crate::inode::Inode;
use crate::superblock::Superblock;

pub struct FileCore;

impl FileCore {
    pub fn init(device: &Device, total_blocks: u16) -> std::io::Result<()> {
        let buf = vec![0u8; Device::BLOCK_SIZE as usize];
        for i in 0..total_blocks {
            device.write_block(i, &buf)?;
        }
        Superblock::init(device, total_blocks)?;
        Bitmap::init(device)?;
        Inode::init_inode_array(device)?;
        Ok(())
    }
}
