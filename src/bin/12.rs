use std::collections::{HashMap, HashSet};

fn main() {
    let mut g: HashMap<&str, HashSet<&str>> = HashMap::new();
    include_str!("../input/12.txt").lines().for_each(|line| {
        if let Some((a, b)) = line.split_once('-') {
            g.entry(a).or_insert_with(HashSet::new).insert(b);
            g.entry(b).or_insert_with(HashSet::new).insert(a);
        }
    });

    println!("{}", go(&g, &mut HashMap::from([("start", 1)]), "start", 1));
    println!("{}", go(&g, &mut HashMap::from([("start", 2)]), "start", 2));

    fn go<'a>(
        graph: &HashMap<&'a str, HashSet<&'a str>>,
        path: &mut HashMap<&'a str, usize>,
        start: &'a str,
        max: usize,
    ) -> usize {
        // this check is only valid if we allow running through small caves more than once
        if max > 1 && path.values().filter(|&v| *v == max).count() > 2 {
            return 0;
        }
        graph[start]
            .iter()
            .map(|&child| follow(child, path, graph, max))
            .sum()
    }

    fn follow<'a>(
        child: &'a str,
        path: &mut HashMap<&'a str, usize>,
        graph: &HashMap<&'a str, HashSet<&'a str>>,
        max: usize,
    ) -> usize {
        // at tne end - counts as 1 path
        if child == "end" {
            return 1;
        }
        // we have gone through small caves too many times - can't go down this child
        if path.get(child) == Some(&max) {
            return 0;
        }
        // set this before iteration and unset aftewards - is there a better pattern?
        if child.to_uppercase() != child {
            *path.entry(child).or_insert(0) += 1;
        }
        let val = go(graph, path, child, max);
        if child.to_uppercase() != child {
            *path.get_mut(child).unwrap() -= 1;
        }
        val
    }
}
