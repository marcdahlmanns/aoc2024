use crate::utils::import;
use std::io::Error;

pub fn main() -> Result<(), Error> {
    let filepath = "src/utils/file.txt"; // Path to the input file

    match import::import_file(filepath) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line); // Print each line from the file
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", filepath, e);
            Err(e) // Return the error
        }
    }
}
