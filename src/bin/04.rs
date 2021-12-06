#![feature(drain_filter)]
type Board = Vec<Vec<Option<&'static str>>>;

fn main() {
    let mut lines = include_str!("../input/4.txt").lines();
    let called: Vec<&str> = lines.next().unwrap().split(',').collect();

    let mut all_boards: Vec<Board> = lines
        .into_iter()
        .collect::<Vec<_>>()
        .chunks(6)
        .map(|line_group| {
            (1..6)
                .map(|i| line_group[i].split_whitespace().map(Some).collect())
                .collect()
        })
        .collect();

    let mut winners = Vec::new();
    for call in called {
        all_boards.iter_mut().for_each(|board| {
            board.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|cell| {
                    if let Some(val) = cell {
                        if *val == call {
                            cell.take();
                        }
                    }
                })
            })
        });

        winners.append(
            &mut all_boards
                .drain_filter(|b| is_bingo(b))
                .map(|b| sum_board(b, call))
                .collect(),
        );
    }
    println!("Part 1 {}", winners.first().unwrap());
    println!("Part 2 {}", winners.last().unwrap());
}

fn is_bingo(board: &Board) -> bool {
    for x in 0..5 {
        let mut row_match = true;
        let mut col_match = true;
        for y in 0..5 {
            row_match &= board[x][y].is_none();
            col_match &= board[y][x].is_none();
        }
        if row_match | col_match {
            return true;
        }
    }
    false
}

fn sum_board(winner: Board, call: &str) -> usize {
    winner.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, col| {
            acc + col.unwrap_or("0").parse::<usize>().unwrap()
        })
    }) * call.parse::<usize>().unwrap()
}
