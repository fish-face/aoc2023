use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use itertools::Itertools;
use aoc2023::common::read_input_bytes;
use aoc2023::coord::Pt;
use aoc2023::grid::Grid;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Dir { N, E, S, W }

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct State {
    pos: Pt<usize>,
    gone_straight: usize,
    straight_dir: Dir,
}

#[derive(Clone, Eq, PartialEq)]
struct CostedState (State, usize);

impl Ord for CostedState {
    fn cmp(&self, other: &Self) -> Ordering {
        // flipped because Heap is a max-heap
        other.1.cmp(&self.1)
            .then_with(|| other.0.gone_straight.cmp(&self.0.gone_straight))
            .then_with(|| self.0.pos.0.cmp(&other.0.pos.0))
            .then_with(|| self.0.pos.1.cmp(&other.0.pos.1))
    }
}

impl PartialOrd for CostedState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(map: &Grid<u8>, start: Pt<usize>, end: Pt<usize>, min_straight: usize, max_straight: usize) -> usize {
    let start_state = State{pos: start, gone_straight: 0, straight_dir: Dir::E};
    let mut queue = BinaryHeap::<CostedState>::new();
    queue.push(CostedState(start_state, 0));
    let mut best = HashMap::<State, usize>::new();
    best.insert(start_state, 0);

    while let Some(CostedState(state, cost)) = queue.pop() {
        if state.pos == end {
            return cost;
        }

        if cost <= *best.get(&state).unwrap_or(&usize::MAX) {
            best.insert(state, cost);
            for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
                if dir != state.straight_dir && state.gone_straight < min_straight {
                    continue;
                }
                let straight_dist = if dir == state.straight_dir {
                    state.gone_straight + 1
                } else {
                    1
                };
                if straight_dist > max_straight { continue; }

                let next_pos = match dir {
                    Dir::N => {
                        if state.straight_dir == Dir::S || state.pos.1 == 0 { continue; }
                        Pt(state.pos.0, state.pos.1 - 1)
                    }
                    Dir::E => {
                        if state.straight_dir == Dir::W || state.pos.0 == map.width - 1 {continue;}
                        Pt(state.pos.0 + 1, state.pos.1)
                    }
                    Dir::S => {
                        if state.straight_dir == Dir::N || state.pos.1 == map.height - 1 {continue;}
                        Pt(state.pos.0, state.pos.1 + 1)
                    }
                    Dir::W => {
                        if state.straight_dir == Dir::E || state.pos.0 == 0 {continue;}
                        Pt(state.pos.0 - 1, state.pos.1)
                    }
                };

                let next_state = State{pos: next_pos, gone_straight: straight_dist, straight_dir: dir};
                let next_cost = cost + map[next_pos] as usize;

                if next_cost < *best.get(&next_state).unwrap_or(&usize::MAX) {
                    queue.push(CostedState(next_state, next_cost));
                    best.insert(next_state, next_cost);
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
            Some(line.filter(|b| *b != b'\n').map(|b| b - b'0'))
        });
    let grid = Grid::from_row_data(input);

    let part1 = search(&grid, Pt(0, 0), Pt(grid.width - 1, grid.height - 1), 0, 3);
    let part2 = search(&grid, Pt(0, 0), Pt(grid.width - 1, grid.height - 1), 4, 10);
    println!("{part1}\n{part2}");
}