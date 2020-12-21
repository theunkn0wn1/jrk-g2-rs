use linux_embedded_hal::Serial;
use std::{path, thread, time};

use jrk_g2_rs::{JrkG2, JrkG2Serial};

fn main() {
    let serial = Serial::open(path::Path::new("/dev/ttyS0")).unwrap();
    let mut jrk = JrkG2Serial::new(serial);
    println!("jrk initialized on rpi by serial");
    let mut ret: String = "".to_string();

    loop {
        if let Err(e) = jrk.stop_motor() {
            println!("SerialError: {:?}", e);
        }
        thread::sleep(time::Duration::from_secs(2));
        jrk.show_vars(&mut ret).ok();
        println!("{}", ret);
        ret = "".to_string();

        if let Err(e) = jrk.set_target(1450) {
            println!("SerialError: {:?}", e);
        }
        thread::sleep(time::Duration::from_secs(2));
        jrk.show_vars(&mut ret).ok();
        println!("{}", ret);
        ret = "".to_string();
    }
}
