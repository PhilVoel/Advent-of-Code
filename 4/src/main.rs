use std::collections::HashSet;

fn main() {
    let mut nums = vec![1; 203];
    for (line_nr, line) in std::fs::read_to_string("input").unwrap()
        .lines()
        .enumerate() {
            let (all_win, all_have) = line.split_once(':').unwrap()
                .1
                .split_once('|').unwrap();
            let mut wins: HashSet<&str> = HashSet::new();
            for n in all_win.split(' ') {
                if n == "" {
                    continue;
                }
                wins.insert(n);
            }
            for i in (line_nr+1)..=(line_nr+all_have.split(' ')
                               .filter(|n| wins.contains(n))
                               .count()) {
                nums[i] += nums[line_nr];
            }
    };
    println!("{}", nums.iter().sum::<i64>());
}
