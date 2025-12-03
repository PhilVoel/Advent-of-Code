use std::{cmp::max, env::args, fs};

fn get_joltage(batteries: Vec<u32>, num_bats: u8) -> u64 {
    let mut digits = Vec::with_capacity(num_bats as usize);
    let mut start = 0;

    for iteration in 0..num_bats {
        let mut largest = 0;
        for num in batteries.iter().rev().skip((num_bats - iteration - 1) as usize).rev().skip(start) {
            largest = max(*num, largest);
        }
        start = start + batteries.iter().skip(start).position(|x| *x == largest).unwrap() + 1;
        digits.push(largest);
    }

    digits.into_iter().rev().enumerate().map(|(idx, digit)| {
        digit as u64 * 10_u64.pow(idx as u32)
    }).sum()
}

fn main() {
    let res: u64 = fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| {
            let nums = line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
            let num_bats = if args().any(|arg| arg == "--part2") { 12 } else { 2 };
            get_joltage(nums, num_bats)
        }).sum();
    println!("{res}");
}
