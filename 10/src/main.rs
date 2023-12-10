#[derive(PartialEq, Clone, Copy, Debug)]
enum PipeType {
    Start,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    None,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

use PipeType::*;
use Direction::*;

fn main() {
    let pipes = std::fs::read_to_string("input").unwrap()
        .lines()
        .map(|line| line.chars()
             .map(|c| match c {
                 'S' => Start,
                 '|' => NS,
                 '-' => EW,
                 'L' => NE,
                 'J' => NW,
                 '7' => SW,
                 'F' => SE,
                 '.' => None,
                 _ => unreachable!(),
             }).collect::<Vec<PipeType>>()
        ).collect::<Vec<Vec<PipeType>>>();
    let mut current = (0, 0);
    let mut last = Direction::East;
    for (i, row) in pipes.iter().enumerate() {
        for (j, pipe) in row.iter().enumerate() {
            if *pipe == Start {
                (current, last) = match pipes[i][j-1] {
                    EW|NE|SE => ((j-1, i), West),
                    _ => match pipes[i][j+1] {
                        EW|NW|SW => ((j+1, i), East),
                        _ => ((j, i-1), North)
                    }
                }
            }
        }
    };
    let mut count = 1;
    loop {
        let (x, y) = current;
        let pipe = pipes[y][x];
        count += 1;
        match (pipe, &last) {
            (Start, _) => {
                println!("{}", count/2);
                break;
            }
            (NE, West) |
            (NW, East) |
            (NS, North) => {
                last = North;
                current = (x, y-1);
            }
            (SE, West) |
            (SW, East) |
            (NS, South) => {
                last = South;
                current = (x, y+1);
            }
            (NE, South) |
            (SE, North) |
            (EW, East) => {
                last = East;
                current = (x+1, y);
            }
            (NW, South) |
            (SW, North) |
            (EW, West) => {
                last = West;
                current = (x-1, y);
            }
            _ => unreachable!(),
        };
    }
}
