use crate::enums::{JrkG2Command, VarOffset};
use crate::jrk::JrkG2;
use core::fmt;
use embedded_hal::blocking::i2c;
use nb::block;

pub struct JrkG2I2c<I2c> {
    device: u8,
    i2c: I2c,
}

impl<I2c, I2cError> JrkG2I2c<I2c>
where
    I2c: i2c::Write<Error = nb::Error<I2cError>> + i2c::Read<Error = nb::Error<I2cError>>,
{
    pub fn new(i2c: I2c) -> Self {
        JrkG2I2c { device: 0x0B, i2c }
    }
    pub fn set_device(&mut self, device: u8) {
        self.device = device;
    }
}

impl<I2c, I2cError> JrkG2<I2cError> for JrkG2I2c<I2c>
where
    I2c: i2c::Write<Error = nb::Error<I2cError>> + i2c::Read<Error = nb::Error<I2cError>>,
{
    fn write(&mut self, data: &[u8]) -> Result<(), I2cError> {
        block!(self.i2c.write(self.device, &data))
    }
    fn read(&mut self, cmd: VarOffset) -> Result<u16, I2cError> {
        let mut buf: [u8; 2] = [0, 0];
        self.write(&[JrkG2Command::GetVariable16 as u8 | (cmd as u8 + 1)])?;
        block!(self.i2c.read(self.device, &mut buf))?;
        Ok(buf[0] as u16 + buf[1] as u16 * 256)
    }
    fn show_vars_header<W: fmt::Write>(&mut self, f: &mut W) {
        f.write_str("Reading Jrk state from I2C:\n").ok();
    }
}
