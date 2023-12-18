use std::iter::zip;
use bitvec::prelude::*;
use aoc2023::common::read_input;

fn one_off(a: &BitVec, b: &BitVec) -> bool {
    let mut off = 0;
    for (aa, bb) in zip(a.iter(), b.iter()) {
        if aa != bb {
            off += 1;
        }
        if off > 1 {
            return false;
        }
    }
    true
}

fn find_reflections(grid: &Vec<BitVec<usize, Lsb0>>, allow_smudge: bool) -> Option<usize> {
    let mut stack: Vec<&BitVec> = vec![];
    for (i, line) in grid.iter().enumerate() {
        let mut smudge_allowed = allow_smudge;
        if i > 0 {
            // check that this possible reflection point is possible with this data
            let mut backward = i - 1;
            let mut forward = i;
            loop {
                let test = stack[backward];
                if *test != grid[forward] {
                    if smudge_allowed && one_off(&test, &grid[forward]) {
                        smudge_allowed = false;
                    } else {
                        break;
                    }
                }
                if backward > 0 && forward < grid.len() - 1 {
                    backward -= 1;
                    forward += 1;
                } else {
                    if !smudge_allowed {
                        return Some(i);
                    }
                    break;
                }
            }
        }
        stack.push(line);
    }
    None
}

fn find_smudged_reflection(horiz: Vec<BitVec>, vert: Vec<BitVec>) -> (usize, usize) {
    // horizontal grid contains data in normal order so we can compare rows -> we use it to check vertical symmetry

    let unsmudged_horiz = find_reflections(&vert, false);
    let unsmudged_vert = find_reflections(&horiz, false);
    let smudged_horiz = find_reflections(&vert, true);
    let smudged_vert = find_reflections(&horiz, true);

    return (
        unsmudged_horiz.unwrap_or_else(
            || unsmudged_vert.unwrap() * 100),
        smudged_horiz.unwrap_or_else(
            || smudged_vert.unwrap() * 100));
}

fn horiz_grid(block: &str) -> Vec<BitVec> {
    let mut result = vec![];
    for line in block.lines().map(|line| line.bytes()) {
        let mut bits = bitvec![];
        for b in line {
            bits.push(b == b'#');
        }
        result.push(bits);
    }
    result
}

fn vert_grid(block: &str) -> Vec<BitVec> {
    let mut result = vec![];
    let mut lines = block.lines().map(|line| line.bytes()).collect::<Vec<_>>();
    for _ in 0..lines[0].len() {
        let mut bits = bitvec![];
        for row in lines.iter_mut() {
            let b = row.next().unwrap();
            bits.push(b == b'#');
        }
        result.push(bits);
    }
    result
}

fn main() {
    let bleh = read_input().unwrap();
    let inputs = bleh.split("\n\n");

    let grids = inputs
        .map(|block| (horiz_grid(block), vert_grid(block)));

    let both = grids
        .map(|(h, v)| find_smudged_reflection(h, v))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();
    println!("{}\n{}", both.0, both.1);
}
