extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::time::Duration;
use std::thread::sleep;
use std::env;

struct Args {
    pin: u64,
    duration_ms: u64,
    period_ms: u64
}

fn blink(pin: u64, duration_ms: u64, period_ms: u64) -> sysfs_gpio::Result<()> {
    let led = Pin::new(pin);

    led.with_exported(|| {
        led.set_direction(Direction::Low)?;

        let iterations = duration_ms / period_ms / 2;

        for _ in 0..iterations {
            led.set_value(0)?;

            sleep(Duration::from_millis(period_ms));

            led.set_value(1)?;

            sleep(Duration::from_millis(period_ms));
        }

        led.set_value(0)?;

        Ok(())
    })
}

fn print_usage() {
    println!("Usage: cargo run <pin> <duration_ms> <period_ms>");
}

fn get_args() -> Option<Args> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return None;
    }

    let pin = if let Ok(pin) = args[1].parse::<u64>() { pin } else { return None; };

    let duration_ms = if let Ok(ms) = args[2].parse::<u64>() { ms } else { return None; };

    let period_ms = if let Ok(ms) = args[3].parse::<u64>() { ms } else { return None; };

    Some(Args {
        pin: pin,
        duration_ms: duration_ms,
        period_ms: period_ms
    })
}

fn main() {
    match get_args() {
        None => print_usage(),
        Some(args) => {
            match blink(args.pin, args.duration_ms, args.period_ms) {
                Ok(()) => println!("Success!"),
                Err(err) => println!("We have a blinking problem: {}", err),
            }
        }
    }
}
