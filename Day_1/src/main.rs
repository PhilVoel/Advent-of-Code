use std::{env::args, fs};

fn main() {
    let rotations: Vec<i16> = fs::read_to_string("input.txt").unwrap().lines().map(|line| {
        let (direction, amount) = line.split_at(1);
        let mut amount = amount.parse::<i16>().unwrap();
        if direction == "L" {
            amount = 0-amount;
        }
        amount
    }).collect();
    let mut current = 50;
    let mut count = 0;
    for amount in rotations {
        if args().any(|a| a == "--part2") {
            let sum = current + amount;
            if sum <= 0 && current != 0 {
                count += 1;
            }
            count += sum.abs() / 100;
            current = sum.rem_euclid(100);
        } else {
            current = (current + amount) % 100;
            if current == 0 {
                count += 1;
            }
        }
    }
    println!("{count}");
}
