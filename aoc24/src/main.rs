mod utils;
mod day1;

fn main() {
    let day = 1;

    match day {
        1 => {
            if let Err(e) = day1::main::main() {
                eprintln!("Error executing Day 1: {}", e);
            }
        }
        _ => println!("Day {} not yet implemented!", day),
    }
}
