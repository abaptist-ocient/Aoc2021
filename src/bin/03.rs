use std::collections::HashMap;

fn main() {
    let lines: Vec<&str> = include_str!("../input/3.txt").lines().collect();

    let output = create_hist(&lines);
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut counter: usize = 0;
    while let Some(&v) = output.get(&counter) {
        counter += 1;
        gamma = (gamma << 1) + v;
        epsilon = (epsilon << 1) + if v == 0 { 1 } else { 0 };
    }
    println!("Part 1: {}", gamma * epsilon);

    let mut o2: Vec<&str> = lines.clone();
    let mut co2: Vec<&str> = lines.clone();
    let mut check1: String = "".to_string();
    let mut check2: String = "".to_string();
    let mut o2_final = 0;
    let mut co2_final = 0;
    for i in 0..counter {
        let r1 = create_hist(&o2);
        let r2 = create_hist(&co2);
        check1.push(r1.get(&i).map_or('1', |&v| if v == 0 { '0' } else { '1' }));
        check2.push(r2.get(&i).map_or('0', |&v| if v == 0 { '1' } else { '0' }));
        o2.retain(|x| x.starts_with(check1.as_str()));
        co2.retain(|x| x.starts_with(check2.as_str()));

        if o2.len() == 1 {
            o2_final = isize::from_str_radix(o2[0], 2).unwrap();
        }
        if co2.len() == 1 {
            co2_final = isize::from_str_radix(co2[0], 2).unwrap();
        }
    }
    println!("Part 2 {}", o2_final * co2_final);
}

fn create_hist(lines: &[&str]) -> HashMap<usize, usize> {
    let mut output: HashMap<usize, usize> = HashMap::new();
    for l in lines {
        for (i, c) in l.chars().enumerate() {
            let old = *output.get(&i).unwrap_or(&0);
            output.insert(i, old + if c == '1' { 1 } else { 0 });
        }
    }
    output
        .iter()
        .map(|(k, v)| (*k, (v * 2) / lines.len()))
        .collect()
}
