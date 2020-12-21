use linux_embedded_hal::I2cdev;
use std::{thread, time};

use jrk_g2_rs::{JrkG2, JrkG2I2c};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut jrk = JrkG2I2c::new(i2c);
    println!("jrk initialized on rpi by i2c");
    let mut ret: String = "".to_string();

    loop {
        if let Err(e) = jrk.stop_motor() {
            println!("I2cError: {:?}", e);
        }
        thread::sleep(time::Duration::from_secs(2));
        jrk.show_vars(&mut ret).ok();
        println!("{}", ret);
        ret = "".to_string();

        if let Err(e) = jrk.set_target(1500) {
            println!("I2cError: {:?}", e);
        }
        thread::sleep(time::Duration::from_secs(2));
        jrk.show_vars(&mut ret).ok();
        println!("{}", ret);
        ret = "".to_string();
    }
}
