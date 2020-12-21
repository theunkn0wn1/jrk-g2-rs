use crate::enums::{JrkG2Command, VarOffset};
use crate::jrk::JrkG2;
use embedded_hal::serial;
use nb::block;

pub struct JrkG2Serial<Serial> {
    serial: Serial,
}

impl<Serial> JrkG2Serial<Serial>
where
    Serial: serial::Write<u8> + serial::Read<u8>,
{
    pub fn new(serial: Serial) -> Self {
        JrkG2Serial { serial }
    }
}

impl<Serial> JrkG2<<Serial as serial::Read<u8>>::Error> for JrkG2Serial<Serial>
where
    Serial: serial::Read<u8> + serial::Write<u8>,
{
    const HEADER: &'static str = "Reading Jrk state from Serial:\n";

    fn write(&mut self, data: &[u8]) -> Result<(), <Serial as serial::Read<u8>>::Error> {
        for &b in data.iter() {
            block!(self.serial.write(b)).ok(); // Infallible
        }
        Ok(())
    }
    fn read(&mut self, cmd: VarOffset) -> Result<u16, <Serial as serial::Read<u8>>::Error> {
        self.write(&[JrkG2Command::GetVariable16 as u8 | (cmd as u8 + 1)])?;
        let l = block!(self.serial.read())?;
        let h = block!(self.serial.read())?;
        Ok(l as u16 + h as u16 * 256)
    }
}
