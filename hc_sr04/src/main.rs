extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::env;
use std::time::{Duration, Instant};

struct Args {
    output: u64,
    input: u64
}

fn calc_distance(output: u64, input: u64) -> sysfs_gpio::Result<()> {
    let output = Pin::new(output);
    let input = Pin::new(input);

    input.set_direction(Direction::In)?;

    output.with_exported(|| {
        output.set_direction(Direction::High)?;

        sleep(Duration::new(0, 15000));

        output.set_value(0)?;

        while input.get_value().unwrap() == 0 {
            continue;
        };

        let start = Instant::now();

        while input.get_value().unwrap() != 0 {
            continue;
        }

        let time = start.elapsed().subsec_nanos();

        println!("Distance: {}mm", time as f64 * 0.171500 );

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
