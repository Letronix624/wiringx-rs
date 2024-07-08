//! Prints a message when a GPIO interrupt occurs.
//!
//! Rust replication of the official C interrupt example.

use wiringx::{
    gpio::{Input, Output, Pin, Value},
    platform::Platform,
    WiringX,
};

use std::{thread, time::Duration};

const USAGE: &str = "
Usage: %s platform GPIO GPIO
   first GPIO to write to = output
   second GPIO reacts on an interrupt = input
Example: %s raspberrypi2 16 20
";

fn main() {
    let mut args = std::env::args();
    args.next();

    let platform = if let Some(platform) = args.next() {
        Platform::from_string(&platform).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(-1);
        })
    } else {
        eprintln!("{USAGE}");
        std::process::exit(-1);
    };

    let wiringx = WiringX::new(platform).unwrap();

    let output_pin_number = if let Some(number) = args.next() {
        number.parse::<i32>().unwrap_or_else(|e| {
            eprintln!("Invalid output pin: {e}");
            std::process::exit(-1);
        })
    } else {
        eprintln!("{USAGE}");
        std::process::exit(-1);
    };

    if !wiringx.valid_gpio(output_pin_number) {
        eprintln!("This platform does not support the pin number {output_pin_number}.");
        std::process::exit(-1);
    }

    let input_pin_number = if let Some(number) = args.next() {
        number.parse::<i32>().unwrap_or_else(|e| {
            eprintln!("Invalid input pin: {e}");
            std::process::exit(-1);
        })
    } else {
        eprintln!("{USAGE}");
        std::process::exit(-1);
    };

    if !wiringx.valid_gpio(input_pin_number) {
        eprintln!("This platform does not support the pin number {input_pin_number}.");
        std::process::exit(-1);
    }

    let input_pin = wiringx.gpio_pin::<Input>(input_pin_number).unwrap();
    let output_pin = wiringx.gpio_pin::<Output>(output_pin_number).unwrap();

    input_pin
        .set_isr_mode(wiringx::gpio::IsrMode::Both)
        .expect("Can not set given input GPIO to interrupt BOTH");

    let interrupt_thread = thread::spawn(|| interrupt(input_pin));

    for _ in 0..5 {
        println!("  Writing to GPIO {}: High", output_pin_number);
        output_pin.write(Value::High);
        thread::sleep(Duration::from_secs(1));
        println!("  Writing to GPIO {}: Low", output_pin_number);
        output_pin.write(Value::Low);
        thread::sleep(Duration::from_secs(2));
    }

    println!("Main finished, waiting for thread ...");

    interrupt_thread.join().unwrap();
}

fn interrupt(pin: Pin<Input>) {
    // Not a typo as in the official example.
    println!("Thread created successfully");

    for _ in 0..20 {
        if pin.wait_for_interrupt(Duration::from_secs(1)).is_ok() {
            println!(">>Interrupt on GPIO {}", pin.number());
        } else {
            println!("  Timeout on GPIO {}", pin.number());
        }
    }
}
