use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn read_inputs_from_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    return Ok(lines);
}
