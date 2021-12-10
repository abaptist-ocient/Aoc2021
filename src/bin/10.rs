use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let end: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let p1: HashMap<char, usize> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let p2: HashMap<char, usize> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let res: Vec<_> = include_str!("../input/10.txt")
        .lines()
        .map(|line| {
            line.chars().try_fold(Vec::new(), |mut stack, c| {
                if end.contains_key(&c) {
                    stack.push(c);
                    Ok(stack)
                } else if c == end[stack.last().unwrap()] {
                    stack.pop();
                    Ok(stack)
                } else {
                    // part 1 we hit an error - single value stack with the error value
                    Err(p1[&c])
                }
            })
        })
        .map(| stack|
        // part 2 we didn't hit any errors - detemine the value of the remaining stack
            stack.map(|stack| stack.iter().rev().fold(0, |acc, c| 5 * acc + p2[c])))
        .collect();

    let errors = res.iter().filter_map(|val| val.err());
    println!("Part 1 {}", errors.sum::<usize>());
    let success = res.iter().filter_map(|val| val.ok()).sorted().collect_vec();
    println!("Part 2 {}", success[success.len() / 2]);
}
