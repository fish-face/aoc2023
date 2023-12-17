use bitvec::prelude::*;
use aoc2023::common::read_input;
use aoc2023::grid::Grid;

fn possible_reflections(possible: &mut BitVec, row_or_col: &[u8], allow_smudge: usize) {
    let mut stack = vec![];
    for (i, c) in row_or_col.iter().enumerate() {
        let mut smudge_allowed = allow_smudge;
        stack.push(*c);
        if possible[i] {
            // check that this possible reflection point is possible with this data
            let mut backward = i;
            let mut forward = i + 1;
            loop {
                let test = stack[backward];
                if test != row_or_col[forward] {
                    if smudge_allowed > 0 {
                        smudge_allowed -= 1;
                    } else {
                        possible.set(i, false);
                        break;
                    }
                }
                if backward > 0 && forward < row_or_col.len() - 1 {
                    backward -= 1;
                    forward += 1;
                } else {
                    break;
                }
            }

            if smudge_allowed != 0 {
                possible.set(i, false);
            }
        }
    }
}

fn find_smudged_reflection(grid: Grid<u8>) -> (usize, usize) {
    // first find unsmudged reflections

    // horizontal
    let mut unsmudged_horiz = bitvec![usize, Lsb0;];
    unsmudged_horiz.resize(grid.width-1, true);
    unsmudged_horiz.push(false);
    for row in grid.rows() {
        possible_reflections(&mut unsmudged_horiz, row, 0);
    }

    // vertical
    let mut unsmudged_vert = bitvec![usize, Lsb0;]; // true; grid.width];
    let mut part1 = 0;
    if unsmudged_horiz.count_ones() == 1 {
        unsmudged_vert.resize(grid.height, false);
        part1 = unsmudged_horiz.first_one().unwrap() + 1;
    } else {
        unsmudged_vert.resize(grid.height-1, true);
        unsmudged_vert.push(false);
        for col in grid.columns() {
            possible_reflections(&mut unsmudged_vert, col.copied().collect::<Vec<_>>().as_slice(), 0);
            if unsmudged_vert.count_ones() == 1 {
                part1 = (unsmudged_vert.first_one().unwrap() + 1) * 100;
                break;
            }
        }
    }
    // println!("done part1:\n{unsmudged_horiz}\n{unsmudged_vert}");

    // now find smudged
    
    // horizontal
    for smudge_idx in 0..grid.height {
        let mut reflections = !unsmudged_horiz.clone();
        reflections.set(grid.width-1, false);
        for (y, row) in grid.rows().enumerate() {
            if smudge_idx == 9 && y == 9 {
                // println!("break");
            }
            // println!("horiz: {reflections}");
            possible_reflections(&mut reflections, row, (y == smudge_idx) as usize);
        }
        // println!("-->    {reflections}");
        if reflections.count_ones() == 1 {
            // return reflections.first_one().unwrap() + 1;
            return (part1, reflections.first_one().unwrap() + 1);
        }
    }

    // vertical
    for smudge_idx in 0..grid.width {
        let mut reflections = !unsmudged_vert.clone();
        reflections.set(grid.height-1, false);
        for (x, col) in grid.columns().enumerate() {
            possible_reflections(&mut reflections, col.copied().collect::<Vec<_>>().as_slice(), (x == smudge_idx) as usize);
            // println!("vert: {reflections}");
        }
        if reflections.count_ones() == 1 {
            return (part1, (reflections.first_one().unwrap() + 1) * 100);
            // return (reflections.first_one().unwrap() + 1) * 100;
        }
        // println!("-->   {reflections}");
    }
    // return 0;
    panic!("Couldn't find reflection for {}", grid.map(|c| *c as char).to_string(Some("")))
}

fn main() {
    let bleh = read_input().unwrap();
    let inputs = bleh.split("\n\n");
    let grids = inputs
        .map(|block| Grid::map_from_lines(block.lines().map(|line| line.bytes()), |x| x));

    let both = grids.map(find_smudged_reflection).reduce(|a, b| (a.0 + b.0, a.1 + b.1)).unwrap();
    println!("{}\n{}", both.0, both.1);
}
