use aoc2023::common::read_input_lines;

fn main() {
    let mut width = 0;
    let mut height = 0;

    let mut galaxies = 0;
    let mut galaxies_x = vec![0; 140];
    let mut galaxies_y = vec![0; 140];
    for (y, line) in read_input_lines().unwrap().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                galaxies += 1;
                galaxies_x[x] += 1;
                galaxies_y[y] += 1;
            }
            if height == 0 {
                width += 1;
            }
        }
        height += 1;
    }

    let mut idx = 0;
    let mut dist = 0;
    let mut skips = 0;
    let mut part1 = 0;
    let mut part2 = 0_usize;

    for x in 0..width {
        let n = galaxies_x[x];
        if n == 0 {
            skips += 1;
        } else {
            let multiple = (galaxies - idx) * idx;
            part1 += (dist + skips) * multiple;
            part2 += (dist + skips * (1_000_000 - 1)) * multiple;
            dist = 0;
            skips = 0;
            idx += n;
        }
        dist += 1;
    }

    idx = 0;
    dist = 0;
    skips = 0;

    for y in 0..height {
        let n = galaxies_y[y];
        if n == 0 {
            skips += 1;
        } else {
            let multiple = (galaxies - idx) * idx;
            part1 += (dist + skips) * multiple;
            part2 += (dist + skips * (1_000_000 - 1)) * multiple;
            dist = 0;
            skips = 0;
            idx += n;
        }
        dist += 1;
    }

    println!("{}\n{}", part1, part2);
}