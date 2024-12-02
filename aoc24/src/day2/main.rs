// Level     1 2 3 4 5
// ___________________
// Report 1: 7 6 4 2 1 safe
// Report 2: 1 2 7 8 9 unsafe
// Report 3: 9 7 6 2 1 unsafe
// Report 4: 1 3 2 4 5 unsafe
// Report 5: 8 6 4 4 1 unsafe
// Report 6: 1 3 6 7 9 safe

// Task: Count the number of safe reports
// Report is safe when:
//          The levels are either all increasing or all decreasing.
//          Any two adjacent levels differ by at least one and at most three.

use crate::utils::import;

pub fn main() -> Result<(), std::io::Error> {
    let part1 = false;
    let _part2 = true;



    let mut number_safe_vectors = 0;
    let mut number_unsafe_vectors = 0;
    if part1 {
        println!("Day 2, part1");
        match import::import_file("day2/2-1.txt") {
            Ok(lines) => {
                for line in lines {
                    let mut vec = Vec::new();
                    for item in line.split(' ') {
                        vec.push(item.parse::<i32>().unwrap());
                    }
                    if is_safe(vec) {
                        number_safe_vectors += 1;
                    } else {
                        number_unsafe_vectors += 1;
                    }
                }
                print!("The number of safe reports is: {}", number_safe_vectors); // Solution: 502 (correct)
                print!("The number of unsafe reports is: {}", number_unsafe_vectors);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                Err(e)
            }
        }
    } else {
        println!("Day 2, part2");
        let mut number_safe_vectors = 0;
        let mut number_unsafe_vectors = 0;
        match import::import_file("day2/2-1.txt") {
            Ok(lines) => {
                for line in lines {
                    let mut vec = Vec::new();
                    for item in line.split(' ') {
                        vec.push(item.parse::<i32>().unwrap());
                    }
                    if is_safe_part2(vec) {
                        number_safe_vectors += 1;
                    } else {
                        number_unsafe_vectors += 1;
                    }
                }
                print!("The number of safe reports is: {}", number_safe_vectors); // Solution: 544
                print!("The number of unsafe reports is: {}", number_unsafe_vectors);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                Err(e)
            }
        }
    }
}

//Helper functions part1
fn is_safe(vec: Vec<i32>) -> bool {
    if is_gradually_increasing_with_correct_distance(&vec)
        || is_gradually_decreasing_with_correct_distance(&vec)
    {
        return true;
    }
    false
}

fn is_gradually_increasing_with_correct_distance(vec: &Vec<i32>) -> bool {
    for i in 1..vec.len() {
        if vec[i - 1] >= vec[i] {
            return false;
        }

        let diff = vec[i] - vec[i - 1];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_gradually_decreasing_with_correct_distance(vec: &Vec<i32>) -> bool {
    for i in 1..vec.len() {
        if vec[i - 1] <= vec[i] {
            return false;
        }
        let diff = vec[i - 1] - vec[i];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

//Helper functions part2

fn is_safe_part2(vec: Vec<i32>) -> bool {
    is_safe_with_removal(&vec)
}

fn is_safe_with_removal(vec: &Vec<i32>) -> bool {
    if is_gradually_increasing_with_correct_distance(&vec)
        || is_gradually_decreasing_with_correct_distance(&vec) {
        return true;
    }

    for i in 0..vec.len() {
        let mut modified_vec = vec.clone();
        modified_vec.remove(i);
        if is_gradually_increasing_with_correct_distance(&modified_vec)
            || is_gradually_decreasing_with_correct_distance(&modified_vec) {
            return true;
        }
    }

    false
}