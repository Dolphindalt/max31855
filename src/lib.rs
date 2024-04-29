#![no_std]

/// Crate that communicates with Max31855 over SPI.
/// 
/// Datasheet:
/// https://www.analog.com/media/en/technical-documentation/data-sheets/MAX31855.pdf

use embedded_hal::spi::SpiDevice;
use bit_field::BitField;

pub struct Max31855Data(u32);

impl Max31855Data {

    /// Raw data.
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// These bits contain the signed 14-bit thermocouple temperature.
    pub fn thermo_data(&self) -> i16 {
        self.0.get_bits(18..31) as i16
    }

    /// Temperature read by the thermocouple in celsius.
    pub fn thermo_temperature(&self) -> f32 {
        self.thermo_data() as f32 * 0.25
    }

    /// This bit reads 1 if SCV, SCG, or OC faults are active.
    /// Default value is 0.
    pub fn fault(&self) -> bool {
        self.0.get_bit(16)
    }

    /// These bits contain the signed 12-bit value of the reference junction temperature.
    pub fn temperature_data(&self) -> i16 {
        self.0.get_bits(4..15) as i16
    }

    /// Temperature of the chip in celsius.
    pub fn ambient_temperature(&self) -> f32 {
        self.temperature_data() as f32 * 0.0625
    }

    /// This bit is 1 when the thermocouple is short circuited to VCC.
    /// Default value is 0.
    pub fn scv_bit(&self) -> bool {
        self.0.get_bit(2)
    }

    /// This bit is 1 when the thermocouple is short circuited to ground.
    /// Default value is 0.
    pub fn scg_bit(&self) -> bool {
        self.0.get_bit(1)
    }

    /// This bit is 1 when the thermocouple is open (no connections).
    /// Default value is 0.
    pub fn oc_bit(&self) -> bool {
        self.0.get_bit(0)
    }
}

pub struct Max31855<SPI> {
    spi: SPI,
}

impl<SPI> Max31855<SPI> 
where
    SPI: SpiDevice,
{
    /// Creates a new Max31855 instance.
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Reads a single data point from the MAX31855 containing temperature and fault information.
    pub fn read(&mut self) -> Result<Max31855Data, Max31855Error<SPI::Error>> {
        let mut buf = [0u8; 4];
        self.spi.read(&mut buf).map_err(Max31855Error::Read)?;
        Ok(Max31855Data(u32::from_be_bytes(buf)))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Max31855Error<SPI> {
    Read(SPI),
}

#[test]
fn temperature_conversion_test() {
    let data = Max31855Data(0b0000_0110_0100_1100_0111_1111_0000_0000);
    assert_eq!(data.ambient_temperature(), 127.0);
    assert_eq!(data.thermo_temperature(), 100.75);
}
