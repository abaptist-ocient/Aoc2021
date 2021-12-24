use std::{
    collections::{BinaryHeap, HashMap},
    fmt::{Display, Formatter},
    hash::Hash,
};

#[derive(Debug, Eq, Clone)]
struct Board {
    p: Vec<u8>,
    score: usize,
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Hash for Board {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.p.hash(state);
    }
}

impl Ord for Board {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            r#"*************
*{}{}{}{}{}{}{}{}{}{}{}*
***{}*{}*{}*{}***
  *{}*{}*{}*{}*
  *{}*{}*{}*{}*
  *{}*{}*{}*{}*
  *********
Cost: {}"#,
            c(self.p[0]),
            c(self.p[1]),
            c(self.p[2]),
            c(self.p[3]),
            c(self.p[4]),
            c(self.p[5]),
            c(self.p[6]),
            c(self.p[7]),
            c(self.p[8]),
            c(self.p[9]),
            c(self.p[10]),
            c(self.p[11]),
            c(self.p[12]),
            c(self.p[13]),
            c(self.p[14]),
            c(self.p[15]),
            c(self.p[16]),
            c(self.p[17]),
            c(self.p[18]),
            c(self.p[19]),
            c(self.p[20]),
            c(self.p[21]),
            c(self.p[22]),
            c(self.p[23]),
            c(self.p[24]),
            c(self.p[25]),
            c(self.p[26]),
            self.score,
        )
    }
}
fn c(val: u8) -> char {
    match val {
        1 => 'A',
        2 => 'B',
        3 => 'C',
        4 => 'D',
        _ => ' ',
    }
}

// *****************
// *01-23-45-67-8910*
//   *11*12*13*14*
//   *15*16*17*18*
//   *19*20*21*22*
//   *23*24*25*26*
//   *************

impl Board {
    fn get_moves(&self) -> Vec<Self> {
        self.p
            .iter()
            .enumerate()
            .filter(|(_, &val)| val > 0)
            .flat_map(|(pos, _)| self.move_from(pos))
            .collect()
    }
    fn is_solved(&self) -> bool {
        for x in 0..=10 {
            if self.p[x] != 0 {
                return false;
            }
        }
        for (num, &col) in DROPS.iter().enumerate() {
            for &row in &col[1..] {
                if self.p[row] != num as u8 + 1 {
                    return false;
                }
            }
        }
        true
    }

    fn move_from(&self, pos: usize) -> Vec<Board> {
        if pos < 11 {
            self.move_horizontal(pos, true)
        } else if let Some((new_pos, cost)) = self.try_rise(pos) {
            let mut p = self.p.clone();
            p[new_pos] = p[pos];
            p[pos] = 0;
            let updated_board = Board {
                p,
                score: self.score + cost,
            };
            updated_board.move_horizontal(new_pos, false)
        } else {
            Vec::new()
        }
    }

    // all boards from this postion that are valid
    fn move_horizontal(&self, pos: usize, must_drop: bool) -> Vec<Board> {
        let mut moves = Vec::new();
        // move right
        self.move_right(pos, must_drop, &mut moves);
        if pos > 0 {
            self.move_left(pos, must_drop, &mut moves);
        }
        moves
    }

    fn move_left(&self, pos: usize, must_drop: bool, moves: &mut Vec<Board>) {
        let val = self.p[pos];

        for x in (0..pos).rev() {
            if self.p[x] == 0 {
                let mut p = self.p.clone();
                p[x] = p[pos];
                p[pos] = 0;

                if let Some(drop_slots) = check_drop(&x, p[x]) {
                    let y_drop = match self.try_drop(drop_slots, &mut p, val, x) {
                        Some(value) => value,
                        None => continue,
                    };
                    if y_drop > 0 || !must_drop {
                        let score = self.score + ((pos - x) + y_drop) * get_cost(val);
                        moves.push(Board { p, score });
                    }
                }
            } else {
                break;
            }
        }
    }

    fn move_right(&self, pos: usize, must_drop: bool, moves: &mut Vec<Board>) {
        let val = self.p[pos];

        for x in pos + 1..=10 {
            if self.p[x] == 0 {
                let mut p = self.p.clone();
                p[x] = val;
                p[pos] = 0;

                if let Some(drop_slots) = check_drop(&x, p[x]) {
                    let y_drop = match self.try_drop(drop_slots, &mut p, val, x) {
                        Some(value) => value,
                        None => continue,
                    };
                    if y_drop > 0 || !must_drop {
                        let score = self.score + ((x - pos) + y_drop) * get_cost(val);
                        moves.push(Board { p, score });
                    }
                }
            } else {
                break;
            }
        }
    }

    // rise up if possible from this pos - return "temp pos" and cost to get up
    fn try_rise(&self, pos: usize) -> Option<(usize, usize)> {
        let val = self.p[pos];
        for (col_match, col) in DROPS.iter().enumerate() {
            let mut empty_col = true;
            for (depth, &y) in col.iter().enumerate() {
                // verify that this col matches and is empty above us
                if y == pos && empty_col && self.p[y] != 0 {
                    // also verify that there is at least one "non-match" below us
                    let mut hidden = false;
                    for (depth2, &y2) in col.iter().enumerate().rev() {
                        hidden |= self.p[y2] != col_match as u8 + 1;
                        if depth <= depth2 && hidden {
                            return Some((col[0], depth * get_cost(val)));
                        }
                    }
                    // can't move we are already home
                } else if self.p[y] != 0 {
                    empty_col = false;
                }
            }
        }
        None
    }

    fn try_drop(&self, drop_slots: &[usize], p: &mut Vec<u8>, val: u8, x: usize) -> Option<usize> {
        let mut y_drop = 0;
        if !drop_slots.is_empty() {
            for &y in drop_slots {
                if self.p[y] == 0 {
                    y_drop += 1;
                } else if self.p[y] != val {
                    return None;
                }
            }
            if y_drop == 0 {
                return None;
            } else {
                p[drop_slots[y_drop - 1]] = val;
                p[x] = 0;
            }
        }
        Some(y_drop)
    }
}

const DROP_POS: [bool; 11] = [
    false, false, true, false, true, false, true, false, true, false, false,
];
const DROPS: [[usize; 5]; 4] = [
    [2, 11, 15, 19, 23],
    [4, 12, 16, 20, 24],
    [6, 13, 17, 21, 25],
    [8, 14, 18, 22, 26],
];
// num of places it can drop
// None - illegal
// 0 - stay there
// n - drop n spaces
fn check_drop(pos: &usize, val: u8) -> Option<&'static [usize]> {
    if DROP_POS[*pos] {
        let drop_list = &DROPS[val as usize - 1];
        if *pos != drop_list[0] {
            None
        } else {
            Some(&drop_list[1..])
        }
    } else {
        Some(&[])
    }
}

fn get_cost(val: u8) -> usize {
    10_usize.pow(val as u32 - 1)
}

fn main() {
    let mut start = vec![0u8; 11];
    let input: Vec<_> = include_str!("../input/23.txt")
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| 1 + c - b'A')
        .collect();
    start.extend_from_slice(&input[0..4]);
    start.extend_from_slice(&[4, 3, 2, 1, 4, 2, 1, 3]);
    start.extend_from_slice(&input[4..]);
    // for part one uncomment this
    // start.extend_from_slice(&[1, 2, 3, 4, 1, 2, 3, 4]);

    let b = Board { p: start, score: 0 };
    println!("{}", b);

    let mut heap = BinaryHeap::new();
    let mut checked: HashMap<Vec<u8>, usize> = HashMap::new();
    heap.push(b);
    let mut first = true;
    while !heap.is_empty() {
        if let Some(attempt) = heap.pop() {
            if !first && attempt.is_solved() {
                println!("Solved \n{}", attempt);
                return;
            }
            first = false;
            for b in attempt.get_moves() {
                if let Some(&exising) = checked.get(&b.p) {
                    if b.score < exising {
                        checked.insert(b.p.clone(), b.score);
                        heap.push(b);
                    }
                } else {
                    checked.insert(b.p.clone(), b.score);
                    heap.push(b);
                }
            }
        }
    }
    println!("No solution");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move() {
        let mut p = vec![0u8; 27];

        p[1] = 2;
        p[11] = 1;
        p[15] = 1;
        p[19] = 1;
        p[23] = 1;

        p[12] = 0;
        p[16] = 2;
        p[20] = 1;
        p[24] = 2;

        p[13] = 3;
        p[17] = 3;
        p[21] = 3;
        p[25] = 3;

        p[14] = 4;
        p[18] = 4;
        p[22] = 4;
        p[26] = 4;

        let b = Board { p, score: 0 };
        println!("{}", b);
        println!("{}", b.is_solved());
        let moves = b.get_moves();
        for m in moves {
            println!("{}", m);
        }
    }
}
