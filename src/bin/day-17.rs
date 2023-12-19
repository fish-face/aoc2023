use std::cmp::{min, Ordering};
use std::collections::BinaryHeap;
use itertools::Itertools;
use aoc2023::common::read_input_bytes;
use aoc2023::coord::Pt;
use aoc2023::grid::Grid;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Dir { N, E, S, W }

#[inline]
fn flip(d: Dir) -> Dir {
    match d {
        Dir::N => Dir::S,
        Dir::E => Dir::W,
        Dir::S => Dir::N,
        Dir::W => Dir::E,
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct State {
    pos: Pt<usize>,
    straight_dir: Dir,
}

impl State {
    fn idx(&self, width: usize, height: usize) -> usize {
        self.pos.0 + self.pos.1 * width + (self.straight_dir as usize) * width * height
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct CostedState(State, u16);

impl Ord for CostedState {
    fn cmp(&self, other: &Self) -> Ordering {
        // flipped because Heap is a max-heap
        other.1.cmp(&self.1)
            .then_with(|| self.0.pos.0.cmp(&other.0.pos.0))
            .then_with(|| self.0.pos.1.cmp(&other.0.pos.1))
    }
}

impl PartialOrd for CostedState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(map: &Grid<u16>, start: Pt<usize>, end: Pt<usize>, min_straight: usize, max_straight: usize) -> u16 {
    let mut queue = BinaryHeap::<CostedState>::new();
    let mut best = vec![None; map.width + map.width * map.height + map.width * map.height * max_straight + map.width * map.height * max_straight * 4];

    let start_state = State { pos: start, straight_dir: Dir::E };
    queue.push(CostedState(start_state, 0));
    best[start_state.idx(map.width, map.height)] = Some(0);

    let start_state = State { pos: start, straight_dir: Dir::S };
    queue.push(CostedState(start_state, 0));
    best[start_state.idx(map.width, map.height)] = Some(0);

    // let mut prev_map = std::collections::HashMap::<State, State>::new();

    while let Some(CostedState(state, cost)) = queue.pop() {
        if state.pos == end {
            // let mut debugmap = Grid::new(map.width, map.height);
            // let mut debugstate = state;
            // loop {
            //     let opt = prev_map.get(&debugstate);
            //     if opt.is_none() {break;}
            //     debugstate = *opt.unwrap();
            //     debugmap[debugstate.pos] = true;
            // }
            // println!("{}\n", debugmap.map(|x| if *x {'#'} else {'.'}).to_string(Some("")));
            return cost;
        }

        for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
            if dir == state.straight_dir || dir == flip(state.straight_dir) {
                continue;
            }

            let max_movement = match dir {
                Dir::N => state.pos.1,
                Dir::E => map.width - state.pos.0 - 1,
                Dir::S => map.height - state.pos.1 - 1,
                Dir::W => state.pos.0,
            };
            if max_movement < min_straight {
                continue;
            }

            let mut next_cost = cost;
            for dist in 1..=min(max_movement, max_straight) {
                let next_pos = match dir {
                    Dir::N => Pt(state.pos.0, state.pos.1 - dist),
                    Dir::E => Pt(state.pos.0 + dist, state.pos.1),
                    Dir::S => Pt(state.pos.0, state.pos.1 + dist),
                    Dir::W => Pt(state.pos.0 - dist, state.pos.1),
                };
                assert!(map.contains(next_pos));

                next_cost += map[next_pos];

                if dist < min_straight { continue; }

                let next_state = State { pos: next_pos, straight_dir: dir };

                if next_cost < best[next_state.idx(map.width, map.height)].unwrap_or(u16::MAX) {
                    // println!("{:?}", CostedState(next_state, next_cost));
                    queue.push(CostedState(next_state, next_cost));
                    best[next_state.idx(map.width, map.height)] = Some(next_cost);
                    // prev_map.insert(next_state, state);
                }
            }
        }
    }
    0
}

fn main() {
    let input = read_input_bytes()
        .group_by(|b| *b == b'\n');
    let input = input
        .into_iter()
        .filter_map(|(newline, line)| if newline {
            None
        } else {
            Some(line.filter(|b| *b != b'\n').map(|b| (b - b'0') as u16))
        });
    let grid = Grid::from_row_data(input);

    let part1 = search(&grid, Pt(0, 0), Pt(grid.width - 1, grid.height - 1), 0, 3);
    let part2 = search(&grid, Pt(0, 0), Pt(grid.width - 1, grid.height - 1), 4, 10);
    println!("{part1}\n{part2}");
}