use crate::utils::import;
use regex::Regex;

pub fn main() -> Result<(), std::io::Error> {
    println!("Day 3, part1");

    let mut input_string = String::new();
    let multiplication_parenthesis_pattern = r"mul\(\d+,\s*\d+\)";
    let numbers_in_parenthesis_pattern = r"mul\((\d+),\s*(\d+)\)".to_owned();
    let regex = Regex::new(&*multiplication_parenthesis_pattern).expect("Invalid regex pattern");
    let numbers_regex =
        Regex::new(&*numbers_in_parenthesis_pattern).expect("Invalid regex pattern");

    match import::import_file("day3/3-1.txt") {
        Ok(lines) => {
            for line in lines {
                input_string += &*line;
            }
            let mut part1_sum = 0;

            for multiplication_instruction in regex.captures_iter(&input_string) {
                if let Some(matched_string) = multiplication_instruction.get(0) {
                    for numbers in numbers_regex.captures_iter(matched_string.as_str()) {
                        if let Some(first_number) = numbers.get(1) {
                            if let Some(second_number) = numbers.get(2) {
                                let first_number = first_number.as_str().parse::<i32>().unwrap();
                                let second_number = second_number.as_str().parse::<i32>().unwrap();
                                part1_sum += first_number * second_number;
                            }
                        }
                    }
                }
            }
            println!(
                "The sum of the multiplication instructions is: {}",
                part1_sum
            ); // Solution: 187194524 (correct)

            // Part 2
            println!("Day 3, part2");
            let mul_pattern = r"mul\(\d+,\s*\d+\)";
            let do_pattern = r"do\(\)";
            let dont_pattern = r"don't\(\)";
            let full_pattern = format!("{}|{}|{}", mul_pattern, do_pattern, dont_pattern);
            let combined_regex = Regex::new(&full_pattern).expect("Invalid combined regex");

            let mut part2_sum = 0;
            let mut mul_enabled = true;

            for cap in combined_regex.captures_iter(&input_string) {
                let matched_string = cap.get(0).unwrap().as_str();
                if matched_string.starts_with("do(") {
                    mul_enabled = true;
                } else if matched_string.starts_with("don't(") {
                    mul_enabled = false;
                } else if matched_string.starts_with("mul(") && mul_enabled {
                    if let Some(numbers) = numbers_regex.captures(matched_string) {
                        let num1: i32 = numbers[1].parse().unwrap();
                        let num2: i32 = numbers[2].parse().unwrap();
                        part2_sum += num1 * num2;
                    }
                }
            }
            println!("The sum of valid mul instructions is: {}", part2_sum); // Solution: 127092535 (correct)
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Err(e)
        }
    }
}
