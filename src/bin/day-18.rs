use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::iter::once;
use itertools::Itertools;
use aoc2023::common::{read_input_bytes, read_input_lines};
use aoc2023::coord::Pt;
use aoc2023::grid::Grid;

enum Dir { N, E, S, W }

struct Instruction {
    dir: Dir,
    dist: usize,
    colour: u32,
}

fn offset(dir: Dir) -> Pt<isize> {
    match dir {
        Dir::N => Pt( 0, -1),
        Dir::E => Pt( 1,  0),
        Dir::S => Pt( 0,  1),
        Dir::W => Pt(-1,  0),
    }
}

impl Instruction {
    fn from_bytes(mut bytes: impl Iterator<Item=u8>) -> Self {
        let dir = match bytes.next().unwrap() {
            b'U' => Dir::N,
            b'R' => Dir::E,
            b'D' => Dir::S,
            b'L' => Dir::W,
            _ => {panic!("nope");}
        };
        bytes.nth(0);
        let dist_bytes = bytes.by_ref().take_while(|b| *b != b' ');
        let mut dist = 0;
        for b in dist_bytes {
            dist *= 10;
            dist += (b - b'0') as usize;
        }
        bytes.nth(1);
        let colour_bytes = bytes.take(6).collect();
        // let colour_bytes = [
        //     bytes.next().unwrap(),
        //     bytes.next().unwrap(),
        //     bytes.next().unwrap(),
        //     bytes.next().unwrap(),
        //     bytes.next().unwrap(),
        //     bytes.next().unwrap(),
        // ];
        // todo this heap allocates making all the bytes and unsafeness pointless
        let colour = unsafe { u32::from_str_radix(&String::from_utf8_unchecked(colour_bytes), 16).unwrap() };
        // let colour = unsafe { String::from_utf8_unchecked(Vec::from(colour_bytes)) }.parse().unwrap();

        Instruction{dir, dist, colour}
    }
}

fn flood_fill<T: Copy>(map: &mut Grid<T>, start: Pt<usize>, cond: impl Fn(T) -> bool, value: T) -> usize {
    let mut to_visit: Vec<_> = vec![start];
    let mut visited = HashSet::new();
    visited.insert(start);
    while let Some(start) = to_visit.pop() {
        map[start] = value;
        for neighbour in start.neighbours4() {
            if map.contains(neighbour) && !visited.contains(&neighbour) && cond(map[neighbour]) {
                visited.insert(neighbour);
                to_visit.push(neighbour);
            }
        }
    }
    visited.len()
}

fn main() {
    let input = read_input_bytes()
        .group_by(|b| *b == b'\n');
    let input = input
        .into_iter()
        .filter_map(|(newline, line)| if newline {
            None
        } else {
            Some(line)
        });
    let instructions = input
        .map(Instruction::from_bytes);

    let (mut current, mut map) = if true {
        (Pt(5000_usize, 5000_usize), Grid::new(10_000, 10_000))
    } else {
        (Pt(40_usize, 40_usize), Grid::new(80, 80))
    };

    let top_border_pt = current;
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (usize::MAX, usize::MAX, usize::MIN, usize::MIN);
    let mut border_len = 0;
    for instruction in instructions {
        let offset = offset(instruction.dir);
        for _ in 0..instruction.dist {
            map[current] = Some(instruction.colour);
            border_len += 1;
            let icurrent: Pt<isize> = current.into();
            current = (icurrent + offset).into();
            min_x = min(min_x, current.0);
            min_y = min(min_y, current.1);
            max_x = max(max_x, current.0);
            max_y = max(max_y, current.1);
        }
    }

    current = top_border_pt;
    // find interior point
    while map[Pt(current.0, current.1 + 1)].is_some() {
        current = Pt(current.0 + 1, current.1);
    }
    let interior = Pt(current.0, current.1 + 1);

    // println!("{}", interior);
    println!("{}", border_len + flood_fill(&mut map, interior, |colour| colour.is_none(), Some(0)));

    // println!("{}", map.map(|x| if x.is_none() {'.'} else {'#'}).to_string(Some("")))
    // println!("{} {} {} {}", min_x, min_y, max_x, max_y);
}