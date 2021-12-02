fn main() {
    let lines: Vec<_> = include_str!("../input/2.txt")
        .lines()
        .map(|s| s.split_ascii_whitespace())
        .map(|mut s| (s.next().unwrap(), s.next().unwrap().parse::<i64>().unwrap()))
        .collect();
    let mut x = 0;
    let mut y = 0;
    for ele in &lines {
        match ele.0 {
            "forward" => x += ele.1,
            "up" => y -= ele.1,
            "down" => y += ele.1,
            _ => {}
        }
    }
    println!("Part 1 {} {} {}", x, y, x * y);

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
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
