use std::fmt::{Display, Formatter, Write};
use std::iter;
use std::iter::zip;
use bitvec::bitvec;
use bitvec::vec::BitVec;
use itertools::Itertools;
use aoc2023::common::{read_input_lines, strs_to_nums};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Cell {
    Blank, Filled
}

fn cell(c: u8) -> Option<Cell> {
    match c {
        b'.' => Some(Cell::Blank),
        b'#' => Some(Cell::Filled),
        b'?' => None,
        _ => panic!("unknown cell"),
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Blank => '.',
            Cell::Filled => '#',
        })
    }
}

type BoardData = Vec<Option<Cell>>;
#[derive(Debug)]
struct Board { data: BoardData }

#[derive(Debug, Clone)]
struct RulePos {
    start: usize,
    len: usize,
}

type Rules = Vec<RulePos>;

// impl Rules {
//     fn rules_iter(&self) -> RulesIter {
//         RulesIter::from_vec(self)
//     }
// }

#[derive(Debug, Clone)]
struct RulesIter<'a> {
    rules: &'a Rules,
    cur_rule: usize,
    cur_idx: usize,
}

impl <'a>RulesIter<'a> {
    fn from_vec(rules: &Rules) -> RulesIter {
        RulesIter{rules, cur_rule: 0, cur_idx: 0}
    }
}

impl <'a>Iterator for RulesIter<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx < self.rules[self.cur_rule].start {
            self.cur_idx += 1;
            Some(Cell::Blank)
        } else if self.cur_idx - self.rules[self.cur_rule].start < self.rules[self.cur_rule].len {
            self.cur_idx += 1;
            Some(Cell::Filled)
        } else if self.cur_rule >= self.rules.len() - 1 {
            self.cur_idx += 1;
            Some(Cell::Blank)
        } else if self.cur_idx < self.rules[self.cur_rule+1].start {
            self.cur_idx += 1;
            if self.cur_idx == self.rules[self.cur_rule+1].start {
                self.cur_rule += 1;
            }
            Some(Cell::Blank)
        } else {
            None
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.data.iter() {
            match c {
                Some(c) => c.fmt(f)?,
                None => f.write_char('?')?,
            }
        }
        Ok(())
    }
}

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
    known: Board,
    clues: Vec<usize>,
    fit_map: Vec<BitVec>,
    gap_map_seen: Vec<BitVec>,
    gap_map: Vec<BitVec>,
}

impl Nonogram1D {
    fn new(known: Board, clues: Vec<usize>) -> Self {
        let mut res = Nonogram1D{known, clues, fit_map: vec![], gap_map_seen: vec![], gap_map: vec![]};
        // res.preprocess();
        res
    }

    fn from_line(line: &String) -> Nonogram1D {
        let (known, clues) = line.split_once(' ').unwrap();
        let known = known.bytes().map(cell).collect();
        let clues = strs_to_nums(clues.split(',')).collect();
        Self::new(Board{data: known}, clues)
    }
    fn expand_from_line(line: &String) -> Nonogram1D {
        let (known, clues) = line.split_once(' ').unwrap();
        let known = Intersperse::new(known.bytes(), 5, b'?').map(cell).collect();
        let clues = strs_to_nums(clues.split(',')).collect::<Vec<_>>().repeat(5);
        Self::new(Board{data: known}, clues)
    }

    fn solutions(&mut self) -> usize {
        self.n_valid_boards()
        // self.valid_placements()
    }

    fn is_valid(&self, board: &BoardData) -> bool {
        let mut block_len = 0;
        let mut clue_idx = 0;
        for cell in board.iter() {
            let cell = cell.expect("can't determine validity of incomplete board");
            if cell == Cell::Filled {
                if clue_idx >= self.clues.len() {
                    return false;
                }
                block_len += 1;
            } else {
                if block_len > 0 {
                    // println!("finished block len {}, idx {} expt {}", block_len, clue_idx, self.clues[clue_idx]);
                    if block_len != self.clues[clue_idx] {
                        return false;
                    }
                    block_len = 0;
                    clue_idx += 1;
                }
            }
        }
        // println!("finished blocks {}, idx {} returning {}", block_len, clue_idx, clue_idx == self.clues.len() || block_len == self.clues[clue_idx]);
        clue_idx == self.clues.len() || clue_idx == self.clues.len() - 1 && block_len == self.clues[clue_idx]
    }

    fn is_valid_placement(&self, placements: RulesIter) -> bool {
        // assumed that we have the correct number and spacing of placements
        for (_, (known, maybe)) in iter::zip(self.known.data.iter(), placements).enumerate() {
            if known.unwrap_or(maybe) != maybe {
                return false;
            }
        }
        true
    }

    fn preprocess(&mut self) {
        let max_size = self.clues.iter().max().unwrap();
        let mut seen = bitvec![0; *max_size];
        self.fit_map.resize_with(*max_size, Default::default);
        let size = self.size();
        for map in self.fit_map.iter_mut() {
            map.resize(size, true);
        }

        self.gap_map_seen.resize_with(self.size(), Default::default);
        self.gap_map.resize_with(self.size(), Default::default);
        for (seen, map) in zip(self.gap_map_seen.iter_mut(), self.gap_map.iter_mut()) {
            seen.resize(size, false);
            map.resize(size, false);
        }
        for size in self.clues.iter().unique() {
            if seen[*size-1] {
                continue;
            }
            seen.set(*size-1, true);

            for start in 0..self.size() {
                if start+size > self.size() {
                    self.fit_map[*size-1].set(start, false);
                    continue;
                }
                for i in start..start+size {
                    if self.known.data[i] == Some(Cell::Blank) {
                        self.fit_map[*size-1].set(start, false);
                        break;
                    }
                }
            }
        }
    }

    fn can_fit(&self, next_start: usize, next_len: usize) -> bool {
        self.fit_map[next_len-1][next_start]
    }

    fn gap_valid(&mut self, placements: &Rules, next_start: usize) -> bool {
        let gap_start = if placements.len() >= 1 {
            let penultimate = &placements[placements.len() - 1];
            penultimate.start + penultimate.len
        } else {
            0
        };
        if self.gap_map_seen[gap_start][next_start] {
            return self.gap_map[gap_start][next_start];
        }
        self.gap_map_seen[gap_start].set(next_start, true);
        for i in gap_start..next_start {
            if self.known.data[i] == Some(Cell::Filled) {
                self.gap_map[gap_start].set(next_start, false);
                return false;
            }
        }
        self.gap_map[gap_start].set(next_start, true);
        true
    }

    // fn is_next_placement_valid(&self, placements: &Rules, next_start: usize, next_len: usize) -> bool {
    //     let gap_start = if placements.len() >= 1 {
    //         let penultimate = &placements[placements.len() - 1];
    //         penultimate.start + penultimate.len
    //     } else {
    //         0
    //     };
    //     for i in gap_start..next_start {
    //         if self.known.data[i] == Some(Cell::Filled) {
    //             return false;
    //         }
    //     }
    //     for i in next_start..next_start + next_len {
    //         if self.known.data[i] == Some(Cell::Blank) {
    //             return false;
    //         }
    //     }
    //     // if placements.len() == self.clues.len() - 1 {
    //     //     for i in next_start+next_len..self.size() {
    //     //         if self.known.data[i] == Some(Cell::Filled) {
    //     //             return false;
    //     //         }
    //     //     }
    //     // }
    //     true
    // }

    fn size(&self) -> usize {
        self.known.data.len()
    }

    // fn valid_placements(&self) -> impl Iterator<Item = Rules> {
    fn valid_placements(&mut self) -> usize {
        // let mut result = vec![];
        let mut result = 0;

        let mut queue = vec![vec![]];

        while let Some(partial_placements) = queue.pop() {
            let next_rule = partial_placements.len();
            let is_last = next_rule == self.clues.len() - 1;

            let remaining_padding = self.clues.len() - next_rule - 1;
            let end_at = self.size() - self.clues[next_rule..].iter().sum::<usize>() - remaining_padding + 1;

            let next_pos_start = if next_rule > 0 {
                let last_placed: &RulePos = &partial_placements[next_rule-1];
                last_placed.start + last_placed.len + 1
            } else {
                0
            };
            let next_len = self.clues[next_rule];

            for start in next_pos_start..end_at {
                if !self.can_fit(start, next_len) {
                    continue;
                }
                if !self.gap_valid(&partial_placements, start) {
                    continue;
                }
                let mut new = partial_placements.clone();
                new.push(RulePos{start, len: next_len});
                if is_last {
                    // if self.is_valid_placement(RulesIter::from_vec(&new)) {
                        // result.push(new);
                        // println!("{}", RulesIter::from_vec(&new).take(self.size()).join(""));
                        result += 1;
                    // }
                } else {
                    queue.push(new);
                }
            }
        }

        // result.into_iter().map(|r| r)
        result
    }

    fn board_could_be_valid(&mut self, board: &BoardData, new_pos: usize, new: Cell) -> bool {
        let mut block_len = 0;
        let mut clue_idx = 0;
        for (i, cell) in board.iter().enumerate() {
            let cell = match cell {
                Some(cell) => *cell,
                None => if i == new_pos {
                    new
                } else {
                    return true;
                }
            };
            if cell == Cell::Filled {
                if clue_idx >= self.clues.len() {
                    return false;
                }
                block_len += 1;
            } else {
                if block_len > 0 {
                    // println!("finished block len {}, idx {} expt {}", block_len, clue_idx, self.clues[clue_idx]);
                    if block_len != self.clues[clue_idx] {
                        return false;
                    }
                    block_len = 0;
                    clue_idx += 1;
                }
            }
        }
        // println!("finished blocks {}, idx {} returning {}", block_len, clue_idx, clue_idx == self.clues.len() || block_len == self.clues[clue_idx]);
        clue_idx == self.clues.len() || clue_idx == self.clues.len() - 1 && block_len == self.clues[clue_idx]
    }

    fn valid_boards(&self) -> Vec<Board> {
        let mut result = vec![];
        let mut queue = vec![self.known.data.clone()];
        while let Some(item) = queue.pop() {
            if let Some((unknown_pos, _)) = item
                .iter()
                .enumerate()
                .filter(|(_, c)| c.is_none()).next()
            {
                let mut fill = item.clone();
                fill[unknown_pos] = Some(Cell::Filled);
                queue.push(fill);
                let mut unfill = item.clone();
                unfill[unknown_pos] = Some(Cell::Blank);
                queue.push(unfill);
            } else {
                // println!("  valid? {}", self.is_valid(&item));
                // println!("  {}", Board{data: item});
                if self.is_valid(&item) {
                    result.push(Board{data: item});
                }
            }
        }
        result
    }

    fn n_valid_boards(&mut self) -> usize {
        let mut result = 0;
        let mut queue = vec![self.known.data.clone()];
        while let Some(item) = queue.pop() {
            if let Some((unknown_pos, _)) = item
                .iter()
                .enumerate()
                .filter(|(_, c)| c.is_none()).next()
            {
                if self.board_could_be_valid(&item, unknown_pos, Cell::Filled) {
                    let mut fill = item.clone();
                    fill[unknown_pos] = Some(Cell::Filled);
                    queue.push(fill);
                }
                if self.board_could_be_valid(&item, unknown_pos, Cell::Blank) {
                    let mut unfill = item.clone();
                    unfill[unknown_pos] = Some(Cell::Blank);
                    queue.push(unfill);
                }
            } else {
                if self.is_valid(&item) {
                    result += 1;
                }
            }
        }
        result
    }
    // fn all_boards(&self) -> impl Iterator<Item=Board> {
    //     self.known.iter().enumerate().filter(|(i, c)| c.is_none())
    // }
}

fn main() {
    let lines = read_input_lines().unwrap().collect::<Vec<_>>();

    // for n in lines.iter().map(|l| Nonogram1D::from_line(&l)).skip(0) {
    //     println!("{} {:?}", n.known, n.clues);
    //     // n.valid_boards();
    //     // for board in n.valid_boards() {
    //     //     println!("  {}", board);
    //     // }
    //
    //     n.valid_placements();
    //     // for p in n.valid_placements() {
    //         // if n.is_valid_placement(p.clone()) {
    //         //     println!("{}", p.take(n.known.data.len()).join(""));
    //         // }
    //     // }
    // }

    // println!("{:?}", lines.iter().map(|l| Nonogram1D::from_line(&l).solutions()).collect::<Vec<_>>());
    println!("{}", lines.iter().map(|l| Nonogram1D::from_line(&l).solutions()).sum::<usize>());
    println!("{}", lines.iter().map(|l| {
        println!("{}", l);
        Nonogram1D::expand_from_line(&l).solutions()
    }).sum::<usize>());
}