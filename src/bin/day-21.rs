use num::{Integer, pow};
use aoc2023::common::read_input_lines;
use aoc2023::coord::{PointSet, Pt};

fn spread(
    map: &PointSet<usize>,
    even: bool,
    frontier: &mut PointSet<usize>,
    reachable_odd: &mut PointSet<usize>,
    reachable_even: &mut PointSet<usize>
) {
    let mut next_reachable = PointSet::new(map.width());

    for p in frontier.iter() {
        for neighbour in p.neighbours4() {
            if p.0 < map.width() && p.1 < map.width() && !map.contains(neighbour) {
                next_reachable.insert(neighbour)
            }
        }
    }

    next_reachable.storage.difference_with(&reachable_odd.storage);
    next_reachable.storage.difference_with(&reachable_even.storage);
    *frontier = next_reachable;
    if even {
        reachable_even.storage.union_with(&frontier.storage);
    } else {
        reachable_odd.storage.union_with(&frontier.storage);
    }
}

fn simulate_twice(map: &PointSet<usize>, initial: &mut PointSet<usize>, iters_a: usize, iters_b: usize) -> usize {
    let mut even_reachable = PointSet::new(map.width());
    let mut odd_reachable = PointSet::new(map.width());
    for i in 0..iters_a {
        spread(&map, i % 2 == 0, initial, &mut even_reachable, &mut odd_reachable);
    }
    let to_use = if iters_a % 2 == 0 {
        &even_reachable
    } else {
        &odd_reachable
    };
    let count_a = to_use.storage.iter().count();

    for i in 0..(iters_b - iters_a) {
        spread(&map, (iters_a + i) % 2 == 0, initial, &mut even_reachable, &mut odd_reachable);
    }

    *initial = if iters_b % 2 == 0 {
        even_reachable
    } else {
        odd_reachable
    };

    count_a
}

fn count_rect(large: &PointSet<usize>, left: usize, top: usize, size: usize) -> usize {
    let mut acc = 0;
    for y in top..top + size {
        for x in left..left + size {
            acc += large.contains(Pt(x, y)) as usize;
        }
    }
    acc
}

fn main() {
    let input = read_input_lines().unwrap().collect::<Vec<_>>();
    let tile_width = input[0].len();

    let mut tiled_map = PointSet::new(tile_width * 5);
    let mut start = Pt(0, 0);

    for (y, line) in input.into_iter().enumerate() {
        for (x, c) in line.as_bytes().into_iter().enumerate() {
            if *c == b'#' {
                for ty in 0..5 {
                    for tx in 0..5 {
                        tiled_map.insert(Pt(tx * tile_width + x, ty * tile_width + y));
                    }
                }
            } else if *c == b'S' {
                start = Pt(tile_width * 2 + x, tile_width * 2 + y);
            }
        }
    }

    const part2_target : usize = 26501365;
    let part1_target = 64;
    let (full_iters, remaining_iters) = part2_target.div_mod_floor(&tile_width);

    let mut reachable = PointSet::new(tile_width * 5);
    reachable.insert(start);
    let part1 = simulate_twice(&tiled_map, &mut reachable, part1_target, tile_width * 2 + remaining_iters);
    println!("{}", part1);


    let full_even_per_tile = count_rect(&reachable, tile_width * 2, tile_width * 2, tile_width);
    let full_odd_per_tile = count_rect(&reachable, tile_width * 2, tile_width * 1, tile_width);

    // on parity = 0, we are filling corner tiles of the same parity as the origin
    let tile_parity = full_iters % 2;
    // call "even" tiles those aligned with the origin tile
    let (full_even, full_odd) = if tile_parity == 0 {
        (pow(full_iters - 1, 2), pow(full_iters, 2))
    } else {
        (pow(full_iters, 2), pow(full_iters - 1, 2))
    };

    // println!("tile parity: {tile_parity}. full even tiles: {full_even}, full odd tiles {full_odd}");
    let full_odd_count = full_even * full_even_per_tile;
    let full_even_count = full_odd * full_odd_per_tile;
    // println!(
    //     "fully covered after {} ({} edge-to-edges): {} tiles; {} area",
    //     TARGET, full_iters, full_odd + full_even, full_even_count + full_odd_count
    // );
    // println!("remaining iterations: {}; approx tiles: {}",
    //          remaining_iters, 4 * remaining_iters * full_tile_occupied);
    // println!("approx total: {}", full_odd_count + 4 * remaining_iters * full_tile_occupied / 2);

    // println!("Full tile: {full_even}*{full_even_per_tile}, odd: {full_odd}*{full_odd_per_tile}");

    // we will have 4 * each of these (but each one of the four will be a different fill pattern)
    let low_perimeters = if tile_parity == 0 {
        full_iters - 1
    } else {
        full_iters
    };
    let high_perimeters = if tile_parity == 0 {
        full_iters
    } else {
        full_iters - 1
    };

    let even_ul_per_tile = count_rect(&reachable, tile_width, tile_width, tile_width);
    let odd_ul_per_tile = count_rect(&reachable, 0, tile_width, tile_width);

    let even_ur_per_tile = count_rect(&reachable, 3 * tile_width, tile_width, tile_width);
    let odd_ur_per_tile = count_rect(&reachable, 4 * tile_width, tile_width, tile_width);

    let even_br_per_tile = count_rect(&reachable, 3 * tile_width, 3 * tile_width, tile_width);
    let odd_br_per_tile = count_rect(&reachable, 4 * tile_width, 3 * tile_width, tile_width);

    let even_bl_per_tile = count_rect(&reachable, tile_width, 3 * tile_width, tile_width);
    let odd_bl_per_tile = count_rect(&reachable, 0, 3 * tile_width, tile_width);

    let even_l = count_rect(&reachable, 0, 2 * tile_width, tile_width);
    // let odd_l = count_rect(&reachable, 0, 2 * tile_width, tile_width);

    let even_u = count_rect(&reachable, 2 * tile_width, 0, tile_width);
    // let odd_l = count_rect(&reachable, 0, 2 * tile_width, tile_width);

    let even_r = count_rect(&reachable, 4 * tile_width, 2 * tile_width, tile_width);
    // let odd_l = count_rect(&reachable, 0, 2 * tile_width, tile_width);

    let even_b = count_rect(&reachable, 2 * tile_width, 4 * tile_width, tile_width);
    // let odd_l = count_rect(&reachable, 0, 2 * tile_width, tile_width);

    println!(
        "{}",
        low_perimeters * (even_ur_per_tile + even_ul_per_tile + even_bl_per_tile + even_br_per_tile) +
            high_perimeters * (odd_ul_per_tile + odd_ur_per_tile + odd_bl_per_tile + odd_br_per_tile) +
            even_l + even_u + even_r + even_b +
            full_odd_count + full_even_count
    );

    return;
}