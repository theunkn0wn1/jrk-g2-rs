#![no_std]

mod enums;
pub use enums::{JrkG2Command, VarOffset};

mod jrk;
pub use jrk::JrkG2;

mod i2c;
pub use i2c::JrkG2I2c;

mod blocking_i2c;
pub use blocking_i2c::JrkG2BlockingI2c;

mod serial;
#[cfg(feature = "async_trait")]
mod non_blocking;

pub use serial::JrkG2Serial;
