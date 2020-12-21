#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use panic_halt as _;

use jrk_g2_rs::{JrkG2, JrkG2I2c};

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Initialize USART0 on D0 & D1 for monitoring
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        115_200.into_baudrate(),
    );

    // Initialize connexion to the Jrk: I2C on A4 & A5
    let i2c = arduino_uno::I2cMaster::new(
        dp.TWI,
        pins.a4.into_pull_up_input(&mut pins.ddr),
        pins.a5.into_pull_up_input(&mut pins.ddr),
        50000,
    );

    let mut jrk = JrkG2I2c::new(i2c);
    ufmt::uwriteln!(&mut serial, "jrk initialized on arduino by i2c").void_unwrap();

    loop {
        if let Err(e) = jrk.stop_motor() {
            ufmt::uwriteln!(&mut serial, "I2cError: {:?}", e).void_unwrap();
        }
        arduino_uno::delay_ms(1000);
        jrk.ushow_vars(&mut serial).ok();

        if let Err(e) = jrk.set_target(1300) {
            ufmt::uwriteln!(serial, "I2cError: {:?}", e).ok();
        }
        arduino_uno::delay_ms(1000);
        jrk.ushow_vars(&mut serial).ok();
    }
}
