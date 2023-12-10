use std::iter::zip;
use aoc2023::common::{read_input_lines};
use aoc2023::coord::{PointSet, Pt};
use aoc2023::grid::Grid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    W, E, N, S,
}

#[derive(Debug, PartialEq, Eq)]
enum Elem {
    S,
    Pipe([Dir; 2]),
    None
}

#[inline]
fn dirs(c: u8) -> Elem {
    let c = c as char;
    match c {
        '-' => Elem::Pipe([Dir::E, Dir::W]),
        '|' => Elem::Pipe([Dir::N, Dir::S]),
        'F' => Elem::Pipe([Dir::S, Dir::E]),
        'J' => Elem::Pipe([Dir::W, Dir::N]),
        'L' => Elem::Pipe([Dir::N, Dir::E]),
        '7' => Elem::Pipe([Dir::S, Dir::W]),
        'S' => Elem::S,
        _ => Elem::None,
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

fn walk(grid: &Grid<Elem>, pt: &mut Pt<usize>, from_dir: &mut Dir) {
    if let Elem::Pipe(dirs) = &grid[*pt] {
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

fn count_parity(grid: &Grid<Elem>, path: &PointSet<usize>) -> usize {
    let mut acc = 0;
    for x in 0..grid.width {
        let mut cur = Pt(x, 0);
        let mut outside = true;
        while cur.1 < grid.height {
            if path.contains(cur) {
                if let Elem::Pipe(dirs) = grid[cur] {
                    if dirs.contains(&Dir::E) {
                        outside = !outside;
                    }
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
            .into_bytes()
            .into_iter()
            .map(dirs)
        )
        .collect::<Vec<_>>();
    let mut grid = Grid::from_row_data(input.into_iter());
    let start = grid
        .enumerate()
        .find(|(_, e)| **e == Elem::S)
        .unwrap().0;

    let start_neighbours = start.neighbours4();
    let foo = &mut zip(start_neighbours.iter(), [Dir::E, Dir::W, Dir::S, Dir::N])
        .find(|(pt, from_dir)|
                  if grid.contains(**pt) {
                      if let Elem::Pipe(dirs) = grid[**pt] {
                          dirs.contains(from_dir)
                      } else {
                          false
                      }
                  } else {
                      false
                  }
        ).unwrap();
    let mut cur = *foo.0;
    let mut from_dir = foo.1;
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

    grid[start] = Elem::Pipe([start_dir_a, start_dir_b]);

    println!("{}", steps / 2);
    println!("{}", count_parity(&grid, &path));
}