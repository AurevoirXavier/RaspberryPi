extern crate led;

pub fn print_usage() {
    println!("Usage: cargo run <pin> <duration_ms> <period_ms>");
}

fn main() {
    match led::get_args() {
        None => print_usage(),
        Some(args) => {
            match led::blink(args.pin, args.duration_ms, args.period_ms) {
                Ok(()) => println!("Success!"),
                Err(err) => println!("We have a blinking problem: {}", err),
            }
        }
    }
}
