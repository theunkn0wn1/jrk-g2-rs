use crate::enums::{JrkG2Command, VarOffset};
use crate::jrk::JrkG2;
use embedded_hal::blocking::i2c;
use nb::block;

/// Implement the JrkG2 trait for Blocking I2C
pub struct JrkG2BlockingI2c<I2c> {
    device: u8,
    i2c: I2c,
}

impl<I2c, I2cError> JrkG2BlockingI2c<I2c>
where
    I2c: i2c::Write<Error = nb::Error<I2cError>> + i2c::Read<Error = nb::Error<I2cError>>,
{
    pub fn new(i2c: I2c) -> Self {
        JrkG2BlockingI2c { device: 0x0B, i2c }
    }
    /// The controller have a default 0x0B I2C address, but this can be manually changed in the
    /// configuration utility.
    pub fn set_device(&mut self, device: u8) {
        self.device = device;
    }
}

impl<I2c, I2cError> JrkG2<I2cError> for JrkG2BlockingI2c<I2c>
where
    I2c: i2c::Write<Error = nb::Error<I2cError>> + i2c::Read<Error = nb::Error<I2cError>>,
{
    const HEADER: &'static str = "Reading Jrk state from Blocking I2C:\n";

    fn write(&mut self, data: &[u8]) -> Result<(), I2cError> {
        block!(self.i2c.write(self.device, &data))
    }
    fn read(&mut self, cmd: VarOffset) -> Result<u16, I2cError> {
        let mut buf: [u8; 2] = [0, 0];
        self.write(&[JrkG2Command::GetVariable16 as u8 | (cmd as u8 + 1)])?;
        block!(self.i2c.read(self.device, &mut buf))?;
        Ok(buf[0] as u16 + buf[1] as u16 * 256)
    }
}
