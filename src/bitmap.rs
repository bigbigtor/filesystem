use crate::device::Device;
use crate::superblock::Superblock;

pub struct Bitmap;

impl Bitmap {
    pub fn init(device: &Device) -> std::io::Result<()> {
        let mut sb = Superblock::read(device)?;
        for i in 0..sb.get_first_data_block() {
            Self::write_bit(&device, 1, i)?;
            sb.decrement_free_blocks();
        }
        Superblock::write(device, &sb)?;
        Ok(())
    }

    fn write_bit(device: &Device, value: u8, position: u16) -> std::io::Result<()> {
        let mut buf = vec![0u8; Device::BLOCK_SIZE as usize];
        Self::read_bitmap_block(device, position, &mut buf)?;
        Self::set_bit(value, position, &mut buf);
        Self::write_bitmap_block(device, position, &buf)?;
        Ok(())
    }

    fn set_bit(value: u8, position: u16, buf: &mut [u8]) {
        let byte_position = (position % (Device::BLOCK_SIZE * 8)) / 8;
        let bit_offset = (position % (Device::BLOCK_SIZE * 8)) % 8;
        let mut target_byte = buf[byte_position as usize];
        let mask = 128 >> bit_offset; // 10000000 shifted bit_offset positions
        match value {
            1 => target_byte |= mask,
            0 => target_byte &= !mask,
            b => panic!("Invalid bit value: {}", b),
        }
        buf[byte_position as usize] = target_byte;
    }

    fn read_bitmap_block(
        device: &Device,
        position: u16,
        mut buf: &mut [u8],
    ) -> std::io::Result<()> {
        let block = (position / (Device::BLOCK_SIZE * 8) as u16) + 1;
        device.read_block(block.into(), &mut buf)?;
        Ok(())
    }

    fn write_bitmap_block(device: &Device, position: u16, buf: &[u8]) -> std::io::Result<()> {
        let block = (position / (Device::BLOCK_SIZE * 8) as u16) + 1;
        device.write_block(block.into(), &buf)?;
        Ok(())
    }

    // Returns the number of blocks used by the bitmap
    // for a given total of blocks in the device.
    // 1 bit per block, and every block is BLOCK_SIZE * 8 bits.
    pub fn size(total_blocks: u16) -> u16 {
        let bits_per_block = Device::BLOCK_SIZE * 8;
        if total_blocks % bits_per_block == 0 {
            total_blocks / bits_per_block
        } else {
            (total_blocks / bits_per_block) + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn size() {
        assert_eq!(Bitmap::size(8192), 1);
        assert_eq!(Bitmap::size(8500), 2);
    }

    #[test]
    fn set_bit() {
        let mut buf = vec![0u8; Device::BLOCK_SIZE as usize];
        Bitmap::set_bit(1, 15, &mut buf);
        assert_eq!(buf[1], 1);
        Bitmap::set_bit(0, 15, &mut buf);
        assert_eq!(buf[1], 0);
    }
}
