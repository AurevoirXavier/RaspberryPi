extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::env::args;
use std::thread::sleep;
use std::time::Duration;

struct Args {
    pin: u64,
    duration_s: u64,
    period_s: u64
}

fn detect(pin: u64, duration_s: u64, period_s: u64) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);

    input.with_exported(|| {
        input.set_direction(Direction::In)?;

        let iterations = duration_s / period_s;

        for _ in 0..iterations {
            if input.get_value().unwrap() == 1 {
                println!("Detected!");
            } else {
                println!("Nobody.");
            }

            sleep(Duration::from_secs(period_s))
        }

        Ok(())
    })
}

fn print_usage() {
    println!("Usage: cargo run <output> <duration_s> <period_s>");
}

fn get_args() -> Option<Args> {
    let args: Vec<String> = args().collect();

    if args.len() != 4 {
        return None;
    }

    let pin = if let Ok(pin) = args[1].parse::<u64>() { pin } else { return None; };

    let duration_ms = if let Ok(ms) = args[2].parse::<u64>() { ms } else { return None; };

    let period_ms = if let Ok(ms) = args[3].parse::<u64>() { ms } else { return None; };

    Some(Args {
        pin,
        duration_s,
        period_s
    })
}

fn main() {
    if let Some(args) = get_args() {
        match detect(args.pin, args.duration_s, args.period_s) {
            Ok(()) => println!("Success!"),
            Err(err) => println!("Something wrong when detect: {}", err)
        }
    } else {
        print_usage();
    }
}
