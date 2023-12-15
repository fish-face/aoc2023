use std::collections::{HashMap};
use std::fmt::{Debug, Display, Formatter};
use bitvec::prelude::*;
use itertools::{Either, Itertools};
use aoc2023::common::{read_input};
use aoc2023::coord::{Pt};

pub struct PointSet {
    width: usize,
    height: usize,
    storage: BitVec,
}

impl PointSet {
    pub fn new(width: usize, height: usize) -> Self {
        let mut storage = bitvec![];
        storage.resize(width * height, false);
        PointSet {width, height, storage}
    }

    pub fn set(&mut self, p: Pt<usize>) {
        self.storage.set(p.0 + p.1 * self.width, true);
    }

    pub fn contains(&self, p: Pt<usize>) -> bool {
        self.storage[p.0 + p.1 * self.width]
    }

    pub fn pt(&self, i: usize) -> Pt<usize> {
        Pt(i % self.width, i / self.width)
    }

    pub fn idx(&self, p: Pt<usize>) -> usize {
        p.0 + p.1 * self.width
    }
}

impl Display for PointSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[({}x{}): [", self.width, self.height))?;
        if self.storage.count_ones() == 0 {
            f.write_str("]")?;
        } else {
            f.write_fmt(format_args!("{}", self.pt(self.storage.first_one().unwrap())))?;
            for p in self.storage.iter_ones().skip(1) {
                f.write_fmt(format_args!(", {}", self.pt(p)))?;
            }
            f.write_str("]]")?;
        }
        Ok(())
    }
}

impl Debug for PointSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.contains(Pt(x, y)) {
                    f.write_str("#")?;
                } else {
                    f.write_str(".")?;
                }
            }
            if y < self.height - 1 {
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

fn shift(board: &mut PointSet, solid: &PointSet, by: Pt<isize>) -> bool {
    let mut changed = false;
    let mut next_board = bitvec![];
    next_board.resize(board.storage.len(), false);

    let positions = match by {
        Pt(1, 0) => Either::Left(board.storage.iter_ones().rev()),
        Pt(-1, 0) => Either::Right(board.storage.iter_ones()),
        Pt(0, 1) => Either::Left(board.storage.iter_ones().rev()),
        Pt(0, -1) => Either::Right(board.storage.iter_ones()),
        _ => panic!("can't handle shift direction {by}")
    };

    for p in positions {
        let pp: Pt<isize> = board.pt(p).into();
        let next = pp + by;
        if next.0 >= 0 &&
            next.1 >= 0 &&
            next.0 < board.width as isize &&
            next.1 < board.height as isize &&
            !solid.contains(next.into()) &&
            !board.contains(next.into())
        {
            next_board.set(board.idx(next.into()), true);
            changed = true;
        } else {
            next_board.set(p, true);
        }
    }
    board.storage = next_board;
    changed
}

fn weight(board: &BitVec, height: usize) -> usize {
    (height * board.count_ones()) -
        board.iter_ones().map(|idx| idx/height).sum::<usize>()
}

fn main() {
    for _ in 0..1 {
        let input = read_input().unwrap();
        let width = input.as_bytes().iter().find_position(|x| **x == b'\n').unwrap().0;
        let height = input.as_bytes().iter().filter(|b| **b == b'\n').count();

        let mut rollinghams = PointSet::new(width, height);
        let mut walls = PointSet::new(width, height);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.as_bytes().iter().enumerate() {
                match c {
                    b'#' => walls.set(Pt(x, y)),
                    b'O' => rollinghams.set(Pt(x, y)),
                    _ => {},
                };
            }
        }

        let directions = [
            Pt(0, -1),
            Pt(-1, 0),
            Pt(0, 1),
            Pt(1, 0),
        ];

        while shift(&mut rollinghams, &walls, directions[0]) {};
        println!("{}", weight(&rollinghams.storage, height));

        let mut seen_to_iter = HashMap::<BitVec, usize>::new();
        let mut iter_to_seen = vec![];
        for dir in &directions[1..] {
            while shift(&mut rollinghams, &walls, *dir) {};
        }

        const TARGET: usize = 1_000_000_000;

        for i in 1..10000 {
            for dir in directions {
                while shift(&mut rollinghams, &walls, dir) {};
            }
            match seen_to_iter.get(&rollinghams.storage) {
                Some(j) => {
                    let remainder = (TARGET - j) % (i-j);
                    println!("{}", weight(&iter_to_seen[j + remainder - 2], height));
                    return;
                },
                None => {
                    seen_to_iter.insert(rollinghams.storage.clone(), i);
                    iter_to_seen.push(rollinghams.storage.clone());
                },
            };
        }
    }

    panic!("Didn't find a cycle after 10000 iterations");
}