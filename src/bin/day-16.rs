use std::cmp;
use itertools::Itertools;
use aoc2023::common::read_input_bytes;
use aoc2023::coord::Pt;
use aoc2023::grid::Grid;

#[derive(Clone, Copy)]
enum Dir {
    N = 0b0001,
    E = 0b0010,
    S = 0b0100,
    W = 0b1000,
}

type Dirs = u8;

fn contains(dirs: Dirs, dir: Dir) -> bool {
    (dirs & (dir as u8)) != 0
}

fn add(dirs: &mut Dirs, dir: Dir) {
    *dirs |= dir as u8
}

fn propagate(mut p: Pt<isize>, mut dir: Dir, map: &Grid<u8>, dirs_grid: &mut Grid<Dirs>) {
    loop {
        if p.0 >= 0 &&
            p.1 >= 0 &&
            map.contains(p.into()) &&
            !contains(dirs_grid[p.into()], dir)
        {
            add(&mut dirs_grid[p.into()], dir);

            match map[p.into()] {
                b'/' => dir = match dir {
                    Dir::N => Dir::E,
                    Dir::E => Dir::N,
                    Dir::S => Dir::W,
                    Dir::W => Dir::S,
                },
                b'\\' => dir = match dir {
                    Dir::N => Dir::W,
                    Dir::E => Dir::S,
                    Dir::S => Dir::E,
                    Dir::W => Dir::N,
                },
                b'-' => match dir {
                    Dir::N | Dir::S => {
                        propagate(p, Dir::E, map, dirs_grid);
                        dir = Dir::W;
                    },
                    Dir::E | Dir::W => {},
                },
                b'|' => match dir {
                    Dir::N | Dir::S => {},
                    Dir::E | Dir::W => {
                        propagate(p, Dir::N, map, dirs_grid);
                        dir = Dir::S;
                    },
                },
                b'.' => {},
                _ => panic!("nope"),
            };
        } else {
            break;
        }
        p = match dir {
            Dir::N => Pt(p.0, p.1 - 1),
            Dir::E => Pt(p.0 + 1, p.1),
            Dir::S => Pt(p.0, p.1 + 1),
            Dir::W => Pt(p.0 - 1, p.1),
        };
    }
}

fn main() {
    let input = read_input_bytes()
        .group_by(|b| *b == b'\n');
    let input = input
        .into_iter()
        .filter_map(|(newline, line)| if newline {
            None
        } else {
            Some(line.filter(|b| *b != b'\n'))
        });
    let mut grid = Grid::map_from_lines(input, |x| x);
    let dirs_grid = Grid::<Dirs>::new(grid.width, grid.height);

    let mut dirs_grid_clone = dirs_grid.clone();
    propagate(Pt(0, 0), Dir::E, &mut grid, &mut dirs_grid_clone);
    println!("{}", dirs_grid_clone.iter().map(|dirs| (*dirs > 0) as usize).sum::<usize>());

    let horiz = (0..grid.height)
        .map(|y| {
            let mut dirs_grid_clone = dirs_grid.clone();
            propagate(Pt(0, y).into(), Dir::E, &mut grid, &mut dirs_grid_clone);
            let a = dirs_grid_clone.iter().map(|dirs| (*dirs > 0) as usize).sum::<usize>();

            let mut dirs_grid_clone = dirs_grid.clone();
            propagate(Pt(grid.width - 1, y).into(), Dir::W, &mut grid, &mut dirs_grid_clone);
            let b = dirs_grid_clone.iter().map(|dirs| (*dirs > 0) as usize).sum::<usize>();
            cmp::max(a, b)
        }).max().unwrap();

    let vert = (0..grid.width)
        .map(|x| {
            let mut dirs_grid_clone = dirs_grid.clone();
            propagate(Pt(x, 0).into(), Dir::S, &mut grid, &mut dirs_grid_clone);
            let a = dirs_grid_clone.iter().map(|dirs| (*dirs > 0) as usize).sum::<usize>();
            // println!("{}\n{a}\n", dirs_grid_clone.map(|dirs| if (*dirs as u8 > 0) {'#'} else {'.'}).to_string(Some("")));

            let mut dirs_grid_clone = dirs_grid.clone();
            propagate(Pt(x, grid.height - 1).into(), Dir::N, &mut grid, &mut dirs_grid_clone);
            let b = dirs_grid_clone.iter().map(|dirs| (*dirs > 0) as usize).sum::<usize>();
            cmp::max(a, b)
        }).max().unwrap();
    println!("{}", cmp::max(horiz, vert));
}