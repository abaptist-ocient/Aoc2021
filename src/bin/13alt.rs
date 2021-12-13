#![feature(int_abs_diff)]

fn uint(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn main() {
    if let Some((points, folds)) = include_str!("../input/13.txt").split_once("\n\n") {
        let mut points: Vec<_> = points .lines() .map(|p| p.split_once(',').map(|(x, y)| (uint(y), uint(x))).unwrap()) .collect();
        let folds: Vec<_> = folds .lines() .map(|f| { f.split_once('=') .map(|(d, v)| (d.ends_with('x'), uint(v))) .unwrap() }) .collect();

        for fold in folds {
            for (x, y) in &mut points {
                let coord = if fold.0 { y } else { x };
                *coord = fold.1 - coord.abs_diff(fold.1);
            }
        }
        (0..=points.iter().fold(0, |b, &p| std::cmp::max(b, p.0))).for_each(|i| {
            (0..=points.iter().fold(0, |b, &p| std::cmp::max(b, p.1))).for_each(|j| {
                print!("{}", if points.contains(&(i, j)) { '#' } else { ' ' });
            });
            println!();
        });
    }
}
