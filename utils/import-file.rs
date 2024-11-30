//Import an external File and read it line-by-line

use std::fs::File;
use std::io::{BufReader};

[derive(Debug)]
pub fn importFile(filepath: &str) -> Result(Error, Vec<String>){ 
    let file = File::open("file.txt")?;
    let mut reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    println!("{:?}", lines);
    
    lines
}

 
