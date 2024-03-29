use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::iter::Map;
use std::str::FromStr;

use anyhow::Result;

pub fn read_input_lines() -> Result<impl Iterator<Item = String>> {
    Ok(read_file_lines(env::args().nth(1).expect("No input supplied!"))?.filter_map(
        |line| line.ok()
    ))
}

pub fn read_input() -> Result<String> {
    let path = env::args().nth(1).expect("No input supplied!");
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

pub fn read_input_bytes() -> impl Iterator<Item=u8> {
    let path = env::args().nth(1).expect("No input supplied!");
    let file = File::open(path).expect("Could not open file");
    unsafe { BufReader::new(file).bytes().map(|r| r.unwrap_unchecked()) }
}

// pub fn read_input_byte_lines() -> impl Iterator<Item=impl Iterator<Item=u8>> {
//     // let path = env::args().nth(1).expect("No input supplied!");
//     // let file = File::open(path).expect("Could not open file");
//     // let bytes = unsafe { BufReader::new(file)
//     //     .bytes()
//     //     .map(|r| r.unwrap_unchecked())
//     // };
//     read_input_bytes()
//         .group_by(|b| *b == b'\n')
//         .into_iter()
//         .map(|(_, line)| line
//             .filter(|b| *b != b'\n')
//         )
// }

pub fn parse_input_lines<T>() -> Result<impl Iterator<Item = T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug
{
    Ok(read_input_lines()?.map(|line|
        T::from_str(&line).expect("Error while parsing line")
    ))
}

fn read_file_lines(path: String) -> Result<impl Iterator<Item = Result<String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().map(|line| line.map_err(anyhow::Error::from)))
    // let mut content = String::new();
    // file.read_to_string(&mut content)?;

    // Ok(content.split('\n').map(|s| s.to_string()))
}

pub fn strs_to_nums<T, S>(strs: T) -> Map<<T as IntoIterator>::IntoIter, impl FnMut(T::Item) -> S>
where
    T: IntoIterator,
    T::Item: Into<String>,
    S: FromStr,
    S::Err: Debug,
{
    strs.into_iter().map(|s: T::Item| s.into().parse::<S>().expect("Couldn't parse "))
}