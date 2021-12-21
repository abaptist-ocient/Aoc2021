// board pos  = start - 1
const START_1: usize = 5;
const START_2: usize = 7;

fn main() {
    let mut player_1_pos = START_1;
    let mut player_2_pos = START_2;
    let mut player_1_score = 0;
    let mut player_2_score = 0;
    let mut next = true; // true means player 1
    let mut dice = 0;
    let mut turn = 0;
    while player_1_score < 1000 && player_2_score < 1000 {
        turn += 1;
        for _ in 0..3 {
            dice += 1;
            if next {
                player_1_pos = (player_1_pos + dice) % 10;
            } else {
                player_2_pos = (player_2_pos + dice) % 10;
            }
        }
        if next {
            player_1_score += player_1_pos + 1;
        } else {
            player_2_score += player_2_pos + 1;
        }
        next = !next;
    }
    println!("{}", player_1_score * turn * 3);
    println!("{}", player_2_score * turn * 3);
}
