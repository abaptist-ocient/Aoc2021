use std::collections::HashMap;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display("{start} -> {end}")]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Display, FromStr, Debug)]
#[display("{x},{y}")]
struct Point {
    x: isize,
    y: isize,
}
fn main() {
    let lines = include_str!("../input/5.txt").lines();
    let lines: Vec<Line> = lines.map(|line| line.parse().unwrap()).collect();

    let mut board = HashMap::new();
    for line in lines {
        let x_diff = line.end.x - line.start.x;
        let y_diff = line.end.y - line.start.y;

        if x_diff == 0 || y_diff == 0 || x_diff.abs() == y_diff.abs() {
            for count in 0..=x_diff.abs().max(y_diff.abs()) {
                *board
                    .entry((
                        (line.start.x + count * x_diff.signum()),
                        (line.start.y + count * y_diff.signum()),
                    ))
                    .or_insert(0) += 1;
            }
        }
    }

    println!("{}", board.values().filter(|&val| val > &1).count());
}
