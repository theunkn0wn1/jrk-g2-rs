#![no_std]
use core::fmt;
use core::marker::PhantomData;
use embedded_hal::{blocking::i2c, serial};
use nb::block;

mod enums;
pub use enums::{JrkG2Command, VarOffset};

#[derive(Debug)]
pub enum Error<I2cError, Serial>
where
    Serial: serial::Read<u8>,
{
    I2c(I2cError),
    Serial(Serial::Error),
    Format(fmt::Error),
}

pub struct JrkBoard<I2c, Tx, Rx, I2cError> {
    config: Config,
    i2c: I2c,
    tx: Tx,
    rx: Rx,
    _i2c_err: PhantomData<I2cError>,
}

pub struct Config {
    /// Prefer I²C over serial
    pub use_i2c: bool,

    /// I²C identifier
    pub device: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            use_i2c: true,
            device: 0x0B,
        }
    }
}

impl<I2c, Tx, Rx, I2cError> JrkBoard<I2c, Tx, Rx, I2cError>
where
    I2c: i2c::Write<Error = nb::Error<I2cError>> + i2c::Read<Error = nb::Error<I2cError>>,
    Tx: serial::Write<u8>,
    Rx: serial::Read<u8>,
{
    pub fn new(config: Config, i2c: I2c, tx: Tx, rx: Rx) -> Self {
        JrkBoard {
            config,
            i2c,
            tx,
            rx,
            _i2c_err: PhantomData,
        }
    }
    pub fn write(&mut self, data: &[u8]) -> Result<(), Error<I2cError, Rx>> {
        if self.config.use_i2c {
            block!(self.i2c.write(self.config.device, &data)).map_err(Error::I2c)?;
        } else {
            for &b in data.iter() {
                block!(self.tx.write(b)).ok(); // infallible
            }
        }
        Ok(())
    }
    pub fn read(&mut self, cmd: VarOffset) -> Result<u16, Error<I2cError, Rx>> {
        self.write(&[JrkG2Command::GetVariable16 as u8 | (cmd as u8 + 1)])?;
        if self.config.use_i2c {
            let mut buf: [u8; 2] = [0, 0];
            block!(self.i2c.read(self.config.device, &mut buf)).map_err(Error::I2c)?;
            Ok(buf[0] as u16 + buf[1] as u16 * 256)
        } else {
            let l = block!(self.rx.read()).map_err(Error::Serial)?;
            let h = block!(self.rx.read()).map_err(Error::Serial)?;
            Ok(l as u16 + h as u16 * 256)
        }
    }
    pub fn set_target(&mut self, target: u16) -> Result<(), Error<I2cError, Rx>> {
        self.write(&[
            JrkG2Command::SetTarget as u8 + (target & 0x1F) as u8,
            0x7F & (target >> 5) as u8,
        ])
    }
    pub fn stop_motor(&mut self) -> Result<(), Error<I2cError, Rx>> {
        self.write(&[JrkG2Command::MotorOff as u8])
    }
    pub fn switch_to_i2c(&mut self) {
        self.config.use_i2c = true;
    }
    pub fn switch_to_serial(&mut self) {
        self.config.use_i2c = false;
    }
    pub fn show_state<W: fmt::Write>(&mut self, f: &mut W) -> Result<(), Error<I2cError, Rx>> {
        if self.config.use_i2c {
            f.write_fmt(format_args!("Reading Jrk state from I2C:"))
                .map_err(Error::Format)?;
        } else {
            f.write_fmt(format_args!("Reading Jrk state from serial:"))
                .map_err(Error::Format)?;
        }
        f.write_fmt(format_args!("Input: {}", self.read(VarOffset::Input)?))
            .map_err(Error::Format)?;
        f.write_fmt(format_args!("Target: {}", self.read(VarOffset::Target)?))
            .map_err(Error::Format)?;
        f.write_fmt(format_args!(
            "Feedback: {}",
            self.read(VarOffset::Feedback)?
        ))
        .map_err(Error::Format)?;
        f.write_fmt(format_args!(
            "ScaledFeedback: {}",
            self.read(VarOffset::ScaledFeedback)?
        ))
        .map_err(Error::Format)?;
        f.write_fmt(format_args!(
            "Integral: {}",
            self.read(VarOffset::Integral)?
        ))
        .map_err(Error::Format)?;
        f.write_fmt(format_args!(
            "DutyCycleTarget: {}",
            self.read(VarOffset::DutyCycleTarget)?
        ))
        .map_err(Error::Format)?;
        f.write_fmt(format_args!(
            "PIDPeriodCount: {}",
            self.read(VarOffset::PIDPeriodCount)?
        ))
        .map_err(Error::Format)?;
        f.write_fmt(format_args!(
            "ErrorFlagsHalting: {}",
            self.read(VarOffset::ErrorFlagsHalting)?
        ))
        .map_err(Error::Format)?;
        f.write_fmt(format_args!(
            "ErrorFlagsOccurred: {}",
            self.read(VarOffset::ErrorFlagsOccurred)?
        ))
        .map_err(Error::Format)?;
        f.write_fmt(format_args!(
            "VinVoltage: {}",
            self.read(VarOffset::VinVoltage)?
        ))
        .map_err(Error::Format)?;
        f.write_fmt(format_args!("Current: {}", self.read(VarOffset::Current)?))
            .map_err(Error::Format)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
