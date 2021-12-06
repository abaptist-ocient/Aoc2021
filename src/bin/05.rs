use std::cmp::max;

//type Point = (usize, usize);
struct Line {
    start: Point,
    end: Point,
}
struct Point {
    x: usize,
    y: usize,
}
const MAX: usize = 1000;
fn main() {
    let lines = include_str!("../input/5.txt").lines();
    let lines: Vec<Line> = lines
        .map(|line| {
            line.split_once(" -> ")
                .map(|(p1, p2)| Line {
                    start: parse_point(p1),
                    end: parse_point(p2),
                })
                .unwrap()
        })
        .collect();

    fn parse_point(p: &str) -> Point {
        p.split_once(',')
            .map(|(x, y)| Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
            .unwrap()
    }

    let mut board = vec![vec![0; MAX]; MAX];
    for line in lines {
        let x_diff = line.end.x as i32 - line.start.x as i32;
        let y_diff = line.end.y as i32 - line.start.y as i32;

        if x_diff == 0 || y_diff == 0 || x_diff.abs() == y_diff.abs() {
            for count in 0..=max(x_diff.abs(), y_diff.abs()) {
                board[(line.start.x as i32 + count * x_diff.signum()) as usize]
                    [(line.start.y as i32 + count * y_diff.signum()) as usize] += 1;
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
