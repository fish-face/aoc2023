use std::collections::HashMap;
use aoc2023::common::{read_input_lines, strs_to_nums};

pub struct Intersperse<I, T> {
    orig: I,
    iter: I,
    count: usize,
    intersperse: T,
}

impl<I: Clone + Iterator<Item=T>, T> Intersperse<I, T> {
    pub fn new(iter: I, count: usize, intersperse: T) -> Intersperse<I, T> {
        Intersperse {
            orig: iter.clone(),
            iter,
            count: count-1,
            intersperse,
        }
    }
}

impl<I, T> Iterator for Intersperse<I, T>
    where
        I: Clone + Iterator<Item=T>,
        T: Clone,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        match self.iter.next() {
            None if self.count == 0 => None,
            None => {
                self.iter = self.orig.clone();
                self.count -= 1;
                Some(self.intersperse.clone())
            }
            y => y,
        }
    }
}

#[derive(Debug)]
struct Nonogram1D {
    board: String,
    clues: Vec<usize>,
}

static mut HIT: usize = 0;
static mut MISS: usize = 0;
type Cache<'a> = HashMap<(String, Vec<usize>), usize>;

impl Nonogram1D {
    fn from_line(line: &String) -> Nonogram1D {
        let (board, clues) = line.split_once(' ').unwrap();
        let board = board.to_owned();
        let clues = strs_to_nums(clues.split(',')).collect();
        Nonogram1D{board, clues}
    }

    fn expand_from_line(line: &String) -> Nonogram1D {
        let (board, clues) = line.split_once(' ').unwrap();
        let board = unsafe {
            core::str::from_utf8_unchecked(
                Intersperse::new(board.bytes(), 5, b'?')
                    .collect::<Vec<_>>().as_slice()
            )
        }.to_owned();
        let clues = strs_to_nums(clues.split(',')).collect::<Vec<_>>().repeat(5);
        Nonogram1D{board, clues}
    }

    fn solutions(&mut self, cache: &mut Cache) -> usize {
        self.n_valid_boards(cache, self.board.clone(), self.clues.clone())
    }

    fn maybe_valid(&self, board: &String, clues: &Vec<usize>, new_pos: usize, new_value: u8) -> (usize, usize, bool) {
        let mut block_len = 0;
        let mut clue_idx = 0;
        for (i, cell) in board.bytes().enumerate() {
            let cell = if i == new_pos {
                new_value
            } else {
                cell
            };
            match cell {
                // b'?' => panic!("Can't evaluate partial board"),
                b'?' => {
                    // return true;
                    if block_len == 0 {
                        // this is the "maybe"
                        return (i, clue_idx, true);
                    } else if block_len < clues[clue_idx] {
                        block_len += 1;
                    } else if block_len == clues[clue_idx] {
                        return (i+1, clue_idx+1, !(i+1 == board.len() && clue_idx+1 < clues.len()));
                        // clue_idx += 1;
                        // block_len = 0;
                    }
                },
                b'#' => {
                    if clue_idx >= clues.len() {
                        return (i, 0, false);
                    }
                    if block_len >= clues[clue_idx] {
                        return (i, 0, false);
                    }
                    block_len += 1;
                },
                b'.' => {
                    if block_len > 0 {
                        // println!("finished block len {}, idx {} expt {}", block_len, clue_idx, self.clues[clue_idx]);
                        if block_len != clues[clue_idx] {
                            return (i, 0, false);
                        }
                        block_len = 0;
                        clue_idx += 1;
                    }
                },
                _ => panic!("nope"),
            }
        }
        // println!("finished blocks {}, idx {} returning {}", block_len, clue_idx, clue_idx == self.clues.len() || block_len == self.clues[clue_idx]);
        (board.len(), clues.len(), clue_idx == clues.len() || clue_idx == clues.len() - 1 && block_len == clues[clue_idx])
    }

    fn cached_n_valid_boards(&mut self, cache: &mut Cache, board: String, clues: Vec<usize>) -> usize {
        if let Some(result) = cache.get(&(board.to_string(), clues.to_vec())) {
            unsafe { HIT += 1; }
            *result
        } else {
            let board_clone = board.clone().clone();
            let clues_clone = clues.clone().clone();

            let result = self.n_valid_boards(cache, board, clues);
            cache.insert((board_clone, clues_clone), result);
            unsafe { MISS += 1; }
            result
        }
    }

    fn n_valid_boards(&mut self, cache: &mut Cache, board: String, clues: Vec<usize>) -> usize {
        if board.len() == 0 && clues.len() == 0 {
            return 1;
        }
        let mut result = 0;
        let mut found = false;
        // find unknown
        for (i, b) in board.bytes().enumerate() {
            if b == b'?' {
                found = true;
                let (skip, clue_idx, valid) = self.maybe_valid(&board, &clues, i, b'#');
                if valid {
                    let new_board = board.chars().skip(skip).collect();
                    let new_clues = clues[clue_idx..].to_vec();
                    result += self.cached_n_valid_boards(cache, new_board, new_clues);
                }

                let (skip, clue_idx, valid) = self.maybe_valid(&board, &clues, i, b'.');
                if valid {
                    let new_board = board.chars().skip(skip).collect();
                    let new_clues = clues[clue_idx..].to_vec();
                    result += self.cached_n_valid_boards(cache, new_board, new_clues);
                }
                break;
            }
        }
        if !found && self.maybe_valid(&board, &clues, 1_000_000, 0).2 {
            result + 1
        } else {
            result
        }
    }
}

fn main() {
    let lines = read_input_lines().unwrap().collect::<Vec<_>>();

    let mut cache = Cache::new();

    println!("{}", lines.iter().map(|l| Nonogram1D::from_line(&l).solutions(&mut cache)).sum::<usize>());
    println!("{}", lines.iter().map(|l| Nonogram1D::expand_from_line(&l).solutions(&mut cache)).sum::<usize>());
}