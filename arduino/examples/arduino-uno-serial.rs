#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use panic_halt as _;

use jrk_g2_rs::{JrkG2, JrkG2Serial};

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Initialize connexion to the Jrk: Serial on D0 & D1
    let serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        115_200.into_baudrate(),
    );

    let mut jrk = JrkG2Serial::new(serial);

    loop {
        jrk.stop_motor().ok();
        arduino_uno::delay_ms(1000);

        jrk.set_target(1250).ok();
        arduino_uno::delay_ms(2000);
    }
}
