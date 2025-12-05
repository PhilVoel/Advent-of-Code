use std::{env::args, fs};

fn main() {
    let mut grid: Vec<Vec<char>> = fs::read_to_string("input.txt").unwrap().lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '@' {
                    let mut local_count = 0;
                    let start_i = if i == 0 { 0 } else { i-1 };
                    let start_j = if j == 0 { 0 } else { j-1 };
                    for i2 in start_i..=i+1 {
                        for j2 in start_j..=j+1 {
                            if match grid.get(i2) {
                                Some(line) => line.get(j2).unwrap_or(&'.') == &'@',
                                None => false
                            } {
                                local_count += 1;
                            }
                        }
                    }
                    if local_count <= 4 {
                        count += 1;
                        grid[i][j] = '.';
                        changed = true;
                    }
                }
            }
        }
        if args().any(|arg| arg == "--part2") {
            break;
        }
    }
    println!("{count}");
}
