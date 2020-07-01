use crate::device::Device;

pub struct Superblock {
    first_bitmap_block: u32,
    last_bitmap_block: u32,
    first_inode_block: u32,
    last_inode_block: u32,
    first_data_block: u32,
    last_data_block: u32,
    root_dir_inode: u32,
    first_free_inode: u32,
    free_blocks: u32,
    free_inodes: u32,
    total_blocks: u32,
    total_inodes: u32,
}

impl Superblock {
    pub const INODE_SIZE: u64 = 64; //bytes

    // Returns the number of blocks used by the bitmap
    // for a given total of blocks in the device.
    // 1 bit per block, and every block is BLOCK_SIZE * 8 bits.
    fn bitmap_size(total_blocks: u64) -> u64 {
        let bits_per_block = Device::BLOCK_SIZE * 8;
        if total_blocks % bits_per_block == 0 {
            total_blocks / bits_per_block
        } else {
            (total_blocks / bits_per_block) + 1
        }
    }

    // Returns the number of blocks used by the inode array.
    // The number of inodes will be total_blocks/2.
    fn inode_array_blocks(total_blocks: u64) -> u64 {
        let total_inodes = total_blocks / 2;
        let array_size = total_inodes * Self::INODE_SIZE;
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
