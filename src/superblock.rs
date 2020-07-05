use crate::bitmap::Bitmap;
use crate::device::Device;
use crate::inode::INODE_SIZE;
use std::mem;
use std::slice;

pub struct Superblock {
    first_bitmap_block: u16,
    last_bitmap_block: u16,
    first_inode_block: u16,
    last_inode_block: u16,
    first_data_block: u16,
    last_data_block: u16,
    root_dir_inode: u16,
    first_free_inode: u16,
    free_blocks: u16,
    free_inodes: u16,
    total_blocks: u16,
    total_inodes: u16,
    padding: [u8; 1000],
}

impl Superblock {
    pub fn init(device: &Device, total_blocks: u16) -> std::io::Result<()> {
        let bitmap_size = Bitmap::size(total_blocks);
        let inode_array_blocks = Self::inode_array_blocks(total_blocks);
        let sb = Superblock {
            first_bitmap_block: 1,
            last_bitmap_block: bitmap_size,
            first_inode_block: bitmap_size + 1,
            last_inode_block: bitmap_size + inode_array_blocks,
            first_data_block: bitmap_size + inode_array_blocks + 1,
            last_data_block: total_blocks - 1,
            root_dir_inode: 0,
            first_free_inode: 1,
            free_blocks: total_blocks - 1 - bitmap_size - inode_array_blocks,
            free_inodes: total_blocks / 2,
            total_blocks,
            total_inodes: total_blocks / 2,
            padding: [0u8; 1000],
        };
        Self::write(&device, &sb)?;
        Ok(())
    }

    pub fn read(device: &Device) -> std::io::Result<Superblock> {
        let mut buf = [0u8; Device::BLOCK_SIZE as usize];
        device.read_block(0, &mut buf)?;
        let sb: Superblock = unsafe { mem::transmute(buf) };
        Ok(sb)
    }

    pub fn write(device: &Device, sb: &Superblock) -> std::io::Result<()> {
        let sb_ptr = sb as *const Superblock as *const u8;
        let sb_size = mem::size_of::<Superblock>();
        let buf = unsafe { slice::from_raw_parts(sb_ptr, sb_size) };
        device.write_block(0, &buf)?;
        Ok(())
    }

    pub fn get_first_data_block(&self) -> u16 {
        self.first_data_block
    }

    pub fn decrement_free_blocks(&mut self) {
        self.free_blocks -= 1;
    }

    // Returns the number of blocks used by the inode array.
    // The number of inodes will be total_blocks/2.
    fn inode_array_blocks(total_blocks: u16) -> u16 {
        let total_inodes = total_blocks / 2;
        let array_size = total_inodes.wrapping_mul(INODE_SIZE);
        if array_size % Device::BLOCK_SIZE == 0 {
            array_size / Device::BLOCK_SIZE
        } else {
            (array_size / Device::BLOCK_SIZE) + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inode_array_blocks() {
        assert_eq!(Superblock::inode_array_blocks(2048), 64);
        assert_eq!(Superblock::inode_array_blocks(2050), 65);
    }
}
