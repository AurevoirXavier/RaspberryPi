extern crate sysfs_gpio;
extern crate led;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

struct Args {
    pin: u64,
    led: Option<u64>
}

fn detect(pin: u64, led: Option<u64>) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);

    input.with_exported(|| {
        input.set_direction(Direction::In)?;

        loop {
            if input.get_value().unwrap() == 1 {
                println!("{}, Detected!", Local::now().format("%m-%d-%Y %H:%M:%S"));

                if let Some(led) = led {
                    println!("Blinking.");

                    if let Ok(_) = blink(led, 1000, 200) { continue; }
                }
            }

            sleep(Duration::from_secs(1));
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

    if let 1 ... 2 = len {
        let pin = if let Ok(pin) = args[1].parse::<u64>() { pin } else { return None; };

        if len == 3 {
            led = if let Ok(led) = args[2].parse::<u64>() {
                Some(led)
            } else {
                return None;
            };
        } else { println!("Led pin not set."); }

        Some(Args { pin, led })
    } else { None }
}

fn main() {
    if let Some(args) = get_args() {
        match detect(args.pin, args.led) {
            Ok(()) => println!("Success!"),
            Err(err) => println!("Something wrong when detect: {}", err),
        }
    } else {
        print_usage();
    }
}
