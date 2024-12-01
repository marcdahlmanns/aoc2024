mod utils;
mod day1;
mod day2;

fn main() {
    let day = 1;

    match day {
        1 => {
            if let Err(e) = day1::main::main() {
                eprintln!("Error executing Day 1: {}", e);
            }
        }

        2 => {
            if let Err(e) = day2::main::main() {
                eprintln!("Error executing Day 2: {}", e);
            }

        }

        _ => println!("Day {} not yet implemented!", day),
    }
}
