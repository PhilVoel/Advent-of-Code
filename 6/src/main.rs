fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines = input.lines()
        .map(|l| 
             l.split_once(':').unwrap()
             .1
             .split(' ')
             .filter(|x| !x.is_empty())
             .map(|x| x.parse::<u32>().unwrap())
             .collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>();
    println!("{:?}", lines[0].iter().enumerate()
        .map(|(i,time)| {
            let record = lines[1][i];
            for j in 0..*time {
                if (time-j) * j > record {
                    return time+1-2*(j);
                }
            }
            return 0;
        })
        .reduce(|a,b| a*b).unwrap()
    );
}
