use crate::enums::{JrkG2Command, VarOffset};
use crate::jrk::JrkG2;
use core::fmt;
use embedded_hal::serial;
use nb::block;

pub struct JrkG2Serial<Tx, Rx> {
    tx: Tx,
    rx: Rx,
}

impl<Tx, Rx> JrkG2Serial<Tx, Rx>
where
    Tx: serial::Write<u8>,
    Rx: serial::Read<u8>,
{
    pub fn new(tx: Tx, rx: Rx) -> Self {
        JrkG2Serial { tx, rx }
    }
}

impl<Rx, Tx> JrkG2<Rx::Error> for JrkG2Serial<Tx, Rx>
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
