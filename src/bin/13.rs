use itertools::Itertools;

const ROWS: usize = 890;
const COLS: usize = 1311;

fn main() {
    let (mut board, folds) = include_str!("../input/13.txt")
        .split_once("\n\n")
        .map(|(points, folds)| {
            let mut board = vec![vec![false; COLS]; ROWS];
            points.lines().for_each(|p| {
                p.split_once(',')
                    .map(|(x, y)| {
                        board[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = true
                    })
                    .unwrap()
            });
            let folds = folds
                .lines()
                .map(|f| {
                    f.split_once('=')
                        .map(|(dir, amount)| (dir.ends_with('x'), amount.parse::<usize>().unwrap()))
                        .unwrap()
                })
                .collect_vec();
            (board, folds)
        })
        .unwrap();

    for f in folds {
        board = fold(board, f);
        println!(
            "{}",
            board.iter().fold(0, |x, r| {
                x + r.iter().fold(0, |x, &c| x + if c { 1 } else { 0 })
            })
        );
    }
    board.iter().for_each(|r| {
        println!(
            "{}",
            r.iter()
                .map(|&c| if c { '#' } else { ' ' })
                .collect::<String>()
        )
    });
}

fn fold(mut board: Vec<Vec<bool>>, f: (bool, usize)) -> Vec<Vec<bool>> {
    if f.0 {
        let overlap = board[0].len() - f.1 - 1;
        board
            .iter_mut()
            .map(|row| {
                let (left, right) = row.split_at_mut(f.1);
                for i in 0..overlap {
                    left[left.len() - i - 1] |= right[i + 1];
                }
                left.to_vec()
            })
            .collect_vec()
    } else {
        let overlap = board.len() - f.1 - 1;
        let (top, bot) = board.split_at_mut(f.1);
        for i in 0..overlap {
            for j in 0..top[i].len() {
                top[top.len() - i - 1][j] |= bot[i + 1][j];
            }
        }
        top.to_vec()
    }
}
