use core::fmt;

use crate::enums::{JrkG2Command, VarOffset};

pub trait JrkG2<ComError> {
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
