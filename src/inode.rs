use crate::device::Device;
use crate::superblock::Superblock;
use std::mem;
use std::slice;

pub struct Inode {
    inode_type: u8,
    permissions: u8,
    atime: u16,          //access time
    mtime: u16,          //data modification time
    ctime: u16,          //inode modification time
    link_number: u16,    //number of link entries in directory
    log_byte_size: u16,  //size in logic bytes
    oc_data_blocks: u16, //number of ocupied data blocks
    direct_pointers: [u16; 12],
    indirect_pointers: [u16; 3], //[0] simple indirect, [1] double indirect, [2] triple indirect
    padding: [u8; 20],
}

pub const INODE_SIZE: u16 = 64; //bytes

impl Inode {
    pub fn init_inode_array(device: &Device) -> std::io::Result<()> {
        let sb = Superblock::read(device)?;
        let inode = Inode {
            inode_type: 'f' as u8,
            permissions: 7,
            atime: 0,
            mtime: 0,
            ctime: 0,
            link_number: 0,
            log_byte_size: 0,
            oc_data_blocks: 0,
            direct_pointers: [0u16; 12],
            indirect_pointers: [0u16; 3],
            padding: [0u8; 20],
        };
        for pos in 0..sb.get_total_inodes() {
            Self::write_inode(device, &inode, pos)?;
        }
        Ok(())
    }

    fn write_inode(device: &Device, inode: &Inode, position: u16) -> std::io::Result<()> {
        let sb = Superblock::read(device)?;
        let mut buf = vec![0u8; Device::BLOCK_SIZE as usize];
        let inodes_per_block = Device::BLOCK_SIZE / INODE_SIZE;
        let target_block = sb.get_first_inode_block() + (position / inodes_per_block);
        let position_in_block = position % inodes_per_block;
        device.read_block(target_block, &mut buf)?;
        let inode_ptr = inode as *const Inode as *const u8;
        let inode_buf = unsafe { slice::from_raw_parts(inode_ptr, INODE_SIZE.into()) };
        for (i, b) in inode_buf.iter().enumerate() {
            buf[((position_in_block * INODE_SIZE) + i as u16) as usize] = *b;
        }
        device.write_block(target_block, &buf)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inode_size() {
        assert_eq!(mem::size_of::<Inode>(), INODE_SIZE as usize);
    }
}
