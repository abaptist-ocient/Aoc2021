fn main() {
    let (x, y, aim) = include_str!("../input/2.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .fold((0, 0, 0), |(x, y, aim), (cmd, amount)| {
            match (cmd, amount.parse::<i32>().unwrap()) {
                ("forward", value) => (x + value, y + value * aim, aim),
                ("down", value) => (x, y, aim + value),
                ("up", value) => (x, y, aim - value),
                _ => unreachable!(),
            }
        });
    println!("Part 1 {}", x * aim);
    println!("Part 2 {}", x * y);
}
