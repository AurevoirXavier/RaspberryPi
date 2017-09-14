extern crate led;

pub fn print_usage() {
    println!("Usage: cargo run <pin> <duration_ms> <period_ms>");
}

fn main() {
    if let Some(args) = led::get_args() {
        match led::blink(args.pin, args.duration_ms, args.period_ms) {
            Ok(()) => println!("Finish!"),
            Err(err) => println!("We have a blinking problem: {}", err),
        }
    } else {
        print_usage();
    }
}
