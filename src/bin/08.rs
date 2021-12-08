use std::collections::{HashMap, HashSet};

fn main() {
    let lines = include_str!("../input/8.txt").lines();
    // part 1
    let sum = lines
        .clone()
        .map(|line| {
            line.split_once('|')
                .map(|(_, values)| {
                    values
                        .split_whitespace()
                        .map(|code| code.len())
                        .filter(|&l| (l == 2) || (l == 3) || (l == 4) | (l == 7))
                        .count()
                })
                .unwrap()
        })
        .sum::<usize>();
    println!("part 1 {}", sum);

    // part 2
    let sum = lines
        .clone()
        .map(|line| {
            line.split_once('|')
                .map(|(keys, values)| {
                    let mut m = HashMap::new();
                    keys.split_whitespace().for_each(|code| {
                        m.entry(code.len())
                            .or_insert_with(Vec::<HashSet<char>>::new)
                            .push(code.chars().collect())
                    });
                    // really ugly...
                    let a = &m[&3][0] - &m[&2][0];
                    let g = m[&6]
                        .iter()
                        .map(|x| &(x - &a) - &m[&4][0])
                        .find(|s| s.len() == 1)
                        .unwrap();
                    let d = m[&5]
                        .iter()
                        .map(|x| &(&(x - &a) - &g) - &m[&2][0])
                        .find(|s| s.len() == 1)
                        .unwrap();
                    let b = &(&m[&4][0] - &m[&2][0]) - &d;
                    let c = m[&6]
                        .iter()
                        .map(|x| &m[&3][0] - x)
                        .find(|s| s.len() == 1)
                        .unwrap();
                    let f = &m[&2][0] - &c;
                    let e = &(&(&(&(&(&m[&7][0] - &a) - &b) - &c) - &d) - &f) - &g;
                    let a = *a.iter().next().unwrap();
                    let b = *b.iter().next().unwrap();
                    let c = *c.iter().next().unwrap();
                    let d = *d.iter().next().unwrap();
                    let e = *e.iter().next().unwrap();
                    let f = *f.iter().next().unwrap();
                    let g = *g.iter().next().unwrap();

                    let mut digits = vec![HashSet::new(); 10];

                    digits[0].extend([a, b, c, e, f, g]);
                    digits[1].extend([c, f]);
                    digits[2].extend([a, c, d, e, g]);
                    digits[3].extend([a, c, d, f, g]);
                    digits[4].extend([b, d, c, f]);
                    digits[5].extend([a, b, d, f, g]);
                    digits[6].extend([a, b, d, e, f, g]);
                    digits[7].extend([a, c, f]);
                    digits[8].extend([a, b, c, d, e, f, g]);
                    digits[9].extend([a, b, c, d, f, g]);

                    let result = values.split_whitespace().fold(0, |val, digit| {
                        let digit: HashSet<char> = digit.chars().collect();
                        for (i, m) in digits.iter().enumerate() {
                            if m == &digit {
                                return i + val * 10;
                            };
                        }
                        9999999
                    });
                    result
                })
                .unwrap()
        })
        .sum::<usize>();
    println!("part 2 {}", sum);
}

/*
println!("all: {:?}", m);
println!(" {} ", a.iter().next().unwrap());
println!("{} {}", b.iter().next().unwrap(), c.iter().next().unwrap());
println!(" {} ", d.iter().next().unwrap());
println!("{} {}", e.iter().next().unwrap(), f.iter().next().unwrap());
println!(" {} ", g.iter().next().unwrap());
*/
