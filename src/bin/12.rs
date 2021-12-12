use std::collections::{HashMap, HashSet};

fn main() {
    let mut graph: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();
    include_str!("../input/12.txt").lines().for_each(|line| {
        if let Some((a, b)) = line.split_once('-') {
            graph.entry(a).or_insert_with(HashSet::new).insert(b);
            graph.entry(b).or_insert_with(HashSet::new).insert(a);
        }
    });

    println!(
        "{:?}",
        follow_path(&graph, &mut HashMap::from([("start", 0)]), "start")
    );

    fn follow_path(
        graph: &HashMap<&'static str, HashSet<&'static str>>,
        path: &mut HashMap<&'static str, usize>,
        start: &'static str,
    ) -> usize {
        graph[start]
            .iter()
            .map(|&child| follow_child(child, path, graph))
            .sum()
    }

    fn follow_child(
        child: &'static str,
        path: &mut HashMap<&'static str, usize>,
        graph: &HashMap<&'static str, HashSet<&'static str>>,
    ) -> usize {
        // at end - found a path
        if child == "end" {
            return 1;
        }
        // if we are at 0 for a path, don't proceed
        if path.get(child) == Some(&0) {
            return 0;
        }

        if child.to_uppercase() != child {
            *path.entry(child).or_insert(2) -= 1;
        }
        let val = if path.values().filter(|&v| *v == 0).count() > 2 {
            // we visited two small caves more than once
            0
        } else {
            follow_path(graph, path, child)
        };
        if child.to_uppercase() != child {
            *path.get_mut(child).unwrap() += 1;
        }
        val
    }
}
