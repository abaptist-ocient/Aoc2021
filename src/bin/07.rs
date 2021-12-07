fn main() {
    let mut input: Vec<isize> = include_str!("../input/7.txt")
        .split(',')
        .filter_map(|x| x.parse::<isize>().ok())
        .collect();
    input.sort_unstable();
    let output =
        (*input.first().unwrap()..=*input.last().unwrap()).map(|x| compute_distance(x, &input));
    println!("{:?}", output.min().unwrap());
}
fn compute_distance(point: isize, input: &[isize]) -> isize {
    input
        .iter()
        .map(|x| (point - x).abs())
        .map(|x| x * (x + 1) / 2)
        .sum()
}
