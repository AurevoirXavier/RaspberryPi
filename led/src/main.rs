extern crate led;

fn main() {
    match led::get_args() {
        None => led::print_usage(),
        Some(args) => {
            match led::blink(args.pin, args.duration_ms, args.period_ms) {
                Ok(()) => println!("Success!"),
                Err(err) => println!("We have a blinking problem: {}", err),
            }
        }
    }
}
