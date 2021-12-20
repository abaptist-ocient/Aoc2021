use std::{
    fmt::{Display, Formatter},
    ops::Add,
};

use Explosion::*;
use Value::*;

#[derive(Debug, Clone)]
enum Value {
    More(Box<Pair>),
    Digit(u32),
}

#[derive(Debug, PartialEq)]
enum Explosion {
    None,
    Exploded((u32, u32)),
    Propogate((u32, u32)),
}

#[derive(Debug, Clone)]
struct Pair {
    left: Value,
    right: Value,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            More(pair) => write!(f, "{}", *pair),
            Digit(d) => write!(f, "{}", d),
        }
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut p = Pair {
            left: More(Box::new(self)),
            right: More(Box::new(other)),
        };
            while p.explode(0) != None || p.split() {
            }
        p
    }
}

impl Pair {
    fn parse(s: &str) -> Pair {
        let stack = s.chars().fold(Vec::new(), |mut stack, c| {
            match c {
                '0'..='9' => stack.push(Digit(c.to_digit(10).unwrap())),
                ']' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(More(Box::new(Pair { left, right })));
                }
                '[' | ',' => {} // ignore
                _ => unreachable!("{}", c),
            }
            stack
        });
        let stack = stack.into_iter().next().unwrap();
        if let More(r) = stack {
            *r
        } else {
            unreachable!("Bad stack {:?}", stack);
        }
    }

    fn split(&mut self) -> bool {
        match &mut self.left {
            Digit(n) => {
                if *n > 9 {
                    self.left = Pair::create_split(*n);
                    true
                } else {
                    self.split_right()
                }
            }
            More(value) => {
                if Pair::split(value.as_mut()) {
                    true
                } else {
                    self.split_right()
                }
            }
        }
    }

    fn create_split(n: u32) -> Value {
        More(Box::new(Pair {
            left: Digit(n / 2),
            right: Digit((n + 1) / 2),
        }))
    }

    fn split_right(&mut self) -> bool {
        match &mut self.right {
            Digit(n) => {
                if *n > 9 {
                    self.right = Pair::create_split(*n);
                    true
                } else {
                    false
                }
            }
            More(value) => Pair::split(value.as_mut()),
        }
    }

    fn explode(&mut self, depth: u32) -> Explosion {
        if depth == 4 {
            self.explode_me()
        } else if let More(value) = &mut self.left {
            match Pair::explode(value.as_mut(), depth + 1) {
                Exploded((a, b)) => {
                    self.left = Digit(0);
                    self.propogate_left(a, b)
                }
                Propogate((a, b)) => self.propogate_left(a, b),
                None => self.explode_right(depth),
            }
        } else {
            self.explode_right(depth)
        }
    }

    fn explode_right(&mut self, depth: u32) -> Explosion {
        if let More(value) = &mut self.right {
            match Pair::explode(value.as_mut(), depth + 1) {
                Exploded((a, b)) => {
                    self.right = Digit(0);
                    self.propogate_right(a, b)
                }
                Propogate((a, b)) => self.propogate_right(a, b),
                None => None,
            }
        } else {
            None
        }
    }

    fn propogate_left(&mut self, a: u32, b: u32) -> Explosion {
        match &mut self.right {
            More(node) => node.as_mut().add_leftmost(b),
            Digit(num) => *num += b,
        }
        Propogate((a, 0))
    }
    fn add_leftmost(&mut self, val: u32) {
        match &mut self.left {
            More(node) => node.as_mut().add_leftmost(val),
            Digit(num) => *num += val,
        }
    }

    fn propogate_right(&mut self, a: u32, b: u32) -> Explosion {
        match &mut self.left {
            More(node) => node.as_mut().add_rightmost(a),
            Digit(num) => *num += a,
        }
        Propogate((0, b))
    }
    fn add_rightmost(&mut self, val: u32) {
        match &mut self.right {
            More(node) => node.as_mut().add_rightmost(val),
            Digit(num) => *num += val,
        }
    }

    fn explode_me(&mut self) -> Explosion {
        if let (Digit(left), Digit(right)) = (&mut self.left, &mut self.right) {
            Exploded((*left, *right))
        } else {
            unreachable!("Too deep already... {}", self);
        }
    }

    fn magnitude(self) -> u32 {
        (match self.left {
            More(node) => node.magnitude(),
            Digit(num) => num,
        }) * 3
            + (match self.right {
                More(node) => node.magnitude(),
                Digit(num) => num,
            }) * 2
    }
}

fn main() {
    let magnitude = include_str!("../input/18.txt")
        .lines()
        .map(Pair::parse)
        .reduce(|p1, p2| p1 + p2)
        .unwrap()
        .magnitude();
    println!("{}", magnitude);
    let lines: Vec<_> = include_str!("../input/18.txt")
        .lines()
        .map(Pair::parse)
        .collect();
    let mut max = 0;
    (0..lines.len()).for_each(|i| {
        (0..lines.len()).for_each(|j| {
            if i != j {
                let sum = lines[i].clone() + lines[j].clone();
                let mag = sum.magnitude();
                if mag > max {
                    max = mag
                };
            }
        });
    });
    println!("{}", max);
}
