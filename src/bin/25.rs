use std::fmt::Display;

use itertools::Itertools;
use Square::*;

#[derive(Debug, PartialEq)]
enum Square {
    Down,
    Right,
    Empty,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Down => 'v',
                Right => '>',
                Empty => '.',
            }
        )
    }
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Square>>,
    x_len: usize,
    y_len: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().try_for_each(|row| {
            row.iter().try_for_each(|square| write!(f, "{}", square))?;
            writeln!(f)
        })
    }
}

impl Board {
    fn swim(&mut self) -> bool {
        let east_count = self.swim_east();
        let south_count = self.swim_south();
        east_count + south_count > 0
    }
    fn swim_south(&mut self) -> usize {
        let mut moves = Vec::new();
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if self.grid[y][x] == Down && self.grid[(y + 1) % self.y_len][x] == Empty {
                    moves.push((x, y));
                }
            }
        }
        let count = moves.len();
        for (x, y) in moves {
            self.grid[y][x] = Empty;
            self.grid[(y + 1) % self.y_len][x] = Down
        }
        count
    }

    fn swim_east(&mut self) -> usize {
        let mut moves = Vec::new();
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if self.grid[y][x] == Right && self.grid[y][(x + 1) % self.x_len] == Empty {
                    moves.push((x, y));
                }
            }
        }
        let count = moves.len();
        for (x, y) in moves {
            self.grid[y][x] = Empty;
            self.grid[y][(x + 1) % self.x_len] = Right
        }
        count
    }

    fn new(grid: Vec<Vec<Square>>) -> Self {
        Board {
            y_len: grid.len(),
            x_len: grid[0].len(),
            grid,
        }
    }
}

fn main() {
    let mut board = Board::new(
        include_str!("../input/25.txt")
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'v' => Down,
                        '>' => Right,
                        _ => Empty,
                    })
                    .collect_vec()
            })
            .collect_vec(),
    );

    let mut count = 1;
    while board.swim() {
        count += 1;
        //        println!("{}", board);
    }
    println!("{}", board);
    println!("P1 {}", count);
}
