extern crate sysfs_gpio;
extern crate led;

use sysfs_gpio::{Direction, Pin};
use std::env::args;
use std::thread::sleep;
use std::time::Duration;
use led::blink;

struct Args {
    pin: u64,
    duration_s: u64,
    period_ms: u64,
    led: Option<u64>,
}

fn detect(pin: u64, duration_s: u64, period_ms: u64, led: Option<u64>) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);

    input.with_exported(|| {
        input.set_direction(Direction::In)?;

        let iterations = duration_s / (period_ms / 1000);

        for _ in 0..iterations {
            if input.get_value().unwrap() == 1 {
                println!("Detected!");

                if let Some(led) = led {
                    println!("Blinking.");

                    if let Ok(_) = blink(led, period_ms, 200) {
                        continue;
                    }
                }

            } else {
                println!("Nobody.");
            }

            sleep(Duration::from_millis(period_ms));
        }

        Ok(())
    })
}

fn print_usage() {
    println!(
        "Usage: cargo run <output> <duration_s> <(Recommend 4000)period_ms> <(Option) led>"
    );
}

fn get_args() -> Option<Args> {
    let args: Vec<String> = args().collect();

    let len = args.len();

    let mut led = None;

    if let 4...5 = len {
        let pin = if let Ok(pin) = args[1].parse::<u64>() {
            pin
        } else {
            return None;
        };
        let duration_s = if let Ok(s) = args[2].parse::<u64>() {
            s
        } else {
            return None;
        };
        let period_ms = if let Ok(s) = args[3].parse::<u64>() {
            s
        } else {
            return None;
        };

        if len == 5 {
            led = if let Ok(led) = args[4].parse::<u64>() {
                Some(led)
            } else {
                return None;
            };
        } else {
            println!("Led pin not set.");
        }

        Some(Args {
            pin,
            duration_s,
            period_ms,
            led,
        })
    } else {
        None
    }
}

fn main() {
    if let Some(args) = get_args() {
        match detect(args.pin, args.duration_s, args.period_ms, args.led) {
            Ok(()) => println!("Success!"),
            Err(err) => println!("Something wrong when detect: {}", err),
        }
    } else {
        print_usage();
    }
}
