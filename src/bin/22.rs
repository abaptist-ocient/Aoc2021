#![feature(binary_heap_retain)]

use std::cmp::max;
use std::cmp::min;
use std::collections::BinaryHeap;

use parse_display::{Display, FromStr};
use State::*;

#[derive(Display, FromStr, Copy, Clone, PartialEq, Eq)]
enum State {
    #[display("on")]
    On,
    #[display("off")]
    Off,
}

#[derive(Display, FromStr, PartialEq, Eq, Clone)]
#[display("{state} x={min_x}..{max_x},y={min_y}..{max_y},z={min_z}..{max_z}")]
#[from_str(default_fields("order"))]
struct Cube {
    state: State,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
    order: usize,
}
struct KeyPoints {
    outer_left: Option<isize>,
    inner_left: isize,
    inner_right: isize,
    outer_right: Option<isize>,
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl Cube {
    fn fix_high_low(self, order: usize) -> Self {
        Cube {
            state: self.state,
            min_x: min(self.min_x, self.max_x),
            max_x: max(self.min_x, self.max_x),
            min_y: min(self.min_y, self.max_y),
            max_y: max(self.min_y, self.max_y),
            min_z: min(self.min_z, self.max_z),
            max_z: max(self.min_z, self.max_z),
            order,
        }
    }

    // do the cubes intersect each other at all
    fn intersects(&self, other: &Self) -> bool {
        let x_intercect = (self.min_x <= other.min_x && self.max_x >= other.min_x)
            || (other.min_x <= self.min_x && other.max_x >= self.min_x);
        let y_intersect = (self.min_y <= other.min_y && self.max_y >= other.min_y)
            || (other.min_y <= self.min_y && other.max_y >= self.min_y);
        let z_intersect = (self.min_z <= other.min_z && self.max_z >= other.min_z)
            || (other.min_z <= self.min_z && other.max_z >= self.min_z);

        x_intercect && y_intersect && z_intersect
    }

    // split other cube into up to 6 non intersecting sub-cubes such that the subcubes either fully or don't intersect with this cube
    fn split_cube(&self, other: &Self) -> Vec<Cube> {
        let x_points = get_points(self.min_x, self.max_x, other.min_x, other.max_x);
        let y_points = get_points(self.min_y, self.max_y, other.min_y, other.max_y);
        let z_points = get_points(self.min_z, self.max_z, other.min_z, other.max_z);

        // cube_top_x
        let mut cubes = Vec::new();
        if let Some(min_x) = x_points.outer_left {
            // full left side
            cubes.push(Cube {
                state: other.state,
                min_x,
                max_x: x_points.inner_left - 1,
                min_y: other.min_y,
                max_y: other.max_y,
                min_z: other.min_z,
                max_z: other.max_z,
                order: other.order,
            });
        }
        if let Some(max_x) = x_points.outer_right {
            // full right side
            cubes.push(Cube {
                state: other.state,
                min_x: x_points.inner_right + 1,
                max_x,
                min_y: other.min_y,
                max_y: other.max_y,
                min_z: other.min_z,
                max_z: other.max_z,
                order: other.order,
            });
        }
        if let Some(min_y) = y_points.outer_left {
            // full left side
            cubes.push(Cube {
                state: other.state,
                min_x: x_points.inner_left,
                max_x: x_points.inner_right,
                min_y,
                max_y: y_points.inner_left - 1,
                min_z: other.min_z,
                max_z: other.max_z,
                order: other.order,
            });
        }
        if let Some(max_y) = y_points.outer_right {
            // full left side
            cubes.push(Cube {
                state: other.state,
                min_x: x_points.inner_left,
                max_x: x_points.inner_right,
                min_y: y_points.inner_right + 1,
                max_y,
                min_z: other.min_z,
                max_z: other.max_z,
                order: other.order,
            });
        }
        if let Some(min_z) = z_points.outer_left {
            // full left side
            cubes.push(Cube {
                state: other.state,
                min_x: x_points.inner_left,
                max_x: x_points.inner_right,
                min_y: y_points.inner_left,
                max_y: y_points.inner_right,
                min_z,
                max_z: z_points.inner_left - 1,
                order: other.order,
            });
        }
        if let Some(max_z) = z_points.outer_right {
            // full left`side
            cubes.push(Cube {
                state: other.state,
                min_x: x_points.inner_left,
                max_x: x_points.inner_right,
                min_y: y_points.inner_left,
                max_y: y_points.inner_right,
                min_z: z_points.inner_right + 1,
                max_z,
                order: other.order,
            });
        }

        cubes
    }

    fn size(&self) -> isize {
        assert!(self.max_x >= self.min_x);
        assert!(self.max_y >= self.min_y);
        assert!(self.max_z >= self.min_z);
        (1 + self.max_x - self.min_x)
            * (1 + self.max_y - self.min_y)
            * (1 + self.max_z - self.min_z)
    }
}
fn get_points(p1_min: isize, p1_max: isize, p2_min: isize, p2_max: isize) -> KeyPoints {
    KeyPoints {
        outer_left: if p2_min < p1_min { Some(p2_min) } else { None },
        inner_left: max(p1_min, p2_min),
        inner_right: min(p1_max, p2_max),
        outer_right: if p2_max > p1_max { Some(p2_max) } else { None },
    }
}

const INTERIOR: Cube = Cube {
    min_x: -50,
    max_x: 50,
    min_y: -50,
    max_y: 50,
    min_z: -50,
    max_z: 50,
    state: Off,
    order: 0,
};

fn main() {
    let mut cubes: BinaryHeap<Cube> = include_str!("../input/22.txt")
        .lines()
        .enumerate()
        .map(|(order, line)| line.parse::<Cube>().unwrap().fix_high_low(order))
        // comment out this line for part 2
        .filter(|cube| cube.intersects(&INTERIOR))
        .collect();

    let mut final_cubes = Vec::new();
    while let Some(top_cube) = cubes.pop() {
        let mut added_cubes = Vec::new();
        // remove all the overlapping cube "parts"
        // would be cleaner once https://github.com/rust-lang/rfcs/issues/2140 is implemented
        cubes.retain(|cube| {
            if top_cube.intersects(cube) {
                added_cubes.extend(top_cube.split_cube(cube).into_iter());
                false
            } else {
                true
            }
        });
        cubes.extend(added_cubes.into_iter());
        if top_cube.state == On {
            final_cubes.push(top_cube);
        }
    }
    println!("{}", final_cubes.iter().map(Cube::size).sum::<isize>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_test() {
        let cube1: Cube = "on x=-26..27,y=-45..3,z=-20..24".parse().unwrap();
        let cube2: Cube = "on x=-41..4,y=-11..41,z=-36..16".parse().unwrap();
        let result = cube1.split_cube(&cube2);
        for cube in result {
            println!("{}", cube);
        }
    }
}
