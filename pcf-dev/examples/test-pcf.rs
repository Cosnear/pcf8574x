use linux_embedded_hal::I2cdev;
use pcf_dev::{pins::PcfHandler, Pcf, PcfDevice, SlaveAddr};
const LCD: &str = "lcd";
const READ: &str = "read";
fn main() {
    let i2c = I2cdev::new("/dev/i2c-2").unwrap();
    let mut pcf = Pcf::new(
        i2c,
        PcfDevice::Pcf8574,
        SlaveAddr::Extend(true, true, false),
        vec![(LCD, 0xff & !0x03), (READ, 0x08)],
    );

    println!(
        "{:?} /t state: {:x}",
        LCD.write(&mut pcf, &[0xff]),
        LCD.prev(&pcf)
    );
    println!(
        "{:?} /t state: {:x}",
        LCD.write(&mut pcf, &[0xd0]),
        LCD.prev(&pcf)
    );

    let mut buff = [0u8; 2];
    println!(
        "{:?} /t read: {:x?} /t state: {:x}",
        READ.read(&mut pcf, &mut buff),
        buff,
        READ.prev(&pcf)
    );
}
