type Board = Vec<Vec<Option<usize>>>;

fn main() {
    let mut lines = include_str!("../input/4.txt").lines();
    let called = lines.next().unwrap();
    let called: Vec<_> = called
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    // strip out the empty line
    lines.next();

    let mut all_boards: Vec<Board> = Vec::new();
    let mut cur_board = Vec::new();

    for line in lines {
        if line.is_empty() {
            all_boards.push(cur_board.clone());
            cur_board.clear();
        } else {
            cur_board.push(
                line.split_whitespace()
                    .map(|x| Some(x.parse::<usize>().unwrap()))
                    .collect(),
            );
        }
    }
    // need to push the last board :(
    all_boards.push(cur_board);

    let mut win_board = None;
    let mut lose_board = None;
    for call in &called {
        if lose_board.is_some() {
            break;
        };
        (0..all_boards.len()).for_each(|b| {
            for r in 0..5 {
                for c in 0..5 {
                    if let Some(val) = all_boards[b][r][c] {
                        if val == *call {
                            all_boards[b][r][c].take();
                        }
                    }
                }
            }
        });
        let len = all_boards.len();
        (0..len).rev().for_each(|b| {
            let mut bingo = false;
            for x in 0..5 {
                let (mut row_match, mut col_match) = (true, true);
                for y in 0..5 {
                    if all_boards[b][x][y].is_some() {
                        row_match = false;
                    }
                    if all_boards[b][y][x].is_some() {
                        col_match = false;
                    }
                }
                if row_match | col_match {
                    bingo = true;
                    break;
                }
            }
            if bingo {
                let winner = all_boards.remove(b);
                if win_board.is_none() {
                    win_board.replace((winner, call));
                } else if all_boards.is_empty() {
                    lose_board.replace((winner, call));
                }
            }
        });
    }
    let (board, board_call) = win_board.unwrap();
    println!("Part 1 {}", sum_board(board) * board_call);

    let (board, board_call) = lose_board.unwrap();
    println!("Part 2 {}", sum_board(board) * board_call);
}

fn sum_board(winner: Vec<Vec<Option<usize>>>) -> usize {
    winner.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, col| acc + col.unwrap_or(0))
    })
}
