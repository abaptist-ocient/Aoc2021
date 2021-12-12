fn main() {
    let lines: Vec<_> = include_str!("../input/11.txt").lines().collect();
    let mut board = vec![vec![0; lines.len() + 2]; lines[0].len() + 2];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            board[i + 1][j + 1] = c.to_digit(10).unwrap();
        }
    }

    let mut count_flash = 0;
    let part2 = (1..).find(|&iter| {
        (1..&board.len() - 1).for_each(|i| {
            (1..board[i].len() - 1).for_each(|j| {
                board[i][j] += 1;
            });
        });
        let mut delta = usize::MAX;
        while delta > 0 {
            delta = (1..&board.len() - 1).fold(0, |acc, i| {
                acc + (1..board[i].len() - 1).fold(0, |acc, j| acc + flash(&mut board, i, j))
            });
            count_flash += delta;
        }
        if iter == 100 {
            println!("part 1 {}", count_flash);
        }
        !board.iter().any(|row| row.iter().any(|cell| *cell != 0))
    });
    println!("part 2 {}", part2.unwrap());

    fn flash(board: &mut Vec<Vec<u32>>, i: usize, j: usize) -> usize {
        if board[i][j] > 9 {
            (0..=2).for_each(|x| {
                (0..=2).for_each(|y| {
                    let pos = &mut board[i + x - 1][j + y - 1];
                    if *pos > 0 {
                        *pos += 1;
                    }
                });
            });
            board[i][j] = 0;
            1
        } else {
            0
        }
    }
}
