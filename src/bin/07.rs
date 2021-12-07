fn main() {
    let input: Vec<isize> = include_str!("../input/7.txt")
        .split(',')
        .filter_map(|x| x.trim().parse::<isize>().ok())
        .collect();
    let output = (*input.iter().min().unwrap()..=*input.iter().max().unwrap())
        .map(|x| compute_distance(x, &input));
    println!("{:?}", output.min().unwrap());
}
fn compute_distance(point: isize, input: &[isize]) -> isize {
    input
        .iter()
        .map(|x| (point - x).abs())
        .map(|x| x * (x + 1) / 2)
        .sum()
}
