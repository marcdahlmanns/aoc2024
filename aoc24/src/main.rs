mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let day = 4;

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

        3 => {
            if let Err(e) = day3::main::main() {
                eprintln!("Error executing Day 3: {}", e);
            }

        }

        4 => {
            if let Err(e) = day4::main::main() {
                eprintln!("Error executing Day 4: {}", e);
            }

        }
        _ => println!("Day {} not yet implemented!", day),
    }
}
