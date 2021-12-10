use itertools::Itertools;

// derived from https://github.com/timvisee/advent-of-code-2021/blob/master/day08b/src/main.rs as a cool solution
// idea for part 2 is to get the first nums (1, 7, 4, 8) and then subract 1 and 4 from each other number
pub fn main() {
    let lines = include_str!("../input/8.txt").lines().collect_vec();

    println!(
        "part 1 {}",
        lines
            .iter()
            .map(|line| line
                .split_once('|')
                .unwrap()
                .1
                .split_ascii_whitespace()
                .filter(|d| matches!(d.len(), 2 | 3 | 4 | 7))
                .count())
            .sum::<usize>()
    );

    println!(
        "part 2 {}",
        lines
            .iter()
            .map(|line| {
                let part = line.split_once('|').unwrap();
                let input = part.0.split_ascii_whitespace().collect_vec();
                let one = input.iter().find(|d| d.len() == 2).unwrap();
                let four = input.iter().find(|d| d.len() == 4).unwrap();
                part.1
                    .split_ascii_whitespace()
                    .map(|d| match d.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        len => match (
                            len,
                            d.chars().filter(|&b| one.contains(b)).count(),
                            d.chars().filter(|&b| four.contains(b)).count(),
                        ) {
                            (5, 1, 3) => 5,
                            (5, 2, 3) => 3,
                            (5, 1, 2) => 2,
                            (6, 1, 3) => 6,
                            (6, 2, 3) => 0,
                            (6, 2, 4) => 9,
                            _ => unreachable!(),
                        },
                    })
                    .fold(0, |sum, n| sum * 10 + n)
            })
            .sum::<u32>()
    );
}
