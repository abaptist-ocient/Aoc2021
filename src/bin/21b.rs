// actually one less than this
const WIN_SCORE: usize = 21;
// puzzle input - 1
const START_1: usize = 5;
const START_2: usize = 7;
fn main() {
    // value is count
    let mut turn_1 = [[[[0; WIN_SCORE]; WIN_SCORE]; 10]; 10];
    let mut turn_2 = [[[[0; WIN_SCORE]; WIN_SCORE]; 10]; 10];
    // put both players at the start positions
    turn_1[START_1][START_2][0][0] = 1;
    let mut play1_wins = 0;
    let mut play2_wins = 0;
    for _ in 0..10 {
        // src/dest flip each turn and src is cleared
        play1_wins += play(&mut turn_1, &mut turn_2);
        play2_wins += play(&mut turn_2, &mut turn_1);
    }
    println!("play 1 final {}", play1_wins);
    println!("play 2 final {}", play2_wins);
}
// possibilites after 3 rolls
const PERMUTATIONS: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

fn play(
    src: &mut [[[[usize; WIN_SCORE]; WIN_SCORE]; 10]; 10],
    dst: &mut [[[[usize; WIN_SCORE]; WIN_SCORE]; 10]; 10],
) -> usize {
    // note - don't move scores of WIN_SCORE
    let mut wins = 0;
    (0..10).for_each(|src_pos| {
        (0..10).for_each(|dst_pos| {
            (0..WIN_SCORE).for_each(|src_score| {
                (0..WIN_SCORE).for_each(|dst_score| {
                    let num_here = src[src_pos][dst_pos][src_score][dst_score];
                    src[src_pos][dst_pos][src_score][dst_score] = 0;
                    if num_here > 0 {
                        for (pos, combinations) in PERMUTATIONS.into_iter().enumerate() {
                            let landed_pos = (src_pos + pos) % 10;
                            let new_score = src_score + landed_pos + 1;
                            let num_ways = num_here * combinations;
                            if new_score < WIN_SCORE {
                                dst[dst_pos][landed_pos][dst_score][new_score] += num_ways;
                            } else {
                                wins += num_ways;
                            }
                        }
                    }
                });
            });
        });
    });
    wins
}
