use max31855_rs::Max31855;
use linux_embedded_hal::SpidevDevice;

fn main() {
    let dev = SpidevDevice::open("/dev/spi-0").unwrap();
    let mut max = Max31855::new(dev);
    
    println!("Getting the temperature from the MAX31855 thermocouple...");

    let max_data = max.read().unwrap();
    let ambient_c = max_data.ambient_temperature();
    let thermo_c = max_data.thermo_temperature();

    println!("Got an ambient temperature of {} C and thermocouple temperature of {} C", ambient_c, thermo_c);

    std::thread::sleep(std::time::Duration::from_nanos(250));
}
