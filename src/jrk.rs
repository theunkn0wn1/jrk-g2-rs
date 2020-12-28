use core::fmt;
use ufmt::{uWrite, uwriteln};

use crate::enums::{JrkG2Command, VarOffset};

/// Trait that defines common operations for the Jrk G2, and has to be implemented for any
/// particular communication bus (currently available: Serial / I2C / Blocking I2C).
pub trait JrkG2<ComError> {
    const HEADER: &'static str;

    fn write(&mut self, data: &[u8]) -> Result<(), ComError>;
    fn read(&mut self, cmd: VarOffset) -> Result<u16, ComError>;

    /// Put the motor in motion
    ///
    /// the target should be between 0 and 4095, and its meaning depends on the configurtion on the
    /// controller, usually made by USB
    fn set_target(&mut self, target: u16) -> Result<(), ComError> {
        self.write(&[
            JrkG2Command::SetTarget as u8 + (target & 0x1F) as u8,
            0x7F & (target >> 5) as u8,
        ])
    }
    /// Stop the motor
    fn stop_motor(&mut self) -> Result<(), ComError> {
        self.write(&[JrkG2Command::MotorOff as u8])
    }
    /// Print one of the internal variables of the controller to a fmt::Write implementor
    fn show_var<W: fmt::Write>(&mut self, f: &mut W, var: VarOffset) -> Result<(), ComError> {
        f.write_fmt(format_args!("{:?}: {}\n", var, self.read(var)?))
            .ok();
        Ok(())
    }
    /// Print main internal variables of the controller to a fmt::Write implementor
    fn show_vars<W: fmt::Write>(&mut self, f: &mut W) -> Result<(), ComError> {
        f.write_str(Self::HEADER).ok();
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

    /// Print one of the internal variables of the controller to a uWrite
    fn ushow_var<W: uWrite>(&mut self, f: &mut W, var: VarOffset) -> Result<(), ComError> {
        uwriteln!(f, "{:?}: {}", var, self.read(var).ok().unwrap()).ok();
        Ok(())
    }
    /// Print main internal variables of the controller to a uWrite implementor
    fn ushow_vars<W: uWrite>(&mut self, f: &mut W) -> Result<(), ComError> {
        f.write_str(Self::HEADER).ok();
        self.ushow_var(f, VarOffset::Input)?;
        self.ushow_var(f, VarOffset::Target)?;
        self.ushow_var(f, VarOffset::Feedback)?;
        self.ushow_var(f, VarOffset::ScaledFeedback)?;
        self.ushow_var(f, VarOffset::Integral)?;
        self.ushow_var(f, VarOffset::DutyCycleTarget)?;
        self.ushow_var(f, VarOffset::PIDPeriodCount)?;
        self.ushow_var(f, VarOffset::ErrorFlagsHalting)?;
        self.ushow_var(f, VarOffset::ErrorFlagsOccurred)?;
        self.ushow_var(f, VarOffset::VinVoltage)?;
        self.ushow_var(f, VarOffset::Current)?;
        Ok(())
    }
}
