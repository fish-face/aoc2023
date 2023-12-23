use std::cmp::{max, min};
use std::collections::VecDeque;
use std::io::BufRead;
use bit_set::BitSet;
use itertools::{iproduct, Itertools};
use aoc2023::common::{read_input_lines, strs_to_nums};

#[derive(Debug, Clone)]
struct Pt (usize, usize, usize);
#[derive(Debug, Clone)]
struct Brick (Pt, Pt);

fn coord(p: &Pt) -> usize {
    p.0 * 10 * 400 + p.1 * 400 + p.2
}

#[inline]
fn dumbpt(p: (usize, usize, usize)) -> Pt {
    Pt(p.0, p.1, p.2)
}

fn brick_pts(brick: &Brick) -> impl Iterator<Item=Pt> {
    let Brick(start, end) = brick;
    iproduct!(
        min(start.0, end.0)..=max(start.0, end.0),
        min(start.1, end.1)..=max(start.1, end.1),
        min(start.2, end.2)..=max(start.2, end.2)
    ).map(|(x, y, z)| Pt(x, y, z))
}

fn chr(v: usize) -> char {
    let chars = "1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"£$%^&*()".as_bytes();
    if v == 0 {
        ' '
    } else if v >= chars.len() {
        '>'
    } else {
        chars[v - 1] as char
    }
}

fn print_stack(stack: &Vec<usize>, height: usize) {
    for z in (1..height+1).rev() {
        for y in 0..10 {
            for x in 0..10 {
                print!("{}", chr(stack[coord(&Pt(x, y, z))]))
            }
            print!("|");
        }
        print!("\n");
    }
    print!("\n");
}

fn drop(stack: &mut Vec<usize>, bricks: &mut Vec<Brick>, height: usize) -> bool {
    let mut fallen = false;

    for (brick_idx, brick) in bricks.iter_mut().enumerate() {
        let mut blocked = false;
        for Pt(x, y, z) in brick_pts(brick) {
            let below_p = Pt(x, y, z - 1);
            // are we on the floor, or is the space below not-empty and not-me?
            if below_p.2 == 0 || (
                stack[coord(&below_p)] != 0 && stack[coord(&below_p)] != brick_idx + 1
            ) {
                blocked = true;
                break;
            }
        }

        if !blocked {
            for Pt(x, y, z) in brick_pts(brick) {
                let p = Pt(x, y, z);
                let below_p = Pt(x, y, z - 1);
                stack[coord(&p)] = 0;
                stack[coord(&below_p)] = brick_idx + 1;
            }

            brick.0.2 -= 1;
            brick.1.2 -= 1;
            fallen = true;
        }
    }

    fallen
}

// static mut iters: usize = 0;

fn count_falling(brick_idx: usize, supports: &mut Vec<BitSet>, supported_by: &mut Vec<BitSet>) -> usize {
    let mut visited = BitSet::with_capacity(1400);
    let mut queue = VecDeque::with_capacity(1400);
    let mut falling = BitSet::new();
    falling.insert(brick_idx);
    queue.push_back(brick_idx);

    while let Some(current) = queue.pop_front() {
        if visited.contains(current) {
            continue;
        }
        // unsafe { iters += 1 };
        visited.insert(current);
        for next_brick_idx in supports[current].iter() {
            if supported_by[next_brick_idx].len() > 0 && supported_by[next_brick_idx].difference(&falling).count() == 0 {
                falling.insert(next_brick_idx);
                queue.push_back(next_brick_idx);
            }
        }
    }
    falling.len() - 1
}

fn main() {
    let input = read_input_lines().unwrap();
    let mut bricks = Vec::<Brick>::with_capacity(1400);

    let mut height = 0;
    let mut stack = vec![0; 10 * 10 * 400];

    for (i, line) in input.enumerate() {
        let (start, end) = line.split_once('~').unwrap();
        let start = dumbpt(strs_to_nums(start.split(',')).collect_tuple().unwrap());
        let end = dumbpt(strs_to_nums(end.split(',')).collect_tuple().unwrap());

        height = max(height, max(start.2, end.2));
        let brick = Brick(start, end);

        for p in brick_pts(&brick) {
            stack[coord(&p)] = i + 1;
        }
        bricks.push(brick);

    }

    // STEP 1: drop blocks
    while drop(&mut stack, &mut bricks, height) {}

    // relabel bricks in final height order
    bricks.sort_unstable_by(
        |a, b| min(a.0.2, a.1.2).cmp(min(&b.0.2, &b.1.2))
    );

    let mut stack = vec![0; 10 * 10 * 400];
    for (brick_idx, brick) in bricks.iter().enumerate() {
        for p in brick_pts(&brick) {
            stack[coord(&p)] = brick_idx + 1;
        }
    }
    // print_stack(&stack, height);

    // STEP 2: build digraph of supports

    // brick --> bricks it supports
    let mut supports = vec![BitSet::new(); bricks.len() + 1];
    // brick --> bricks it's supported by
    let mut supported_by = vec![BitSet::new(); bricks.len() + 1];

    for (brick_idx, brick) in bricks.iter().enumerate() {
        for Pt(x, y, z) in brick_pts(brick) {
            let below_p = Pt(x, y, z - 1);
            // are we on the floor, or is the space below not-empty and not-me?
            if below_p.2 > 0 &&
                stack[coord(&below_p)] != 0 &&
                stack[coord(&below_p)] != brick_idx + 1
            {
                let below_brick = stack[coord(&below_p)];
                supports[below_brick].insert(brick_idx + 1);
                supported_by[brick_idx + 1].insert(below_brick);
            }
        }
    }

    // STEP 3: traverse graph
    let falling = (1..bricks.len() + 1)
        .map(|i| count_falling(i, &mut supports, &mut supported_by))
        .collect::<Vec<_>>();

    // unsafe { println!("{iters} iters"); }
    println!("{}\n{}", falling.iter().filter(|n| **n == 0).count(), falling.into_iter().sum::<usize>());
}