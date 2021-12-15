use comparator::collections::BinaryHeap;
use comparator::{comparing, Comparator};

type Position = (usize, usize);
type Val = (usize, usize);
type State = (usize, Position);
const REPEAT: usize = 5;

fn main() {
    // parse
    let lines: Vec<Vec<Val>> = include_str!("../input/15.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap() as usize, usize::MAX))
                .collect()
        })
        .collect();

    // expand grid
    let (max_i, max_j) = (lines.len(), lines[0].len());
    let mut grid: Vec<Vec<Val>> = vec![vec![(0, usize::MAX); max_i * REPEAT]; max_j * REPEAT];
    (0..max_i * REPEAT).for_each(|i| {
        (0..max_j * REPEAT).for_each(|j| {
            grid[i][j] = (
                (lines[i % max_i][j % max_j].0 + i / max_i + j / max_j + 8) % 9 + 1,
                usize::MAX,
            )
        })
    });

    // dijikstra's algo
    let mut heap = BinaryHeap::with_comparator(comparing(|s: &State| s.0).reversed());
    let max = (grid[0].len(), grid.len());
    heap.push(Default::default());

    while let Some((cost, position)) = heap.pop() {
        if cost <= grid[position.0][position.1].1 {
            for (i, j) in get_neighbors(position, max) {
                let (edge, old_cost) = grid[i][j];
                let next = (cost + edge, (i, j));
                if next.0 < old_cost {
                    grid[i][j].1 = next.0;
                    heap.push(next);
                }
            }
        }
    }
    println!("{}", grid.last().unwrap().last().unwrap().1);
}

fn get_neighbors((i, j): Position, (max_x, max_y): Position) -> Vec<Position> {
    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .map(|(dx, dy)| (i as isize + dx, j as isize + dy))
        .filter(|&(i, j)| i >= 0 && j >= 0)
        .map(|(i, j)| (i as usize, j as usize))
        .filter(|&(i, j)| i < max_x && j < max_y)
        .collect()
}
