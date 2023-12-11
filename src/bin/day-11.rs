use std::cmp::{max, min};
use bitvec::bitvec;
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use num::abs;
use aoc2023::common::read_input_lines;
use aoc2023::coord::Pt;
use aoc2023::grid::Grid;

#[derive(Debug, Eq, PartialEq)]
enum G {
    Empty, Galaxy,
}

fn from_byte(b: u8) -> G {
    match b {
        b'.' => G::Empty,
        b'#' => G::Galaxy,
        _ => panic!("bad input"),
    }
}

fn main() {
    let (grid, galaxies) = Grid::map_from_lines_and_find(read_input_lines().unwrap(),
                                                         from_byte,
                                                         |x| *x == G::Galaxy);
    // let empty_y = BitSet::from_iter(grid
    //     .rows()
    //     .enumerate()
    //     .filter_map(|(y, row)| match row.iter().all(|g| g == G::Empty) {
    //         true => Some(y),
    //         false => None
    //     })
    // );
    let mut empty_x = bitvec![0; grid.width];
    let mut empty_y = bitvec![0; grid.height];
    galaxies.iter().for_each(|Pt(x, y)| {
        empty_x.set(*x, true);
        empty_y.set(*y, true);
    });
    let empty_x = !empty_x;
    let empty_y = !empty_y;

    let mut part1 = 0;
    let mut part2 = 0_usize;

    for (i, ga) in galaxies[..galaxies.len()-1].iter().enumerate() {
        for gb in galaxies[i+1..].iter() {
            let from_x = min(ga.0, gb.0)+1;
            let to_x = max(ga.0, gb.0);
            let from_y = ga.1+1;
            let to_y = gb.1;
            for i in from_x..=to_x {
                part1 += if empty_x[i] {
                    2
                } else {
                    1
                };
                part2 += if empty_x[i] {
                    1_000_000
                } else {
                    1
                };
            }
            for i in from_y..=to_y {
                part1 += if empty_y[i] {
                    2
                } else {
                    1
                };
                part2 += if empty_y[i] {
                    1_000_000
                } else {
                    1
                };
            }
        }
    }
    // let galaxies = galaxies.collect::<Vec<_>>();
    // println!("{:?}\n{:?}\n{:?}", galaxies, galaxy_x, galaxy_y);
    println!("{}\n{}", part1, part2);
}