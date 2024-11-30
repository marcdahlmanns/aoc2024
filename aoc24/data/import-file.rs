//Import an external File and read it line-by-line

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::Error;

pub fn import_file(filepath: &str) -> Result<Vec<String>, Error> { 
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}
 
fn main() -> Result<(), Error> {
    let filepath = "file.txt";
    match import_file(filepath) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", filepath, e);
        }
    }
    Ok(())
}
