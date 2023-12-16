use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use bitvec::prelude::*;
use itertools::{Either, Itertools};
use aoc2023::common::read_input;
use aoc2023::coord::Pt;

pub struct PointSet {
    width: usize,
    height: usize,
    storage: BitVec,
}

impl PointSet {
    fn new(width: usize, height: usize) -> Self {
        let mut storage = bitvec![];
        storage.resize(width * height, false);
        PointSet {width, height, storage}
    }

    fn set(&mut self, p: Pt<usize>, value: bool) {
        self.storage.set(p.0 + p.1 * self.width, value);
    }

    fn contains(&self, p: Pt<usize>) -> bool {
        self.storage[p.0 + p.1 * self.width]
    }

    fn pt(&self, i: usize) -> Pt<usize> {
        Pt(i % self.width, i / self.width)
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

fn shift(board: &mut PointSet, solid: &PointSet, by: Pt<isize>) {
    let bugger_off_rust = board.storage.clone();
    let ones = bugger_off_rust.iter_ones();
    let positions = match by {
        Pt(1, 0) => Either::Left(ones.rev()),
        Pt(-1, 0) => Either::Right(ones),
        Pt(0, 1) => Either::Left(ones.rev()),
        Pt(0, -1) => Either::Right(ones),
        _ => panic!("can't handle shift direction {by}")
    };

    for p in positions {
        let mut p = board.pt(p);
        let pp: Pt<isize> = p.into();
        let mut next = pp + by;
        while next.0 >= 0 &&
            next.1 >= 0 &&
            next.0 < board.width as isize &&
            next.1 < board.height as isize &&
            !solid.contains(next.into()) &&
            !board.contains(next.into())
        {
            board.set(p, false);
            board.set(next.into(), true);
            p = next.into();
            let pp: Pt<isize> = p.into();
            next = pp + by;
        }
    }
}

fn weight(board: &BitVec, height: usize) -> usize {
    (height * board.count_ones()) -
        board.iter_ones().map(|idx| idx/height).sum::<usize>()
}

const DIRECTIONS: [Pt<isize>; 4] = [
    Pt(0, -1),
    Pt(-1, 0),
    Pt(0, 1),
    Pt(1, 0),
];

fn cycle(board: &mut PointSet, solid: &PointSet) {
    for dir in DIRECTIONS {
        shift(board, solid, dir);
    }
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
                    b'#' => walls.set(Pt(x, y), true),
                    b'O' => rollinghams.set(Pt(x, y), true),
                    _ => {},
                };
            }
        }

        shift(&mut rollinghams, &walls, DIRECTIONS[0]);
        println!("{}", weight(&rollinghams.storage, height));

        let mut seen_to_iter = HashMap::<BitVec, usize>::new();
        let mut iter_to_seen = vec![];
        for dir in &DIRECTIONS[1..] {
            shift(&mut rollinghams, &walls, *dir);
        }

        const TARGET: usize = 1_000_000_000;

        for i in 1..10000 {
            cycle(&mut rollinghams, &walls);
            match seen_to_iter.get(&rollinghams.storage) {
                Some(j) => {
                    let remainder = (TARGET - j - 1) % (i-j);
                    println!("{}", weight(&iter_to_seen[j + remainder - 1], height));
                    return;
                },
                None => {
                    println!("{i}: {:?}", weight(&rollinghams.storage, height));
                    seen_to_iter.insert(rollinghams.storage.clone(), i);
                    iter_to_seen.push(rollinghams.storage.clone());
                },
            };
        }
    }

    panic!("Didn't find a cycle after 10000 iterations");
}