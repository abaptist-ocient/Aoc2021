use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display("target area: x={min_x}..{max_x}, y={min_y}..{max_y}")]
struct Bounds {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Bounds {
    fn contains_x(&self, x: isize) -> bool {
        x >= self.min_x && x <= self.max_x
    }
    fn contains_y(&self, y: isize) -> bool {
        y >= self.min_y && y <= self.max_y
    }
}

fn main() {
    let b: Bounds = include_str!("../input/17.txt")
        .lines()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    println!("part 1 {:?}", (b.min_y * b.min_y + 1) / 2);
    let count: usize = (0..=b.max_x)
        .filter(|&x| can_hit_x(&b, x))
        .map(|x| {
            (b.min_y..=b.min_y.abs())
                .filter(|&y| hits(&b, x, y))
                .count()
        })
        .sum();
    println!("part 2 {:?}", count);
}

fn hits(b: &Bounds, mut x_vel: isize, mut y_vel: isize) -> bool {
    let mut x = 0;
    let mut y = 0;
    while x < b.max_x && y >= b.min_y {
        x += x_vel;
        y += y_vel;
        x_vel = std::cmp::max(0, x_vel - 1);
        y_vel -= 1;
        if b.contains_x(x) && b.contains_y(y) {
            return true;
        }
    }
    false
}

fn can_hit_x(b: &Bounds, max_x: isize) -> bool {
    let mut cur_x = 0;
    for x in (0..=max_x).rev() {
        cur_x += x;
        if b.contains_x(cur_x) {
            return true;
        }
    }
    false
}
