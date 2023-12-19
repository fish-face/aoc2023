use std::cmp::{max, min};
use std::collections::{BTreeSet};
use std::iter::{FilterMap, Map};
use std::mem::transmute;
use itertools::{Group, Groups, Itertools};
use aoc2023::common::{read_input_bytes, read_input_lines};
use aoc2023::coord::Pt;

#[derive(PartialOrd, PartialEq, Eq, Ord, Copy, Clone)]
enum Dir { N, E, S, W }

#[derive(Copy, Clone)]
struct Instruction {
    dir: Dir,
    dist: isize,
}

impl Instruction {
    fn from_bytes(mut bytes: impl Iterator<Item=u8>) -> (Self, Self) {
        let dir1 = match bytes.next().unwrap() {
            b'U' => Dir::N,
            b'R' => Dir::E,
            b'D' => Dir::S,
            b'L' => Dir::W,
            _ => {panic!("nope");}
        };
        bytes.nth(0);
        let dist_bytes = bytes.by_ref().take_while(|b| *b != b' ');
        let mut dist1 = 0;
        for b in dist_bytes {
            dist1 *= 10;
            dist1 += (b - b'0') as isize;
        }
        bytes.nth(1);
        // todo this heap allocates making all the bytes and unsafeness pointless
        let dist2_bytes = bytes.by_ref().take(5).collect();
        let dist2 = unsafe { isize::from_str_radix(&String::from_utf8_unchecked(dist2_bytes), 16).unwrap() };
        let dir2 = unsafe { transmute(bytes.next().unwrap() - b'0') };

        (Instruction{ dir: dir1, dist: dist1}, Instruction{dist: dist2, dir: dir2})
    }
}

fn area(instructions: impl Iterator<Item=Instruction>) -> usize {
    let mut current = Pt(0_isize, 0_isize);
    let mut x_crossings = BTreeSet::new();
    let mut y_crossings = BTreeSet::new();

    let mut perimeter = 0;

    for instruction in instructions {
        let next = match instruction.dir {
            Dir::N => {
                let next = Pt(current.0, current.1 - instruction.dist);
                x_crossings.insert(current.0);
                next
            }
            Dir::S => {
                let next = Pt(current.0, current.1 + instruction.dist);
                x_crossings.insert(current.0);
                next
            }
            Dir::E => {
                let next = Pt(current.0 + instruction.dist, current.1);
                y_crossings.insert((current.1, (current.0, next.0)));
                next
            }
            Dir::W => {
                let next = Pt(current.0 - instruction.dist, current.1);
                y_crossings.insert((current.1, (next.0, current.0)));
                next
            }
        };

        current = next;
        perimeter += instruction.dist;
    }

    let mut xc_prev = x_crossings.pop_first().unwrap();
    let mut yc_prev = isize::MAX;

    let mut acc = 0;

    for xc in x_crossings.iter() {
        let width = xc - xc_prev;
        let mut counting = false;
        let mut height = 0;
        for (yc, (xra, xrb)) in y_crossings.iter() {

            if !(xc > xra && xc <= xrb) {
                continue;
            }

            if *yc >= yc_prev {
                height = *yc - yc_prev;
            }

            // println!("{xc},{yc} {width}x{height} left {}{xra}--{xrb} --> {counting}", if going_right { '+' } else { '-' });
            if counting {
                acc += width * (height + 0);
            }
            counting = !counting;

            yc_prev = *yc;
        }

        xc_prev = *xc;
    }

    (acc + perimeter / 2 + 1).try_into().unwrap()
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
        .map(Instruction::from_bytes).collect::<Vec<_>>();

    let part1 = area(instructions.iter().map(|(a, b)| *a));
    let part2 = area(instructions.into_iter().map(|(a, b)| b));
    println!("{part1}\n{part2}");
}
