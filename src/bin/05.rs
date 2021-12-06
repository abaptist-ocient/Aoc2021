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
    x: usize,
    y: usize,
}
const MAX: usize = 1000;
fn main() {
    let lines = include_str!("../input/5.txt").lines();
    let lines: Vec<Line> = lines.map(|line| line.parse().unwrap()).collect();

    let mut board = vec![vec![0; MAX]; MAX];
    for line in lines {
        let x_diff = line.end.x as isize - line.start.x as isize;
        let y_diff = line.end.y as isize - line.start.y as isize;

        if x_diff == 0 || y_diff == 0 || x_diff.abs() == y_diff.abs() {
            for count in 0..=x_diff.abs().max(y_diff.abs()) {
                board[(line.start.x as isize + count * x_diff.signum()) as usize]
                    [(line.start.y as isize + count * y_diff.signum()) as usize] += 1;
            }
        }
    }

    println!(
        "{}",
        board.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc, &cell| acc + if cell > 1 { 1 } else { 0 })
        })
    );
}
