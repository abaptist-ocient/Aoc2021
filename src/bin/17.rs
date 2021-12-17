use std::{cmp::max, ops::RangeInclusive};

const X_RANGE: RangeInclusive<isize> = 207..=263;
const Y_RANGE: RangeInclusive<isize> = -115..=-63;
fn main() {
    let count = (0..=X_RANGE.max().unwrap())
        .filter(|x| can_hit_x(*x))
        .flat_map(|x| {
            (Y_RANGE.min().unwrap()..=Y_RANGE.min().unwrap().abs())
                .filter(move |y: &isize| hits(x, *y))
        })
        .count();
    println!("{:?}", count);
}

fn hits(mut x_vel: isize, mut y_vel: isize) -> bool {
    let mut x = 0;
    let mut y = 0;
    while x < X_RANGE.last().unwrap() && y > Y_RANGE.min().unwrap() {
        x += x_vel;
        y += y_vel;
        x_vel = max(0, x_vel - 1);
        y_vel -= 1;
        if X_RANGE.contains(&x) && Y_RANGE.contains(&y) {
            return true;
        }
    }
    false
}

fn can_hit_x(max_x: isize) -> bool {
    let mut cur_x = 0;
    for x in (0..=max_x).rev() {
        cur_x += x;
        if X_RANGE.contains(&cur_x) {
            return true;
        }
    }
    false
}
