struct Board {
    grid: Vec<Vec<char>>,
    x_len: usize,
    y_len: usize,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().try_for_each(|row| {
            row.iter().try_for_each(|square| write!(f, "{}", square))?;
            writeln!(f)
        })
    }
}

impl Board {
    fn swim_south(&mut self) -> usize {
        let mut moves = Vec::new();
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if self.grid[y][x] == 'v' && self.grid[(y + 1) % self.y_len][x] == '.' {
                    moves.push((x, y));
                }
            }
        }
        let count = moves.len();
        for (x, y) in moves {
            self.grid[y][x] = '.';
            self.grid[(y + 1) % self.y_len][x] = 'v'
        }
        count
    }

    fn swim_east(&mut self) -> usize {
        let mut moves = Vec::new();
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if self.grid[y][x] == '>' && self.grid[y][(x + 1) % self.x_len] == '.' {
                    moves.push((x, y));
                }
            }
        }
        let count = moves.len();
        for (x, y) in moves {
            self.grid[y][x] = '.';
            self.grid[y][(x + 1) % self.x_len] = '>'
        }
        count
    }

    fn new(grid: Vec<Vec<char>>) -> Self {
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
            .map(|line| line.chars().collect())
            .collect(),
    );

    let mut count = 1;
    while board.swim_east() + board.swim_south() > 0 {
        count += 1;
    }
    println!("{}", board);
    println!("P1 {}", count);
}
