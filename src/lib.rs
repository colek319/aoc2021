use std::{
    env,
};

pub fn get_aoc_input(day: &str) -> String {
    let filename = get_aoc_input_filename(day);
    std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}

pub fn get_aoc_input_filename(day: &str) -> String {
    let rootdir = env!("CARGO_MANIFEST_DIR");
    let infile = format!("{}/inputs/{}.input", rootdir, day);
    return infile;
}

pub fn to_i32(s: &str) -> i32 {
    return s.parse::<i32>().unwrap();
}
