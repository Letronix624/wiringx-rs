use wiringx::{
    gpio::{Output, Value},
    platform::Platform,
    WiringX,
};

use std::{thread, time::Duration};

fn main() {
    let wiringx = WiringX::new(Platform::MilkVDuoS).unwrap();

    let pin = wiringx.gpio_pin::<Output>(0).unwrap();

    loop {
        pin.write(Value::Low);
        thread::sleep(Duration::from_secs(1));
        pin.write(Value::High);
        thread::sleep(Duration::from_secs(1));
    }
}
