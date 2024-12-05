// Day5
// The notation X|Y means that if both page number X and page number Y are to be produced as part of an update,
// page number X must be printed at some point before page number Y.

/*
First Section: Page Ordering Rules
47|53 --> if an update includes 47 and 53, 47 must be printed before 53 (pages in between are allowed)
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

Second Section: Page Numbers of each Update

75,47,61,53,29 --> First Update. 75 must be printed before 47, 61, 53, and 29. 47 must be printed before 61, 53, and 29. 61 must be printed before 53 and 29. 53 must be printed before 29.
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
*/

// Strategy:
// Save the rules in a vector data structure called rules: HashMap.
// Using a HashMap, we have the first number as the key, the value is a Vector of i32.
// The vector of i32 contains the second number of each pair with the first number.
// Then, we go through each Update in the Second Section,
// and determine if it is correct. To do so, we first save the update (ie the line
// of the second section) in a updates_vector: Vec<Vec<i32>>. Then, we go through this updates_vector,
// and check if the first number is in the HashMap. Then, we iteratively go through the updates vector
// and check if the next number is in the vector of the first number. If it is, we continue. If it is not,
// we return false. If we reach the end of the updates_vector, we return true.
// Then, we look up the second number of the updates_vector in the HashMap, and check if the first number
// is in the vector of the second number. If it is, we continue. If it is not, we return false.
// If we reach the end of the updates_vector, we return true.
// If we reach the end of the updates_vector, we return true.
// We return false, if we reach the end of the updates_vector, and we have not returned true.
// If a vector is ok, we find its middle number, and add it to a running total.
// At the end, we return the running total.

use crate::utils::import;
use std::collections::HashMap;

pub fn main() -> Result<(), std::io::Error> {
    print!("Day 5, part1");
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    // Step 0: Read in file and save rules in a HashMap, and updates in a vector of vectors.
    match import::import_file("day5/5-1.txt") {
        Ok(lines) => {
            for line in lines {
                if line.contains("|") {
                    // These are the rules

                    let mut split = line.split("|");
                    let first = split.next().unwrap().parse::<i32>().unwrap();
                    let second = split.next().unwrap().parse::<i32>().unwrap();
                    if rules.contains_key(&first) {
                        rules.get_mut(&first).unwrap().push(second);
                    } else {
                        rules.insert(first, vec![second]);
                    }
                } else if line.contains(",") {

                    //These are the updates
                    let mut line_vector: Vec<i32> = Vec::new();
                    for number in line.split(",") {
                        line_vector.push(number.parse::<i32>().unwrap());
                    }
                    updates.push(line_vector);
                }
            }
            println!("All rules: {:?}", rules);
            println!("All Updates: {:?}", updates);

            for update in updates {
                for i in update {
                    println!("Checking if {} is in the rules", i);
                }
            }




            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Err(e)
        }
    }
}
