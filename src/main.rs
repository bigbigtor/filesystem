use filesystem::device::Device;
use std::io::Write;
use std::str;

fn main() -> std::io::Result<()> {
    let device = Device::mount("asdf.disk")?;
    let mut original = [0u8; 1024];
    write!(&mut original[..], "hello world!").expect("Can't write");
    device.write_block(5, &original)?;
    original = [0u8; 1024];
    write!(&mut original[..], "ola k ase").expect("Can't write");
    device.write_block(4, &original)?;
    let mut read = [0u8; 1024];
    device.read_block(5, &mut read)?;
    println!("{}", str::from_utf8(&read).unwrap());
    device.umount()?;
    Ok(())
}
