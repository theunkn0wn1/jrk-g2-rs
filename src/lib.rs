#![no_std]
use core::fmt;
use embedded_hal::{blocking::i2c, serial};
use nb::block;

mod enums;
pub use enums::{JrkG2Command, VarOffset};

pub trait JrkBoard<ComError> {
    fn write(&mut self, data: &[u8]) -> Result<(), ComError>;
    fn read(&mut self, cmd: VarOffset) -> Result<u16, ComError>;
    fn show_vars_header<W: fmt::Write>(&mut self, f: &mut W);

    fn set_target(&mut self, target: u16) -> Result<(), ComError> {
        self.write(&[
            JrkG2Command::SetTarget as u8 + (target & 0x1F) as u8,
            0x7F & (target >> 5) as u8,
        ])
    }
    fn stop_motor(&mut self) -> Result<(), ComError> {
        self.write(&[JrkG2Command::MotorOff as u8])
    }
    fn show_var<W: fmt::Write>(&mut self, f: &mut W, var: VarOffset) -> Result<(), ComError> {
        f.write_fmt(format_args!("{:?}: {}\n", var, self.read(var)?))
            .ok();
        Ok(())
    }
    fn show_vars<W: fmt::Write>(&mut self, f: &mut W) -> Result<(), ComError> {
        self.show_vars_header(f);
        self.show_var(f, VarOffset::Input)?;
        self.show_var(f, VarOffset::Target)?;
        self.show_var(f, VarOffset::Feedback)?;
        self.show_var(f, VarOffset::ScaledFeedback)?;
        self.show_var(f, VarOffset::Integral)?;
        self.show_var(f, VarOffset::DutyCycleTarget)?;
        self.show_var(f, VarOffset::PIDPeriodCount)?;
        self.show_var(f, VarOffset::ErrorFlagsHalting)?;
        self.show_var(f, VarOffset::ErrorFlagsOccurred)?;
        self.show_var(f, VarOffset::VinVoltage)?;
        self.show_var(f, VarOffset::Current)?;
        Ok(())
    }
}

/******************************************** I2C ************************************************/

pub struct JrkBoardI2c<I2c> {
    device: u8,
    i2c: I2c,
}

impl<I2c, I2cError> JrkBoardI2c<I2c>
where
    I2c: i2c::Write<Error = nb::Error<I2cError>> + i2c::Read<Error = nb::Error<I2cError>>,
{
    pub fn new(i2c: I2c) -> Self {
        JrkBoardI2c { device: 0x0B, i2c }
    }
    pub fn set_device(&mut self, device: u8) {
        self.device = device;
    }
}

impl<I2c, I2cError> JrkBoard<I2cError> for JrkBoardI2c<I2c>
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

/******************************************* serial **********************************************/

pub struct JrkBoardSerial<Tx, Rx> {
    tx: Tx,
    rx: Rx,
}

impl<Tx, Rx> JrkBoardSerial<Tx, Rx>
where
    Tx: serial::Write<u8>,
    Rx: serial::Read<u8>,
{
    pub fn new(tx: Tx, rx: Rx) -> Self {
        JrkBoardSerial { tx, rx }
    }
}

impl<Rx, Tx> JrkBoard<Rx::Error> for JrkBoardSerial<Tx, Rx>
where
    Tx: serial::Write<u8>,
    Rx: serial::Read<u8>,
{
    fn write(&mut self, data: &[u8]) -> Result<(), Rx::Error> {
        for &b in data.iter() {
            block!(self.tx.write(b)).ok(); // infaillible
        }
        Ok(())
    }
    fn read(&mut self, cmd: VarOffset) -> Result<u16, Rx::Error> {
        self.write(&[JrkG2Command::GetVariable16 as u8 | (cmd as u8 + 1)])?;
        let l = block!(self.rx.read())?;
        let h = block!(self.rx.read())?;
        Ok(l as u16 + h as u16 * 256)
    }
    fn show_vars_header<W: fmt::Write>(&mut self, f: &mut W) {
        f.write_str("Reading Jrk state from Serial:\n").ok();
    }
}
