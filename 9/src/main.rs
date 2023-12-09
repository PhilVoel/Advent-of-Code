fn main() {
    println!("{}", std::fs::read_to_string("input").unwrap()
             .lines()
             .map(|line| line.split(" ")
                  .filter(|s| !s.is_empty())
                  .map(|s| s.parse().unwrap())
                  .collect::<Vec<i64>>()
                  )
             .map(get_next)
             .sum::<i64>()
    );
}

fn get_next(nums: Vec<i64>) -> i64 {
    let mut steps = Vec::new(); 
    for i in 0..nums.len()-1 {
        steps.push(nums[i+1] - nums[i]);
    }
    nums.first().unwrap() - {
        if steps.iter().all(|s| *s==0) {
            0
        } else {
            get_next(steps)
        }
    }
}
