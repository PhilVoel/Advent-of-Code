use std::{cmp::max, collections::HashSet, env::args, fs};

fn part1(first: &str, last: &str) -> u128 {
    let first_num = if first.len() % 2 == 1 {
        10_u128.pow((first.len() / 2).try_into().unwrap())
    } else {
        let (start, end) = first.split_at(first.len() / 2);
        let start_num = start.parse().unwrap();
        let end_num = end.parse().unwrap();
        if start_num >= end_num {
            start_num
        } else {
            start_num + 1
        }
    };

    let last_num = if last.len() % 2 == 1 {
        10_u128.pow((TryInto::<u32>::try_into(last.len()).unwrap() - 1) / 2) - 1
    } else {
        let (start, end) = last.split_at(last.len() / 2);
        let start_num = start.parse().unwrap();
        let end_num = end.parse().unwrap();
        if start_num <= end_num {
            start_num
        } else {
            start_num - 1
        }
    };

    let mut sum = 0;
    for n in first_num..=last_num {
        let shifted = n * 10_u128.pow(f64::log10(n as f64).floor() as u32 + 1);
        sum += n + shifted;
    }
    sum
}

fn part2(first: &str, last: &str) -> u128 {
    let mut results = HashSet::new();
    let first_num = first.parse().unwrap();
    let last_num = last.parse().unwrap();
    for part_len in 1..=(last.len() / 2) {
        for target_range_len in max(2, first.len())..=last.len() {
            if target_range_len % part_len == 0 {
                let start = 10_u128.pow(TryInto::<u32>::try_into(part_len).unwrap() - 1);
                let end = 10_u128.pow(part_len.try_into().unwrap()) - 1;
                for part in start..=end {
                    let mut current = part;
                    for i in 1..target_range_len/part_len {
                        current += 10_u128.pow(TryInto::<u32>::try_into(i).unwrap() * TryInto::<u32>::try_into(part_len).unwrap()) * part;
                    }
                    if current < first_num {
                        continue;
                    }
                    if current > last_num {
                        break;
                    }
                    results.insert(current);
                }
            }
        }
    }
    results.into_iter().sum()
}

fn main() {
    let result: u128 = fs::read_to_string("input.txt").unwrap()
        .trim()
        .split(",")
        .map(|range| {
            let (first, last) = range.split_once("-").unwrap();
            
            if args().any(|a| a == "--part2") {
                part2(first, last)
            } else {
                part1(first, last)
            }
        })
        .sum();
    println!("{result}");
}
