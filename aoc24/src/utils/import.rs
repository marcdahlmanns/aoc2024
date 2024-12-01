use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Reads a file line by line and returns the lines in a `Result<Vec<String>, Error>`.
pub fn import_file(filepath: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}
