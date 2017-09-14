extern crate sysfs_gpio;
extern crate led;
extern crate chrono;

use sysfs_gpio::{Direction, Pin};
use std::env::args;
use std::thread::sleep;
use std::time::Duration;
use led::blink;
use chrono::Local;

struct Args {
    pin: u64,
    led: Option<u64>,
}

fn detect(pin: u64, led: Option<u64>) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);

    input.with_exported(|| {
        input.set_direction(Direction::In)?;

        if input.get_value().unwrap() == 0 {
            println!("{}, Detected!", Local::now().format("%m-%d-%Y %H:%M:%S"));

            if let Some(led) = led {
                println!("Blinking.");

                match blink(led, 1000, 500) {
                    Ok(()) => return Ok(()),
                    Err(err) => println!("Something wrong when blinking: {}", err),
                }
            }
        }

        Ok(())
    })
}

fn print_usage() {
    println!("Usage: cargo run <output> <(Option) led>");
}

fn get_args() -> Option<Args> {
    let args: Vec<String> = args().collect();

    let len = args.len();

    let mut led = None;

    if let 2 ... 3 = len {
        let pin = if let Ok(pin) = args[1].parse::<u64>() {
            pin
        } else {
            return None;
        };

        if len == 3 {
            led = if let Ok(led) = args[2].parse::<u64>() {
                Some(led)
            } else {
                return None;
            };
        } else {
            println!("Led pin not set.");
        }

        Some(Args { pin, led })
    } else {
        None
    }
}

fn main() {
    if let Some(args) = get_args() {
        let pin = args.pin;
        let led = args.led;

        loop {
            match detect(pin, led) {
                Ok(()) => sleep(Duration::from_secs(1)),
                Err(err) => println!("Something wrong when detect: {}", err),
            }
        }
    } else {
        print_usage();
    }
}