use std::cmp::{max, min};
use itertools::{Itertools};
use aoc2023::common::{read_input_lines, strs_to_nums};

struct RangeMap {
    // Ranges stored as (dst_start, src_start, len)
    storage: Vec<(usize, usize, usize)>,
    min_mapped: usize,
    max_mapped: usize,
}

impl RangeMap {
    fn from_lines(lines: &mut impl Iterator<Item=String>) -> Self {
        let mut result = Self{storage: vec![], max_mapped: 0, min_mapped: usize::MAX};
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            if let Some(tuple) = strs_to_nums(line.split_ascii_whitespace()).collect_tuple() {
                result.storage.push(tuple);
                result.min_mapped = min(tuple.1, result.min_mapped);
                result.max_mapped = max(tuple.1 + tuple.2 - 1, result.max_mapped);
            } else {
                panic!("{} is malformed", line);
            }
        }
        result.storage.sort_by(|(_, source_start1, _), (_, source_start2, _)| source_start1.cmp(source_start2));
        result
    }

    fn apply_to_ranges(&self, ranges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for (start, len) in ranges.iter() {
            let end = start + len - 1;
            let (next_boundary, offset) = self.get_next_boundary(*start);
            let next_boundary = next_boundary.unwrap_or(end);
            if end <= next_boundary {
                result.push(((*start as isize + offset) as usize, *len));
            } else {
                let next_len = next_boundary - start;
                result.push(((*start as isize + offset) as usize, next_len));
                result.append(&mut self.apply_to_ranges(&vec![(next_boundary, len - next_len)]));
            }
        }
        result
    }

    fn get_next_boundary(&self, input: usize) -> (Option<usize>, isize) {
        if input < self.min_mapped {
            // below the first range - next boundary is the first start, and the transformation is identity
            return (Some(self.storage[0].1), 0)
        }

        for (i, (dest_start, source_start, len)) in self.storage.iter().enumerate() {
            if *source_start <= input {
                if input < source_start + len {
                    // input is in one of our ranges - next boundary is the end of it
                    return (Some(*source_start + len), *dest_start as isize - *source_start as isize);
                }
                if i < self.storage.len() - 1 && input < self.storage[i+1].1 {
                    // input is outside any range - next boundary is start of the next, if it
                    // exists, and we're doing no transformation
                    return (Some(self.storage[i+1].1), 0);
                }
            }
        }
        (None, 0)
    }

    fn get(&self, index: usize) -> usize {
        for (dest_start, source_start, len) in self.storage.iter() {
            if source_start <= &index && index < source_start + len {
                return dest_start + index - source_start;
            }
        }
        index
    }
}

fn main() {
    let mut lines = read_input_lines().expect("Could not read input");

    let first_line = lines
        .next()
        .expect("Empty input");
    let seeds = strs_to_nums(
        first_line
            .split_once(' ')
            .unwrap()
            .1
            .split_ascii_whitespace()
    ).collect::<Vec<_>>();

    let mut lines = lines.skip(1);

    // let seeds_soil = RangeMap::from_lines(&mut lines.skip(1));
    // let soil_fertilizer = RangeMap::from_lines(&mut lines.skip(1));
    // let fertilizer_water = RangeMap::from_lines(&mut lines.skip(1));
    // let water_light = RangeMap::from_lines(&mut lines.skip(1));
    // let light_temp = RangeMap::from_lines(&mut lines.skip(1));
    // let temp_humidity = RangeMap::from_lines(&mut lines.skip(1));
    // let humidity_location = RangeMap::from_lines(&mut lines.skip(1));

    let maps = (0..7)
        .map(|_| RangeMap::from_lines(&mut lines.by_ref().skip(1)))
        .collect::<Vec<_>>();

    let locations = seeds.iter().map(|seed| {
        let mut idx = *seed;
        for map in maps.iter() {
            idx = map.get(idx);
        }
        idx
    });
    println!("{}", locations.min().unwrap());
    let seed_range_chunks = seeds
        .iter()
        .chunks(2);
    let seed_ranges = seed_range_chunks
        .into_iter()
        .map(|pair| pair.copied().collect_tuple::<(_, _)>().unwrap());

    let mut idx_ranges = seed_ranges.collect();
    for map in maps.iter() {
        idx_ranges = map.apply_to_ranges(&idx_ranges);
    }
    println!("{}", idx_ranges.iter().map(|(start, _)| start).min().unwrap());
}