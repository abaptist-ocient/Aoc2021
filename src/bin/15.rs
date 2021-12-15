type Position = (usize, usize);
type Val = (usize, usize);
const REPEAT: usize = 5;

#[derive(Debug, Eq, PartialEq, Default)]
struct State {
    cost: usize,
    position: Position,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let lines: Vec<Vec<Val>> = include_str!("../input/15.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap() as usize, usize::MAX))
                .collect()
        })
        .collect();

    let max_i = lines.len();
    let max_j = lines[0].len();
    let mut grid: Vec<Vec<Val>> = vec![vec![(0, usize::MAX); max_i * REPEAT]; max_j * REPEAT];

    (0..max_i * REPEAT).for_each(|i| {
        (0..max_j * REPEAT).for_each(|j| {
            grid[i][j] = (
                (lines[i % max_i][j % max_j].0 + i / max_i + j / max_j + 8) % 9 + 1,
                usize::MAX,
            )
        })
    });
    grid[0][0].1 = 0;
    let mut heap = std::collections::BinaryHeap::<State>::new();
    heap.push(Default::default());

    while let Some(State { cost, position }) = heap.pop() {
        if cost <= grid[position.0][position.1].1 {
            for (i, j) in get_neighbors(position, (grid[0].len(), grid.len())) {
                let (edge, old_cost) = grid[i][j];
                let next = State {
                    cost: cost + edge,
                    position: (i, j),
                };
                if next.cost < old_cost {
                    grid[i][j].1 = next.cost;
                    heap.push(next);
                }
            }
        }
    }
    println!("{}", grid.last().unwrap().last().unwrap().1);
}

fn get_neighbors((i, j): Position, (x, y): Position) -> Vec<Position> {
    let mut neighbors = Vec::new();
    if i > 0 {
        neighbors.push((i - 1, j));
    }
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    if i + 1 < x {
        neighbors.push((i + 1, j));
    }
    if j + 1 < y {
        neighbors.push((i, j + 1));
    }
    neighbors
}
