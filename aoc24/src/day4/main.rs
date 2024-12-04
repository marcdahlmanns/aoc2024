use crate::utils::import;

//Day4
pub fn main() -> Result<(), std::io::Error> {
    println!("Day 4, part1");

    let mut grid = Vec::new(); //1D Vector
    let mut xmas_count = 0;
    let dimensionality= 140;
    match import::import_file("day4/4-1.txt") {
        Ok(lines) => {
            for line in lines { // 140 chars per line, 140 lines, 19,600 elements
                for char in line.chars() {
                    grid.push(char);
                }
            }
            assert_eq!(grid.len(), 140 * 140);
            let mut horizontal_count = 0;
            horizontal_count = horizontal_check(&grid, dimensionality);
            xmas_count += horizontal_count;


            let mut vertical_count = 0;
            vertical_count = vertical_check(&grid, dimensionality);
            xmas_count += vertical_count;

            let mut diagonal_count = 0;
            diagonal_count = diagonal_check(&grid, dimensionality);
            xmas_count += diagonal_count;
            println!("The number of XMAS found is: {}", xmas_count); // Solution: 2297 (correct)


            //Part 2, Check for X-MAS
            let mas_count = diagonal_mas_check(&grid, dimensionality);
            println!("The number of MAS x MAS found is: {}", mas_count); // Solution: 1745 (correct)
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Err(e)
        }
    }
}
fn diagonal_mas_check(grid: &Vec<char>, dimensionality: usize) -> i32 {
    let mut diagonal_xmas_sum = 0;

    for y in 1..dimensionality-1 {
        for x in 1..dimensionality-1 {
            if grid[y * dimensionality + x] == 'A' {
                // Top-left == M, Top-right == S, Bottom-left == M, Bottom-right == S
                if grid[(y - 1) * dimensionality + x - 1] == 'M'
                    && grid[(y - 1) * dimensionality + x + 1] == 'S'
                    && grid[(y + 1) * dimensionality + x - 1] == 'M'
                    && grid[(y + 1) * dimensionality + x + 1] == 'S' {
                    diagonal_xmas_sum += 1;
                }

                //Top-left == S, Top-right == M, Bottom-left == S, Bottom-right == M
                if grid[(y - 1) * dimensionality + x - 1] == 'S'
                    && grid[(y - 1) * dimensionality + x + 1] == 'M'
                    && grid[(y + 1) * dimensionality + x - 1] == 'S'
                    && grid[(y + 1) * dimensionality + x + 1] == 'M' {
                    diagonal_xmas_sum += 1;
                }

                // Top-left == M, Top-right == M, Bottom-left == S, Bottom-right == S
                if grid[(y - 1) * dimensionality + x - 1] == 'M'
                    && grid[(y - 1) * dimensionality + x + 1] == 'M'
                    && grid[(y + 1) * dimensionality + x - 1] == 'S'
                    && grid[(y + 1) * dimensionality + x + 1] == 'S' {
                    diagonal_xmas_sum += 1;
                }

                // Top-left == S, Top-right == S, Bottom-left == M, Bottom-right == M
                if grid[(y - 1) * dimensionality + x - 1] == 'S'
                    && grid[(y - 1) * dimensionality + x + 1] == 'S'
                    && grid[(y + 1) * dimensionality + x - 1] == 'M'
                    && grid[(y + 1) * dimensionality + x + 1] == 'M' {
                    diagonal_xmas_sum += 1;
                }
            }
        }
    }

    diagonal_xmas_sum
}
fn horizontal_check(grid: &Vec<char>, dimensionality: usize) -> i32 {
    let mut horizontal_xmas_sum = 0;

    // Single pass check for both "XMAS" (left-to-right) and "SAMX" (right-to-left)
    for y in 0..dimensionality {
        for x in 0..dimensionality {
            // Check for "XMAS" (left-to-right)
            if grid[y * dimensionality + x] == 'X' {
                if x + 3 < dimensionality {
                    if grid[y * dimensionality + x + 1] == 'M' {
                        if grid[y * dimensionality + x + 2] == 'A' {
                            if grid[y * dimensionality + x + 3] == 'S' {
                                horizontal_xmas_sum += 1;
                            }
                        }
                    }
                }
            }

            // Check for "SAMX" (right-to-left)
            if grid[y * dimensionality + x] == 'S' {
                if x + 3 < dimensionality {
                    if grid[y * dimensionality + x + 1] == 'A' {
                        if grid[y * dimensionality + x + 2] == 'M' {
                            if grid[y * dimensionality + x + 3] == 'X' {
                                horizontal_xmas_sum += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    horizontal_xmas_sum
}

fn vertical_check(grid: &Vec<char>, dimensionality: usize) -> i32 {
    let mut vertical_xmas_sum = 0;

    for x in 0..dimensionality {
        for y in 0..dimensionality {
            // Top to Bottom
            if grid[y * dimensionality + x] == 'X' {
                if y + 3 < dimensionality {
                    if grid[(y + 1) * dimensionality + x] == 'M' {
                        if grid[(y + 2) * dimensionality + x] == 'A' {
                            if grid[(y + 3) * dimensionality + x] == 'S' {
                                vertical_xmas_sum += 1;
                            }
                        }
                    }
                }
            }

            //Bottom-to-top
            if grid[y * dimensionality + x] == 'S' {
                if y + 3 < dimensionality {
                    if grid[(y + 1) * dimensionality + x] == 'A' {
                        if grid[(y + 2) * dimensionality + x] == 'M' {
                            if grid[(y + 3) * dimensionality + x] == 'X' {
                                vertical_xmas_sum += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    vertical_xmas_sum
}
fn diagonal_check(grid: &Vec<char>, dimensionality: usize) -> i32 {
    let mut diagonal_xmas_sum = 0;

    for y in 0..dimensionality {
        for x in 0..dimensionality {
            // TL->BR
            if grid[y * dimensionality + x] == 'X' {
                if x + 3 < dimensionality && y + 3 < dimensionality {
                    if grid[(y + 1) * dimensionality + x + 1] == 'M' {
                        if grid[(y + 2) * dimensionality + x + 2] == 'A' {
                            if grid[(y + 3) * dimensionality + x + 3] == 'S' {
                                diagonal_xmas_sum += 1;
                            }
                        }
                    }
                }
            }

            // BR->TL
            if grid[y * dimensionality + x] == 'S' {
                if x + 3 < dimensionality && y + 3 < dimensionality {
                    if grid[(y + 1) * dimensionality + x + 1] == 'A' {
                        if grid[(y + 2) * dimensionality + x + 2] == 'M' {
                            if grid[(y + 3) * dimensionality + x + 3] == 'X' {
                                diagonal_xmas_sum += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    for y in 0..dimensionality {
        for x in (3..dimensionality).rev() {
            // TR->BL
            if grid[y * dimensionality + x] == 'X' {
                if x >= 3 && y + 3 < dimensionality {
                    if grid[(y + 1) * dimensionality + x - 1] == 'M' {
                        if grid[(y + 2) * dimensionality + x - 2] == 'A' {
                            if grid[(y + 3) * dimensionality + x - 3] == 'S' {
                                diagonal_xmas_sum += 1;
                            }
                        }
                    }
                }
            }

            // BL->TR
            if grid[y * dimensionality + x] == 'S' {
                if x >= 3 && y + 3 < dimensionality {
                    if grid[(y + 1) * dimensionality + x - 1] == 'A' {
                        if grid[(y + 2) * dimensionality + x - 2] == 'M' {
                            if grid[(y + 3) * dimensionality + x - 3] == 'X' {
                                diagonal_xmas_sum += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    diagonal_xmas_sum
}
