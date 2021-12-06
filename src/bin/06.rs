fn main() {
    let mut fish = vec![0; 9];

    include_str!("../input/6.txt")
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .for_each(|x| fish[x] += 1);

    for _ in 0..256 {
        let mut new_fish = vec![0; 9];
        new_fish[..8].clone_from_slice(&fish[1..9]);
        new_fish[8] = fish[0];
        new_fish[6] += fish[0];
        fish = new_fish;
    }
    println!("{:?}", fish.iter().sum::<usize>());
}
