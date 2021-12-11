fn main() {
    let lines: Vec<_> = include_str!("../input/11.txt").lines().collect();
    // make board 1 cell larger on all sides
    let mut board = vec![vec![0; lines.len() + 2]; lines[0].len() + 2];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            board[i + 1][j + 1] = c.to_digit(10).unwrap();
        }
    }

    let mut count_flash = 0;
    let mut iter = 0;
    loop {
        (1..&board.len() - 1).for_each(|i| {
            (1..board[i].len() - 1).for_each(|j| {
                board[i][j] += 1;
            });
        });
        while (1..&board.len() - 1).fold(false, |acc, i| {
            acc | (1..board[i].len() - 1).fold(false, |acc, j| {
                acc | if board[i][j] > 9 {
                    (0..=2).for_each(|x| {
                        (0..=2).for_each(|y| {
                            let pos = &mut board[i + x - 1][j + y - 1];
                            if *pos > 0 {
                                *pos += 1;
                            }
                        });
                    });
                    count_flash += 1;
                    board[i][j] = 0;
                    true
                } else {
                    false
                }
            })
        }) {}
        iter += 1;
        if iter == 100 {
            println!("part 1 {}", count_flash);
        }
        if (0..&board.len() - 1).fold(true, |acc, i| {
            acc & (0..&board[i].len() - 1).fold(true, |acc, j| acc & (board[i][j] == 0))
        }) {
            println!("part 2 {}", iter);
            break;
        }
    }
}
