use aoc2023::common::read_input_lines;

fn main() {
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
        }
    }

    let mut part1 = 0;
    let mut part2 = 0_usize;

    let mut idx = galaxies_x[0];
    let mut prev_x = 0;
    for (x, n) in galaxies_x[1..].iter().enumerate().filter(|(_, n)| **n > 0) {
        let x = x + 1;
        let skips = x - prev_x - 1;

        let multiple = (galaxies - idx) * idx;
        part1 += (1 + skips * 2) * multiple;
        part2 += (1 + skips * 1_000_000) * multiple;

        idx += n;
        prev_x = x;
    }

    let mut idx = galaxies_y[0];
    let mut prev_y = 0;
    for (y, n) in galaxies_y[1..].iter().enumerate().filter(|(_, n)| **n > 0) {
        let y = y + 1;
        let skips = y - prev_y - 1;

        let multiple = (galaxies - idx) * idx;
        part1 += (1 + skips * 2) * multiple;
        part2 += (1 + skips * 1_000_000) * multiple;

        idx += n;
        prev_y = y;
    }

    println!("{}\n{}", part1, part2);
}