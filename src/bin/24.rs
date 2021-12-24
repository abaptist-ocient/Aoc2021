// I didn't solve this without looking through reddit - the "key value" part was non-obvious...
#[derive(Debug)]
struct KeyValues {
    a: isize,
    b: isize,
    c: isize,
}

impl KeyValues {
    fn new(a: &str, b: &str, c: &str) -> Self {
        KeyValues {
            a: get_val(a),
            b: get_val(b),
            c: get_val(c),
        }
    }
    // manually parse the first 18 lines to compute this
    fn compute(&self, w: isize, z: isize) -> isize {
        let x = if (z % 26 + self.b) == w { 0 } else { 1 };
        (z / self.a) * (25 * x + 1) + (w + self.c) * x
    }
}

fn get_val(line: &str) -> isize {
    line.split(' ').collect::<Vec<_>>()[2].parse().unwrap()
}

fn compute(keys: &[KeyValues], idx: usize, z: isize, answer: &mut Vec<isize>) -> bool {
    if idx >= 14 {
        // at the end we must be 0
        return z == 0;
    }
    // for part 1, do this in reverse
    for w in 1..=9 {
        let next_z = keys[idx].compute(w, z);
        // don't allow z to get less than zero or too large (> 26^2?)
        if (0..10000).contains(&next_z) && compute(keys, idx + 1, next_z, answer) {
            answer.push(w);
            return true;
        }
    }
    false
}

fn main() {
    let input: Vec<_> = include_str!("../input/24.txt").lines().collect();
    let mut keys = Vec::new();
    for i in 0..14 {
        keys.push(KeyValues::new(
            input[18 * i + 4],
            input[18 * i + 5],
            input[18 * i + 15],
        ));
    }
    let mut answer = Vec::new();
    compute(&keys, 0, 0, &mut answer);
    answer.iter().rev().for_each(|d| print!("{}", d));
}
