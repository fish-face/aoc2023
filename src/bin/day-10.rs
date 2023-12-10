use std::iter::zip;
use aoc2023::common::{read_input_lines};
use aoc2023::coord::{PointSet, Pt};
use aoc2023::grid::Grid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    W, E, N, S,
}

#[inline]
fn dirs(c: char) -> Option<[Dir; 2]> {
    match c {
        '-' => Some([Dir::E, Dir::W]),
        '|' => Some([Dir::N, Dir::S]),
        'F' => Some([Dir::S, Dir::E]),
        'J' => Some([Dir::W, Dir::N]),
        'L' => Some([Dir::N, Dir::E]),
        '7' => Some([Dir::S, Dir::W]),
        _ => None,
    }
}

#[inline]
fn matches_dir(c: char, d: Dir) -> bool {
    match d {
        Dir::W => c == '-' || c == 'J' || c == '7',
        Dir::E => c == '-' || c == 'L' || c == 'F',
        Dir::N => c == '|' || c == 'J' || c == 'L',
        Dir::S => c == '|' || c == '7' || c == 'F',
    }
}

#[inline]
fn flip(d: Dir) -> Dir {
    match d {
        Dir::W => Dir::E,
        Dir::E => Dir::W,
        Dir::N => Dir::S,
        Dir::S => Dir::N,
    }
}

#[inline]
fn go(p: Pt<usize>, d: Dir) -> Pt<usize> {
    match d {
        Dir::W => p - Pt(1, 0),
        Dir::E => p + Pt(1, 0),
        Dir::N => p - Pt(0, 1),
        Dir::S => p + Pt(0, 1),
    }
}

fn walk(grid: &Grid<char>, pt: &mut Pt<usize>, from_dir: &mut Dir) {
    if let Some(dirs) = dirs(grid[*pt]) {
        let dir = dirs[0];
        if dir != *from_dir {
            *pt = go(*pt, dir);
            *from_dir = flip(dir);
            return;
        }
        let dir = dirs[1];
        if dir != *from_dir {
            *pt = go(*pt, dir);
            *from_dir = flip(dir);
            return;
        }
    }
    panic!("Couldn't find connection from {:?} which is {:?} which was not in dir {:?}", pt, grid[*pt], from_dir);
}

fn count_parity(grid: &Grid<char>, path: &PointSet<usize>) -> usize {
    let mut acc = 0;
    for x in 0..grid.width {
        let mut cur = Pt(x, 0);
        let mut outside = true;
        while cur.1 < grid.height {
            if path.contains(cur) {
                // count when we pass over a piece of the path. We are only travelling south,
                // so passing over a '-' is definitely going from outside to inside or vice-versa.
                // But if we go over a '|', nothing changes, and if we go over two west-facing
                // corners we haven't passed *over* the path, same if we pass over two east-facing
                // ones. We want to flip the parity if we pass over a pair of an east- and a west-
                // connecting corner. But we get the same effect by checking just for east (or just
                // for west) because a second east-connecting corner would flip us again, and we
                // can't end up on something not part of the path without seeing another corner.
                if matches_dir(grid[cur], Dir::E) {
                    outside = !outside;
                }
            } else if !outside {
                acc += 1;
            }
            cur = go(cur, Dir::S);
        }
    }
    acc
}

fn main () {
    let input = read_input_lines()
        .expect("Couldn't read input file")
        .map(|line| line
            .chars()
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    let mut grid = Grid::from_row_data(input.into_iter());
    let start = grid
        .enumerate()
        .find(|(_, e)| **e == 'S')
        .unwrap().0;

    let start_neighbours = start.neighbours4();
    // directions need to be in the same order the neighbours come out... cba doing anything better
    let how_do_you_destructure_into_mut = &mut zip(start_neighbours.iter(), [Dir::E, Dir::W, Dir::S, Dir::N])
        .find(|(pt, from_dir)|
            grid.contains(**pt) && matches_dir(grid[**pt], *from_dir)
        ).unwrap();
    let mut cur = *how_do_you_destructure_into_mut.0;
    let mut from_dir = how_do_you_destructure_into_mut.1;
    let start_dir_a = flip(from_dir);

    let mut steps = 1;
    let mut path = PointSet::new(grid.width);
    path.insert(start);
    while cur != start {
        path.insert(cur);
        walk(&grid, &mut cur, &mut from_dir);
        steps += 1;
    }
    let start_dir_b = from_dir;

    for c in "-|JFL7".chars() {
        if matches_dir(c, start_dir_a) && matches_dir(c, start_dir_b) {
            grid[start] = c;
            break;
        }
    }

    println!("{}", steps / 2);
    println!("{}", count_parity(&grid, &path));
}