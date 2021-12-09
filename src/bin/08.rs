use std::collections::{HashMap, HashSet};

fn main() {
    let lines = include_str!("../input/8.txt").lines();
    println!("part 1 {}", run_fn(&lines, process_row1));
    println!("part 2 {}", run_fn(&lines, process_row2));
}

fn run_fn<F>(lines: &std::str::Lines, fun: F) -> usize
where
    F: Fn(&str, &str) -> usize,
{
    lines
        .clone()
        .map(|line| {
            line.split_once('|')
                .map(|(keys, values)| fun(keys, values))
                .unwrap()
        })
        .sum::<usize>()
}

fn process_row1(_: &str, values: &str) -> usize {
    values
        .split_whitespace()
        .map(|code| code.len())
        .filter(|&l| (l == 2) || (l == 3) || (l == 4) | (l == 7))
        .count()
}

const DATA_MAP: &[&str] = &[
    "abcefg", "cf", "acdeg", "acdfg", "bdcf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn process_row2(keys: &str, values: &str) -> usize {
    let mut m = HashMap::new();
    keys.split_whitespace().for_each(|code| {
        m.entry(code.len())
            .or_insert_with(Vec::<HashSet<char>>::new)
            .push(code.chars().collect())
    });

    let mut t = HashMap::new();
    t.insert(&'a', &m[&3][0] - &m[&2][0]);
    t.insert(
        &'g',
        m[&6]
            .iter()
            .map(|x| &(x - &t[&'a']) - &m[&4][0])
            .find(|s| s.len() == 1)
            .unwrap(),
    );
    t.insert(
        &'d',
        m[&5]
            .iter()
            .map(|x| &(&(x - &t[&'a']) - &t[&'g']) - &m[&2][0])
            .find(|s| s.len() == 1)
            .unwrap(),
    );
    t.insert(&'b', &(&m[&4][0] - &m[&2][0]) - &t[&'d']);
    t.insert(
        &'c',
        m[&6]
            .iter()
            .map(|x| &m[&3][0] - x)
            .find(|s| s.len() == 1)
            .unwrap(),
    );
    t.insert(&'f', &m[&2][0] - &t[&'c']);
    t.insert(
        &'e',
        &(&(&(&(&(&m[&7][0] - &t[&'a']) - &t[&'b']) - &t[&'c']) - &t[&'d']) - &t[&'f']) - &t[&'g'],
    );

    let t: HashMap<char, char> = t
        .iter()
        .map(|(k, v)| (**k, *v.iter().next().unwrap()))
        .collect();

    let mut digits = vec![HashSet::new(); 10];
    for i in 0..digits.len() {
        digits[i].extend(
            DATA_MAP[i]
                .chars()
                .map(|d| &t[&d])
                .collect::<HashSet<&char>>(),
        );
    }

    values.split_whitespace().fold(0, |val, digit| {
        let digit: HashSet<char> = digit.chars().collect();
        for (i, m) in digits.iter().enumerate() {
            if m == &digit {
                return i + val * 10;
            };
        }
        9999999
    })
}
