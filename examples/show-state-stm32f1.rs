//! Get state from Jrk on a STM32f1xx
//!
//! In this example, the Jrk is plugged as follow:
//! SCL is on PB8, SDA on PB9, TX on PB10, RX on PB11
//!
//! PA9 & PA10 are used as a serial monitor

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use core::fmt::Write;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{i2c, pac, prelude::*, serial, timer::Timer};

use jrk_g2_rs::{Config, JrkBoard};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Initialize USART1 on PA9 & PA10 for monitoring
    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let pin_rx = gpioa.pa10;

    let serial = serial::Serial::usart1(
        dp.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr,
        serial::Config::default().baudrate(115_200.bps()),
        clocks,
        &mut rcc.apb2,
    );
    let (mut tx, _rx) = serial.split();
    writeln!(tx, "serial monitor initialized").unwrap();

    // Initialize connexion to the Jrk: I2C1 on PB8 & PB9 and USART3 on PB10 & PB11
    let jrk_scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let jrk_sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
    let jrk_tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let jrk_rx = gpiob.pb11;

    let jrk_i2c = i2c::BlockingI2c::i2c1(
        dp.I2C1,
        (jrk_scl, jrk_sda),
        &mut afio.mapr,
        i2c::Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: i2c::DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        10000,
        100,
        10000,
        10000,
    );

    let jrk_ser = serial::Serial::usart3(
        dp.USART3,
        (jrk_tx, jrk_rx),
        &mut afio.mapr,
        serial::Config::default().baudrate(9_600.bps()),
        clocks,
        &mut rcc.apb1,
    );

    let (jrk_tx, jrk_rx) = jrk_ser.split();

    let mut jrk = JrkBoard::new(Config::default(), jrk_i2c, jrk_tx, jrk_rx);
    writeln!(tx, "Jrk initialized").unwrap();

    loop {
        jrk.switch_to_i2c();
        jrk.show_state(&mut tx).ok();
        jrk.switch_to_serial();
        jrk.show_state(&mut tx).ok();
        block!(timer.wait()).unwrap();
    }
}
