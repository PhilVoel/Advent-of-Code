#[derive(PartialEq, Clone, Copy, Debug)]
enum PipeType {
    Start,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    NN,
}

impl PipeType {
    fn north(&self) -> bool {
        match self {
            NS|NE|NW => true,
            _ => false,
        }
    }

    fn south(&self) -> bool {
        match self {
            NS|SE|SW => true,
            _ => false,
        }
    }

    fn west(&self) -> bool {
        match self {
            NW|SW|EW => true,
            _ => false,
        }
    }
}

type PipePart = (PipeType, bool);

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
    let mut pipes = std::fs::read_to_string("input").unwrap()
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
                 '.' => NN,
                 _ => unreachable!(),
             }).map(|p| (p, false))
            .collect::<Vec<PipePart>>()
        ).collect::<Vec<Vec<PipePart>>>();
    let mut current = (0, 0);
    let mut last = Direction::East;
    let mut first = (NN, false);
    let mut start = (0, 0);
    for (i, row) in pipes.iter().enumerate() {
        for (j, pipe) in row.iter().enumerate() {
            if pipe.0 == Start {
                start = (j, i);
                let (n, s, e) = (pipes[i-1][j].0.south(), pipes[i+1][j].0.north(), pipes[i][j+1].0.west());
                if n {
                    current = (j, i-1);
                    last = North;
                    if s {
                        first = (NS, true);
                    }
                    else if e {
                        first = (NE, true);
                    }
                    else {
                        first = (NW, true);
                    }
                }
                else if s {
                    current = (j, i+1);
                    last = South;
                    if e {
                        first = (SE, true);
                    }
                    else {
                        first = (SW, true);
                    }
                }
                else {
                    current = (j+1, i);
                    first = (EW, true);
                    last = East;
                }
            }
        }
    };
    pipes[start.1][start.0] = first;
    loop {
        let (x, y) = current;
        let pipe = pipes[y][x];
        if pipe.1 {
            break;
        }
        pipes[y][x] = (pipe.0, true);
        match (pipe.0, &last) {
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
            _ => unreachable!("{x} {y} {:?} {:?}", last, pipe.0),
        };
    }
    let mut count = 0;
    for row in pipes {
        let mut inside = false;
        let mut first_pipe = NN;
        for pipe in row {
            if pipe.1 {
                match (pipe.0, first_pipe) {
                    (NS, NN) => inside = !inside,
                    (p, NN) => first_pipe = p,
                    (p, f) if p.north() && f.north() ||
                        p.south() && f.south() => first_pipe = NN,
                    (p, f) if p.north() && f.south() ||
                        p.south() && f.north() => {
                            inside = !inside;
                            first_pipe = NN;
                        }
                    _ => (),
                }
            }
            else if inside {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
