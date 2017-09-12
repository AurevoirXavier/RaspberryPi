extern crate sysfs_gpio;
extern crate time;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::env;
use std::time::Duration;
use time::PreciseTime;

struct Args {
    output: u64,
    input: u64
}

fn calc_distance(output: u64, input: u64) -> sysfs_gpio::Result<()> {
    let output = Pin::new(output);
    let input = Pin::new(input);

    output.with_exported(|| {
        let start = PreciseTime::now();

        output.set_direction(Direction::High)?;

        sleep(Duration::from_millis(1));

        output.set_value(1)?;

        while input.get_value().unwrap() == 0 {
            break;
        };

        loop {
            if let Some(v) = input.get_value() {
                if v == 0 { break; }
            } else { continue; }
        }

        let time = start.to(PreciseTime::now());

        println!("Distance = {}", time);

        Ok(())
    })
}

fn print_usage() {
    println!("Usage: cargo run <input> <output>");
}

fn get_args() -> Option<Args> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return None;
    }

    let output = if let Ok(output) = args[1].parse::<u64>() { output } else { return None; };

    let input = if let Ok(input) = args[2].parse::<u64>() { input } else { return None; };

    Some(Args {
        output,
        input
    })
}

fn main() {
    match get_args() {
        None => print_usage(),
        Some(args) => {
            match calc_distance(args.output, args.input) {
                Ok(()) => println!("Success!"),
                Err(err) => println!("We have a calculate problem: {}", err),
            }
        }
    }
}
