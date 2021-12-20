#![feature(destructuring_assignment)]
use std::collections::HashSet;

use itertools::Itertools;

struct Board {
    board: HashSet<(isize, isize)>,
    edge: bool,
    x_bounds: (isize, isize),
    y_bounds: (isize, isize),
}

impl Board {
    fn new(board: HashSet<(isize, isize)>, edge: bool) -> Self {
        let mut me = Board {
            board,
            edge,
            x_bounds: (0, 0),
            y_bounds: (0, 0),
        };
        me.update_bounds();
        me
    }
    fn set_board(&mut self, new_board: HashSet<(isize, isize)>) {
        self.board = new_board;
        self.update_bounds();
    }

    fn update_bounds(&mut self) {
        (self.x_bounds, self.y_bounds) = self.board.iter().fold(
            ((isize::MAX, isize::MIN), (isize::MAX, isize::MIN)),
            |init, pos| {
                (
                    (init.0 .0.min(pos.0), init.0 .1.max(pos.0)),
                    (init.1 .0.min(pos.1), init.1 .1.max(pos.1)),
                )
            },
        );
    }

    // fn print_board(&self, iter: usize) {
    //     (self.y_bounds.0 - 2..=self.y_bounds.1 + 2).for_each(|y| {
    //         (self.x_bounds.0 - 2..=self.x_bounds.1 + 2).for_each(|x| {
    //             print!("{}", if self.get_value(x, y, iter) { "#" } else { "." });
    //         });
    //         println!();
    //     })
    // }

    fn get_value(&self, x: isize, y: isize, iter: usize) -> bool {
        if x < self.x_bounds.0 || x > self.x_bounds.1 || y < self.y_bounds.0 || y > self.y_bounds.1
        {
            // handle the case where the edge "flaps" if the 0 cell is on
            iter % 2 == 1 && self.edge
        } else {
            self.board.contains(&(x, y))
        }
    }
}
fn main() {
    let mut input = include_str!("../input/20.txt").lines();
    let mapping: Vec<_> = input.next().unwrap().chars().map(|c| c == '#').collect();

    input.next().unwrap();
    let board: HashSet<(isize, isize)> = input
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();
    let mut board = Board::new(board, mapping[0]);

    (0..50).for_each(|iter| {
        board.set_board(
            (board.x_bounds.0 - 1..=board.x_bounds.1 + 1)
                .flat_map(|x| {
                    (board.y_bounds.0 - 1..=board.y_bounds.1 + 1)
                        .filter_map(|y| {
                            let mapped = (0..9).fold(0, |init, n| {
                                let x = x + (n % 3 - 1);
                                let y = y + (n / 3 - 1);
                                (init << 1) + if board.get_value(x, y, iter) { 1 } else { 0 }
                            });
                            if mapping[mapped] {
                                Some((x, y))
                            } else {
                                None
                            }
                        })
                        .collect_vec()
                })
                .collect(),
        );
    });
    println!("{}", board.board.len());
}
