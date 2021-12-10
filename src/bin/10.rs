use std::collections::HashMap;
fn main() {
    let p1: HashMap<char, usize> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let p2: HashMap<char, usize> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let end: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let res: Vec<_> = include_str!("../input/10.txt")
        .lines()
        .map(|line| {
            match line.chars().try_fold(Vec::new(), |mut stack, c| {
                if matches!(c, '(' | '[' | '{' | '<') {
                    stack.push(c);
                    Ok(stack)
                } else if c == end[stack.last().unwrap()] {
                    stack.pop();
                    Ok(stack)
                } else {
                    Err(vec![c])
                }
            }) {
                // part 2 we didn't hit any errors - but there might be open chars we need to count
                Ok(stack) => Ok(stack.iter().rev().fold(0, |acc, c| 5 * acc + p2[c])),
                // part 1 we hit an error - single value stack with the error
                Err(stack) => Err(p1[&stack[0]]),
            }
        })
        .collect();

    println!(
        "Part 1 {}",
        res.iter()
            .filter_map(|val| if let Err(v) = val { Some(v) } else { None })
            .sum::<usize>()
    );

    // only keep the success
    let mut res: Vec<_> = res.iter().filter_map(|val| val.ok()).collect();
    res.sort_unstable();
    println!("Part 2 {}", res[res.len() / 2]);
}
