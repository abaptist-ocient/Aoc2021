fn main() {
    let lines: Vec<_> = include_str!("../input/1.txt")
        .lines()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    let count = lines.windows(2).filter(|&x| x[1] > x[0]).count();
    println!("Part 1 {}", count);
    let count = lines.windows(4).filter(|&x| x[3] > x[0]).count();
    println!("Part 2 {}", count);
}
