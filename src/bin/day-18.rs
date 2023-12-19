use std::mem::transmute;
use aoc2023::common::read_input_lines;
use aoc2023::coord::Pt;

#[derive(PartialOrd, PartialEq, Eq, Ord, Copy, Clone)]
enum Dir { N, E, S, W }

#[derive(Copy, Clone)]
struct Instruction {
    dir: Dir,
    dist: isize,
}

impl Instruction {
    fn from_str(s: String) -> (Self, Self) {
        let s = s.as_bytes();
        let dir1 = match s[0] {
            b'U' => Dir::N,
            b'R' => Dir::E,
            b'D' => Dir::S,
            b'L' => Dir::W,
            _ => {panic!("nope");}
        };
        let mut dist1 = 0;
        let mut part2 = 0;
        for (i, b) in s[2..].iter().enumerate() {
            if *b == b' ' {
                part2 = i;
                break;
            }
            dist1 *= 10;
            dist1 += (*b - b'0') as isize;
        }
        let mut dist2 = 0_isize;
        for b in s[part2+5..part2+10].iter() {
            dist2 *= 16;
            dist2 += (*b as char).to_digit(16).expect(format!("invalid: {}", b).as_str()) as isize;
        }
        let dir2 = unsafe { transmute(s[part2+10] - b'0') };

        (Instruction{ dir: dir1, dist: dist1}, Instruction{dist: dist2, dir: dir2})
    }
}

#[inline]
fn increment_area(pt: &mut Pt<isize>, area: &mut isize, perimeter: &mut isize, inst: Instruction) {
    let next = match inst.dir {
        Dir::N => Pt(pt.0, pt.1 - inst.dist),
        Dir::S => Pt(pt.0, pt.1 + inst.dist),
        Dir::E => Pt(pt.0 + inst.dist, pt.1),
        Dir::W => Pt(pt.0 - inst.dist, pt.1),
    };
    *area += pt.0 * next.1 - pt.1 * next.0;
    *pt = next;
    *perimeter += inst.dist;
}

fn main() {
    let input = read_input_lines().unwrap();
    let instructions = input
        .map(Instruction::from_str);

    let mut p1_pt = Pt(0, 0);
    let mut p1_area = 0;
    let mut p1_perimeter = 0;
    let mut p2_pt = Pt(0, 0);
    let mut p2_area = 0;
    let mut p2_perimeter = 0;
    for (p1_inst, p2_inst) in instructions {
        increment_area(&mut p1_pt, &mut p1_area, &mut p1_perimeter, p1_inst);
        increment_area(&mut p2_pt, &mut p2_area, &mut p2_perimeter, p2_inst);
    }
    println!("{}", (p1_area + p1_perimeter) / 2 + 1);
    println!("{}", (p2_area + p2_perimeter) / 2 + 1);
}

