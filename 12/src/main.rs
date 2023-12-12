fn main() {
    println!("{}", std::fs::read_to_string("input").unwrap()
             .lines()
             .map(|l| {
                 let (springs, nums) = l.split_once(' ').unwrap();
                 let mut springs = springs.chars().collect::<Vec<_>>();
                 let nums = nums.split(',')
                     .map(|n| n.parse::<i64>().unwrap())
                     .collect::<Vec<_>>();
                 count(&mut springs, &nums)
             }).sum::<u64>());
}

fn count(springs: &mut Vec<char>, nums: &Vec<i64>) -> u64 {
    if let Some(i) = springs.iter().position(|c| *c == '?') {
        springs[i] = '.';
        let first = count(springs, nums);
        springs[i] = '#';
        let second = count(springs, nums);
        springs[i] = '?';
        first + second
    }
    else {
        let mut found_nums = Vec::new();
        let mut count = 0;
        let mut in_group = false;
        for c in springs.iter() {
            if *c == '#' {
                if !in_group {
                    in_group = true;
                }
                count += 1;
            }
            else if in_group {
                found_nums.push(count);
                count = 0;
                in_group = false;
            }
        }
        if in_group {
            found_nums.push(count);
        }
        if nums == &found_nums {
            1
        }
        else {
            0
        }
    }
}
