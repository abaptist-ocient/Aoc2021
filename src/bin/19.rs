use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};
use itertools::Itertools;

type PointIndexPair<'a> = (&'a (usize, usize), &'a (usize, usize));

#[derive(Debug, Clone)]
struct Scanner {
    points: Vec<Point>,
    translation: Point,
}

impl FromIterator<Point> for Scanner {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        Scanner {
            points: iter.into_iter().collect(),
            translation: Point { x: 0, y: 0, z: 0 },
        }
    }
}

impl Scanner {
    fn get_dists(&self) -> Vec<(usize, (usize, usize))> {
        let mut all_dist: Vec<_> = self
            .points
            .iter()
            .enumerate()
            .flat_map(|(n1, first)| {
                self.points
                    .iter()
                    .enumerate()
                    .filter(|(n2, _)| n1 < *n2)
                    .map(|(n2, second)| (first.distance_from(second), (n1, n2)))
                    .collect_vec()
            })
            .collect();
        all_dist.sort_unstable();
        all_dist
    }

    fn apply_permutation(&mut self, perm: &Point) {
        self.points = self
            .points
            .iter()
            .map(|p| p.apply_permutation(perm))
            .collect();
    }

    fn apply_translation(&mut self, trans: &Point) {
        self.translation = trans.clone();
        self.points = self.points.iter().map(|p| p + trans).collect();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl FromIterator<isize> for Point {
    fn from_iter<T: IntoIterator<Item = isize>>(iter: T) -> Self {
        let v: Vec<_> = iter.into_iter().collect();
        Point {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Point {
    fn distance_from(&self, other: &Point) -> usize {
        let x_diff: usize = (other.x - self.x).abs() as usize;
        let y_diff: usize = (other.y - self.y).abs() as usize;
        let z_diff: usize = (other.z - self.z).abs() as usize;
        x_diff * x_diff + y_diff * y_diff + z_diff * z_diff
    }

    fn manhatten(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
    fn apply_permutation(&self, perm: &Point) -> Self {
        Point {
            x: self.get_digit(perm.x),
            y: self.get_digit(perm.y),
            z: self.get_digit(perm.z),
        }
    }

    fn get_digit(&self, val: isize) -> isize {
        val.signum()
            * match val.abs() {
                1 => self.x,
                2 => self.y,
                3 => self.z,
                _ => unreachable!("Illegal val"),
            }
    }
}

// return 48 permutations -  1 = x, 2 = y, 3 = z
fn get_permutations() -> Vec<Point> {
    [1, 2, 3]
        .iter()
        .permutations(3)
        .flat_map(|item| {
            (0..8).map(move |x| Point {
                x: item[0] * if x & 4 == 0 { -1 } else { 1 },
                y: item[1] * if x & 2 == 0 { -1 } else { 1 },
                z: item[2] * if x & 1 == 0 { -1 } else { 1 },
            })
        })
        .collect_vec()
}

fn main() {
    let mut scanners = parse_input();
    let dists: Vec<_> = scanners.iter().map(|scanner| scanner.get_dists()).collect();
    let mut overlaps = dists
        .iter()
        .enumerate()
        .flat_map(|(n1, first)| {
            dists
                .iter()
                .enumerate()
                .filter(|(n2, _)| n1 != *n2)
                .map(|(n2, second)| ((n1, n2), compute_overlaps(first, second)))
                .filter(|x| x.1.len() >= 66)
                .collect_vec()
        })
        .collect_vec();
    overlaps.sort_unstable();

    let mut completed = vec![false; scanners.len()];
    // can set reference frame anywhere - choose 0
    completed[0] = true;

    loop {
        let finish = overlaps
            .iter()
            .filter(|overlap| completed[overlap.0 .0] && !completed[overlap.0 .1])
            .map(|overlap| {
                if !completed[overlap.0 .1] {
                    let overlap_points = &overlap.1;

                    let rotations = find_rotation(
                        &scanners[overlap.0 .0],
                        &scanners[overlap.0 .1],
                        overlap_points,
                    );
                    // always get 2, one is the "mirror" which won't be very effective - but hard to detect which is which
                    assert!(rotations.len() == 2);
                    let orig = scanners[overlap.0 .1].points.clone();

                    for rotation in rotations {
                        scanners[overlap.0 .1].apply_permutation(&rotation);

                        // find the best translation
                        let mut map: HashMap<Point, usize> = HashMap::new();
                        overlap_points.iter().for_each(|point| {
                            let translation = &scanners[overlap.0 .0].points[point.0 .0]
                                - &scanners[overlap.0 .1].points[point.1 .0];
                            *map.entry(translation).or_default() += 1;
                        });
                        let map: HashMap<_, _> =
                            map.iter().filter(|(_, &count)| count > 12).collect();
                        if map.is_empty() {
                            // choose the mirror translation - try again
                            scanners[overlap.0 .1].points = orig.clone();
                            continue;
                        }
                        let translation = *map.keys().next().unwrap();
                        let origin = Point { x: 0, y: 0, z: 0 };
                        // useless case - we try and do the same scanner twice in one iteration
                        if translation != &origin {
                            scanners[overlap.0 .1].apply_translation(translation);
                            break;
                        }
                    }
                }
                overlap.0 .1
            })
            .collect_vec();
        finish.iter().for_each(|&x| completed[x] = true);
        if finish.is_empty() {
            break;
        }
    }

    println!("Part 1 {}", count_points(&scanners));

    let max = scanners
        .iter()
        .map(|el1| {
            scanners
                .iter()
                .map(|el2| (&el1.translation - &el2.translation).manhatten())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("Part 2 {}", max);
}

fn parse_input() -> Vec<Scanner> {
    include_str!("../input/19.txt")
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|point| {
                    point
                        .split(',')
                        .map(|p| p.parse::<isize>().unwrap())
                        .collect::<Point>()
                })
                .collect::<Scanner>()
        })
        .collect::<Vec<_>>()
}

fn count_points(scanners: &[Scanner]) -> usize {
    let mut all_points = HashSet::new();
    for scanner in scanners {
        for p in scanner.points.iter() {
            all_points.insert(p);
        }
    }
    all_points.into_iter().collect_vec().len()
}

fn find_rotation(
    scanner0: &Scanner,
    scanner1: &Scanner,
    overlap_points: &[PointIndexPair],
) -> HashSet<Point> {
    overlap_points
        .iter()
        .filter_map(|points| {
            let source = &scanner1.points[points.1 .0];
            let dest = &scanner1.points[points.1 .1];
            let goal = &scanner0.points[points.0 .0] - &scanner0.points[points.0 .1];
            let valid_perm = get_permutations()
                .into_iter()
                .filter(|p| &source.apply_permutation(p) - &dest.apply_permutation(p) == goal)
                .collect_vec();
            if valid_perm.len() == 1 {
                Some(valid_perm.into_iter().next().unwrap())
            } else {
                None
            }
        })
        .collect()
}

// dists that are identical between scanners
fn compute_overlaps<'a>(
    first: &'a [(usize, (usize, usize))],
    second: &'a [(usize, (usize, usize))],
) -> Vec<PointIndexPair<'a>> {
    first
        .iter()
        .flat_map(|(dist1, points1)| {
            second
                .iter()
                .filter(|(dist2, (_, _))| dist1 == dist2)
                .map(|(_, points2)| (points1, points2))
                .collect_vec()
        })
        .collect()
}
