use Point::*;

#[derive(Clone, Copy, Debug)]
enum Point {
    Wall,
    NotBasin(u32),
    Basin(u32, usize), // val, basin_id
}

fn get_value(p: &Point) -> u32 {
    match p {
        Wall => 9,
        NotBasin(x) => *x,
        Basin(x, _) => *x,
    }
}

fn update_point(point: &mut Point, basin_id: usize) -> bool {
    if let NotBasin(val) = *point {
        *point = Basin(val, basin_id);
        return true;
    }
    false
}

fn main() {
    let lines: Vec<_> = include_str!("../input/9.txt").lines().collect();
    // make board 1 cell larger on all sides
    let mut board = vec![vec![Wall; lines.len() + 2]; lines[0].len() + 2];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let d = c.to_digit(10).unwrap();
            board[i][j] = if let 0..=8 = d { NotBasin(d) } else { Wall }
        }
    }

    // new basin for each min
    let mut basin_id = 0;
    let mut sum = 0;
    (1..&board.len() - 1).for_each(|i| {
        (1..board[i].len() - 1).for_each(|j| {
            let val = get_value(&board[i][j]);
            if val < get_value(&board[i - 1][j])
                && val < get_value(&board[i][j - 1])
                && val < get_value(&board[i + 1][j])
                && val < get_value(&board[i][j + 1])
            {
                sum += get_value(&board[i][j]) + 1;
                update_point(&mut board[i][j], basin_id);
                basin_id += 1;
            }
        });
    });
    println!("Part 1 {}", sum);

    // grow basins as much as possible
    let mut has_change = true;
    while has_change {
        has_change = false;
        (1..&board.len() - 1).for_each(|i| {
            (1..board[i].len() - 1).for_each(|j| {
                if let Basin(_, basin_id) = board[i][j] {
                    has_change |= update_point(&mut board[i][j - 1], basin_id);
                    has_change |= update_point(&mut board[i][j + 1], basin_id);
                    has_change |= update_point(&mut board[i - 1][j], basin_id);
                    has_change |= update_point(&mut board[i + 1][j], basin_id);
                }
            });
        });
    }

    // count basin sizes
    let mut counts = vec![0; basin_id];
    board.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            if let Basin(_, basin_id) = cell {
                counts[*basin_id] += 1;
            }
        })
    });
    counts.sort_unstable();
    counts.reverse();
    println!("Part 2 {}", counts[..3].iter().product::<usize>());
}
