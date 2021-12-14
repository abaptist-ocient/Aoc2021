use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let (mut polymer, rules) = include_str!("../input/14.txt")
        .split_once("\n\n")
        .map(|(polymer, rules)| {
            let rules: HashMap<_, _> = rules
                .lines()
                .map(|line| {
                    line.split_once(" -> ")
                        .map(|(left, right)| {
                            let c = left.chars().collect_vec();
                            ((c[0], c[1]), right.chars().next().unwrap())
                        })
                        .unwrap()
                })
                .collect();
            let polymer: HashMap<_, _> = polymer
                .chars()
                .collect_vec()
                .windows(2)
                .map(|x| ((x[0], x[1]), 1))
                .collect();
            (polymer, rules)
        })
        .unwrap();

    (0..40).into_iter().for_each(|_| {
        let mut new_polymer: HashMap<_, usize> = HashMap::new();
        polymer.iter().for_each(|(key, count)| {
            let c = rules[key];
            *new_polymer.entry((key.0, c)).or_insert(0) += count;
            *new_polymer.entry((c, key.1)).or_insert(0) += count;
        });
        polymer = new_polymer;
    });

    let mut counts = HashMap::new();
    polymer.into_iter().for_each(|((p1, p2), count)| {
        *counts.entry(p1).or_insert(0) += count;
        *counts.entry(p2).or_insert(0) += count;
    });
    // don't double count the ends
    let mut counts: Vec<_> = counts.into_iter().map(|(c, n)| (c, (n + 1) / 2)).collect();
    counts.sort_by_key(|(_, n)| *n);
    println!("{}", counts.last().unwrap().1 - counts.first().unwrap().1);
}
