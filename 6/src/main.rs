fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines = input.lines()
        .map(|l| 
             l.split_once(':').unwrap()
             .1
             .split(' ')
             .filter(|x| !x.is_empty())
             .collect::<Vec<&str>>()
             .join("")
             .parse::<u64>().unwrap()
            ).collect::<Vec<u64>>();
    let time = lines[0];
    for j in 0..time {
        if (time-j) * j > lines[1] {
            println!("{}", time+1-2*(j));
            return;
        }
    }
}
