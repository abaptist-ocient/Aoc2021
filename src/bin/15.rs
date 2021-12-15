use itertools::Itertools;
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const REPEAT: usize = 5;

fn main() {
    let lines = include_str!("../input/15.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap() as usize, usize::MAX))
                .collect_vec()
        })
        .collect_vec();

    let mut grid: Vec<Vec<(usize, usize)>> =
        vec![vec![(0, usize::MAX); lines.len() * REPEAT]; lines.len() * REPEAT];

    let max_i = lines.len();
    let max_j = lines[0].len();

    (0..max_i * REPEAT).for_each(|i| {
        (0..max_j * REPEAT).for_each(|j| {
            grid[i][j] = (
                (lines[i % max_i][j % max_j].0 + i / max_i + j / max_j + 8) % 9 + 1,
                usize::MAX,
            )
        })
    });
    let mut heap = BinaryHeap::<State>::new();

    heap.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost <= grid[position.0][position.1].1 {
            for (i, j) in get_neighbors(position, &grid) {
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

fn get_neighbors((i, j): (usize, usize), grid: &[Vec<(usize, usize)>]) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if i > 0 {
        neighbors.push((i - 1, j));
    }
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    if i + 1 < grid[0].len() {
        neighbors.push((i + 1, j));
    }
    if j + 1 < grid.len() {
        neighbors.push((i, j + 1));
    }
    neighbors
}
