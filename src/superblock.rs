use crate::device::Device;
use crate::inode::{Inode, INODE_SIZE};
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
}

impl Superblock {
    pub fn init(device: &Device, total_blocks: u16) -> std::io::Result<()> {
        Self::init_superblock(device, total_blocks)?;
        Self::init_bitmap(device, total_blocks)?;
        Ok(())
    }

    fn init_superblock(device: &Device, total_blocks: u16) -> std::io::Result<()> {
        let bitmap_size = Self::bitmap_size(total_blocks);
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
        };
        let sb_ptr = &sb as *const Superblock as *const u8;
        let sb_size = mem::size_of::<Superblock>();
        let buf = unsafe { slice::from_raw_parts(sb_ptr, sb_size) };
        device.write_block(0, &buf)?;
        Ok(())
    }

    fn init_bitmap(device: &Device, total_blocks: u16) -> std::io::Result<()> {
        let bitmap_size = Self::bitmap_size(total_blocks);
        let inode_array_blocks = Self::inode_array_blocks(total_blocks);
        let buf = [0u8; Device::BLOCK_SIZE as usize];
        for i in 1..bitmap_size {
            device.write_block(i.into(), &buf)?;
        }
        for i in 0..=(bitmap_size + inode_array_blocks) {
            Self::write_bit(&device, 1, i)?;
        }
        Ok(())
    }

    fn write_bit(device: &Device, value: u8, position: u16) -> std::io::Result<()> {
        let mut buf = [0u8; Device::BLOCK_SIZE as usize];
        let block = (position / (Device::BLOCK_SIZE * 8) as u16) + 1;
        device.read_block(block.into(), &mut buf)?;
        let byte_position = (position % (Device::BLOCK_SIZE * 8)) / 8;
        let bit_offset = (position % (Device::BLOCK_SIZE * 8)) % 8;
        let mut target_byte = buf[byte_position as usize];
        let mask = 128 >> bit_offset; // 10000000 shifted bit_offset positions
        match value {
            1 => target_byte |= mask,
            0 => target_byte &= mask,
            b => panic!("Invalid bit value: {}", b),
        }
        buf[byte_position as usize] = target_byte;
        device.write_block(block.into(), &buf)?;
        Ok(())
    }

    // Returns the number of blocks used by the bitmap
    // for a given total of blocks in the device.
    // 1 bit per block, and every block is BLOCK_SIZE * 8 bits.
    fn bitmap_size(total_blocks: u16) -> u16 {
        let bits_per_block = Device::BLOCK_SIZE * 8;
        if total_blocks % bits_per_block == 0 {
            total_blocks / bits_per_block
        } else {
            (total_blocks / bits_per_block) + 1
        }
    }

    // Returns the number of blocks used by the inode array.
    // The number of inodes will be total_blocks/2.
    fn inode_array_blocks(total_blocks: u16) -> u16 {
        let total_inodes = total_blocks / 2;
        let array_size = total_inodes * INODE_SIZE;
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
    fn bitmap_size() {
        assert_eq!(Superblock::bitmap_size(8192), 1);
        assert_eq!(Superblock::bitmap_size(8500), 2);
    }

    #[test]
    fn inode_array_blocks() {
        assert_eq!(Superblock::inode_array_blocks(2048), 64);
        assert_eq!(Superblock::inode_array_blocks(2050), 65);
    }
}
