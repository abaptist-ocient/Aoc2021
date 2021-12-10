use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn get_value(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_close(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => 'X',
    }
}
fn main() {
    let res: Vec<_> = include_str!("../input/10.txt")
        .lines()
        .map(|line| {
            match line.chars().fold_while(Vec::new(), |mut stack, c| {
                if matches!(c, '(' | '[' | '{' | '<') {
                    stack.push(c);
                    Continue(stack)
                } else if c == get_close(*stack.last().unwrap()) {
                    stack.pop();
                    Continue(stack)
                } else {
                    Done(vec![c])
                }
            }) {
                // we didn't hit any errors - but there might be open chars we need to count
                Continue(stack) => {
                    Continue(stack.iter().rev().fold(0, |acc, c| 5 * acc + get_value(*c)))
                }
                // we hit an error - single value stack with the error
                Done(stack) => Done(get_value(stack[0])),
            }
        })
        .collect();

    println!(
        "Part 1 {}",
        res.iter()
            .filter_map(|val| if let Done(v) = val { Some(v) } else { None })
            .sum::<usize>()
    );
    let res = res
        .iter()
        .filter_map(|val| {
            if let Continue(v) = val {
                Some(*v)
            } else {
                None
            }
        })
        .sorted()
        .collect_vec();
    println!("Part 2 {}", res[res.len() / 2]);
}
