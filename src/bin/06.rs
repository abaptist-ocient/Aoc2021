fn main() {
    let mut lines = include_str!("../input/6.txt").lines();
    let mut fish = vec![0; 9];
    lines
        .next()
        .unwrap()
        .split(',')
        .for_each(|x| fish[x.parse::<usize>().unwrap()] += 1);

    for _ in 0..256 {
        let mut new_fish = vec![0; 9];
        new_fish[..8].clone_from_slice(&fish[1..9]);
        new_fish[8] = fish[0];
        new_fish[6] += fish[0];
        fish = new_fish;
    }
    println!("{:?}", fish.iter().sum::<usize>());
}
