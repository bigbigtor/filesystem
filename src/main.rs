use filesystem::device::Device;
use filesystem::file_core::FileCore;

fn main() -> std::io::Result<()> {
    let device = Device::mount("asdf.disk")?;
    FileCore::init(&device, 8192)?;
    device.umount()?;
    Ok(())
}
