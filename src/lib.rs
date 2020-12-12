#![no_std]
use core::fmt;
use embedded_hal::{blocking::i2c, serial};
use nb::block;

mod enums;
use enums::{JrkG2Command, VarOffset};

pub struct JrkError;

impl fmt::Debug for JrkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "jrk error")
    }
}

pub struct JrkBoard<I, S, E>
where
    I: i2c::Write<Error = nb::Error<E>> + i2c::Read<Error = nb::Error<E>>,
    S: serial::Write<u8> + serial::Read<u8>,
{
    i2c: I,
    ser: S,
    device: u8,
    use_i2c: bool,
}

impl<I, S, E> JrkBoard<I, S, E>
where
    I: i2c::Write<Error = nb::Error<E>> + i2c::Read<Error = nb::Error<E>>,
    S: serial::Write<u8> + serial::Read<u8>,
{
    pub fn take(i2c: I, ser: S, device: u8, use_i2c: bool) -> JrkBoard<I, S, E> {
        JrkBoard {
            i2c,
            ser,
            device,
            use_i2c,
        }
    }
    pub fn write(&mut self, data: &[u8]) -> Result<(), JrkError> {
        if self.use_i2c {
            block!(self.i2c.write(self.device, &data)).map_err(|_e| JrkError {});
        } else {
            for &b in data.iter() {
                block!(self.ser.write(b)).map_err(|_e| JrkError {});
            }
        }
        Ok(())
    }
    pub fn read(&mut self, cmd: VarOffset) -> Result<u16, JrkError> {
        self.write(&[JrkG2Command::GetVariable16 as u8 | (cmd as u8 + 1)])
            .map_err(|_e| JrkError {});
        if self.use_i2c {
            let mut buf: [u8; 2] = [0, 0];
            block!(self.i2c.read(self.device, &mut buf)).map_err(|_e| JrkError {})?;
            Ok(buf[0] as u16 + buf[1] as u16 * 256)
        } else {
            let l = block!(self.ser.read()).map_err(|_e| JrkError {})?;
            let h = block!(self.ser.read()).map_err(|_e| JrkError {})?;
            Ok(l as u16 + h as u16 * 256)
        }
    }
    pub fn set_target(&mut self, target: u16) -> Result<(), JrkError> {
        self.write(&[
            JrkG2Command::SetTarget as u8 + (target & 0x1F) as u8,
            0x7F & (target >> 5) as u8,
        ])
    }
    pub fn stop_motor(&mut self) -> Result<(), JrkError> {
        self.write(&[JrkG2Command::MotorOff as u8])
    }

    pub fn switch(&mut self) -> bool {
        self.use_i2c = !self.use_i2c;
        self.use_i2c
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
