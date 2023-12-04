use std::collections::HashSet;

fn main() {
    println!("{}", std::fs::read_to_string("input").unwrap()
             .lines()
             .map(|line| {
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
                 let mut score = 0;
                 for n in all_have.split(' ') {
                     if wins.contains(n) {
                         if score == 0 {
                             score = 1;
                         } else {
                             score *= 2;
                         }
                     }
                 }
                 score
             })
             .sum::<i32>()
    );
}
