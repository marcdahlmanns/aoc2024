// Day1 AoC24
// Part 1
// Step 0) Import the file
// Step 1) Pair up: smallest number in the left list with smallest number in right list, and them
//         Second smallest in the left list with second smallest in the right list, and so on.

// Step 2) For each pair, calculate the distance, and continuously add them up.

// Part 2
// Figure out exactly how often each number from the left list appears in the right list
// Calculate a total similarity score by adding up each number in the left list
// after multiplying it by the number of times that number appears in the right list.



use crate::utils::import;
use std::io::Error;


#[derive(Debug)]
struct Pair {
    left: Vec<i32>,
    right: Vec<i32>,
}

    impl Pair {
        fn new() -> Pair {
            Pair {
                left: Vec::new(),
                right: Vec::new(),
            }
        }
    }
pub fn main() -> Result<(), Error> {
    let mut unsorted_pairs = Pair::new();

    match import::import_file("day1/1-1.txt") {
        Ok(lines) => {
            for line in lines {
                //Part1
                let left_side_to_number = line.split(' ').next().unwrap().parse::<i32>().unwrap();
                let right_side_to_number = line.split(' ').last().unwrap().parse::<i32>().unwrap();

                unsorted_pairs.right.push(right_side_to_number);
                unsorted_pairs.left.push(left_side_to_number);

                unsorted_pairs.left.sort(); //implemented as min to max
                unsorted_pairs.right.sort();

                let mut sum = 0;
                for i in 0..unsorted_pairs.left.len() {
                    sum += (unsorted_pairs.left[i] - unsorted_pairs.right[i]).abs();
                }

                //Part2
                let mut similarity_score = 0;
                for i in 0..unsorted_pairs.left.len() {
                    let mut count = 0;
                    for j in 0..unsorted_pairs.right.len() {
                        if unsorted_pairs.left[i] == unsorted_pairs.right[j] {
                            count += 1;
                        }
                    }
                    similarity_score += unsorted_pairs.left[i] * count; }
                println!("similarity score: {}", similarity_score); // Solution: 19678534

            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Err(e)
        }
    }
}
