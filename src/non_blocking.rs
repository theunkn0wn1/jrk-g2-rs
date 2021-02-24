use core::fmt;
use async_trait::async_trait;
use ufmt::uWrite;
use crate::enums::{JrkG2Command, VarOffset};

/// Trait that defines common operations for the Jrk G2, and has to be implemented for any
/// particular communication bus (currently available: Serial / I2C / Blocking I2C).
#[async_trait]
pub trait JrkG2Async<ComError> {
    const HEADER: &'static str;

    async fn write(&mut self, data: &[u8]) -> Result<(), ComError>;
    async fn read(&mut self, cmd: VarOffset) -> Result<u16, ComError>;

    /// Put the motor in motion
    ///
    /// the target should be between 0 and 4095, and its meaning depends on the configurtion on the
    /// controller, usually made by USB
    async fn set_target(&mut self, target: u16) -> Result<(), ComError> {
        self.write(&[
            JrkG2Command::SetTarget as u8 + (target & 0x1F) as u8,
            0x7F & (target >> 5) as u8,
        ]).await
    }
    /// Stop the motor
    async fn stop_motor(&mut self) -> Result<(), ComError> {
        self.write(&[JrkG2Command::MotorOff as u8]).await
    }
    /// Print one of the internal variables of the controller to a fmt::Write implementor
    async fn show_var<W: fmt::Write>(&mut self, f: &mut W, var: VarOffset) -> Result<(), ComError> {
        f.write_fmt(format_args!("{:?}: {}\n", var, self.read(var).await?))
            .ok();
        Ok(())
    }
    /// Print main internal variables of the controller to a fmt::Write implementor
    async fn show_vars<W: fmt::Write>(&mut self, f: &mut W) -> Result<(), ComError> {
        f.write_str(Self::HEADER).ok();
        self.show_var(f, VarOffset::Input).await?;
        self.show_var(f, VarOffset::Target).await?;
        self.show_var(f, VarOffset::Feedback).await?;
        self.show_var(f, VarOffset::ScaledFeedback).await?;
        self.show_var(f, VarOffset::Integral).await?;
        self.show_var(f, VarOffset::DutyCycleTarget).await?;
        self.show_var(f, VarOffset::PIDPeriodCount).await?;
        self.show_var(f, VarOffset::ErrorFlagsHalting).await?;
        self.show_var(f, VarOffset::ErrorFlagsOccurred).await?;
        self.show_var(f, VarOffset::VinVoltage).await?;
        self.show_var(f, VarOffset::Current).await?;
        Ok(())
    }

    /// Print one of the internal variables of the controller to a uWrite
    async fn ushow_var<W: uWrite>(&mut self, f: &mut W, var: VarOffset) -> Result<(), ComError> {
        uwriteln!(f, "{:?}: {}", var, self.read(var).await.ok().unwrap()).ok();
        Ok(())
    }
    /// Print main internal variables of the controller to a uWrite implementor
    async fn ushow_vars<W: uWrite>(&mut self, f: &mut W) -> Result<(), ComError> {
        f.write_str(Self::HEADER).ok();
        self.ushow_var(f, VarOffset::Input).await?;
        self.ushow_var(f, VarOffset::Target).await?;
        self.ushow_var(f, VarOffset::Feedback).await?;
        self.ushow_var(f, VarOffset::ScaledFeedback).await?;
        self.ushow_var(f, VarOffset::Integral).await?;
        self.ushow_var(f, VarOffset::DutyCycleTarget).await?;
        self.ushow_var(f, VarOffset::PIDPeriodCount).await?;
        self.ushow_var(f, VarOffset::ErrorFlagsHalting).await?;
        self.ushow_var(f, VarOffset::ErrorFlagsOccurred).await?;
        self.ushow_var(f, VarOffset::VinVoltage).await?;
        self.ushow_var(f, VarOffset::Current).await?;
        Ok(())
    }
}
