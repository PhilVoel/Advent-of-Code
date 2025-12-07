use std::{env::args, fs, iter::zip};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.lines();
    if args().any(|arg| arg == "--part2") {
        let mut input: Vec<Vec<char>> = input.map(|line| line.chars().collect()).collect();
        let num_cols = input[0].len();
        let last = input.pop().unwrap();
        input.push(vec![' '; num_cols]);
        input.push(last);
        let mut transposed: Vec<String> = (0..num_cols).map(|i| input.iter().map(|row| row[i]).collect()).collect();
        transposed.push("".to_string());
        let mut operation = '\0';
        let mut numbers: Vec<u64> = Vec::new();
        let mut sum = 0;
        for line in transposed {
            let mut split = line.split_whitespace();
            if let Some(num) = split.next() {
                numbers.push(num.parse().unwrap());
                if let Some(op) = split.next() {
                    operation = op.chars().collect::<Vec<char>>()[0];
                }
            } else {
                sum += numbers.into_iter().reduce(|acc, num| if operation == '+' {
                    acc + num
                } else {
                    acc * num
                }).unwrap();
                numbers = Vec::new();
            }
        }
        println!("{sum}");
    } else {
        let mut input = input.rev();
        let operations: Vec<&str> = input.next().unwrap().split_whitespace().collect();
        let mut numbers: Vec<Vec<u64>> = vec![Vec::new(); operations.len()];
        for line in input {
            for (index, number) in line.split_whitespace().enumerate() {
                numbers[index].push(number.parse().unwrap());
            }
        }
        let result: u64 = zip(operations, numbers).map(|(op, nums)| {
            nums.into_iter().reduce(|acc, num| if op == "+" {
                acc + num
            } else {
             acc * num
            }).unwrap()
        }).sum();
        println!("{result}");
    }
}
