fn main() {
    let lines: Vec<_> = include_str!("../input/2.txt")
        .lines()
        .filter_map(|s| s.split_once(" "))
        .map(|(a, b)| (a, b.parse::<i64>().unwrap()))
        .collect();
    let (mut x, mut y) = (0, 0);

    for ele in &lines {
        match ele.0 {
            "forward" => x += ele.1,
            "up" => y -= ele.1,
            "down" => y += ele.1,
            _ => {}
        }
    }
    println!("Part 1 {} {} {}", x, y, x * y);

    let (mut x, mut y, mut aim) = (0, 0, 0);
    for ele in &lines {
        match ele.0 {
            "forward" => {
                x += ele.1;
                y += ele.1 * aim
            }
            "up" => aim -= ele.1,
            "down" => aim += ele.1,
            _ => {}
        }
    }
    println!("Part 2 {} {} {}", x, y, x * y);
}
