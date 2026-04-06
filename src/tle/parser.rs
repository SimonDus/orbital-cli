use crate::domain::Satellite;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use itertools::Itertools;

pub fn parse_tle_file(path: &str) -> Result<Vec<Satellite>, Error> {
    let tle_file = File::open(path)?;
    let reader = BufReader::new(tle_file);
    let mut satellites_vec: Vec<Satellite> = Vec::new();

    for chunk in &reader.lines().take(3).chunks(3) {
        let lines: Vec<String> = chunk.filter_map(Result::ok).collect();

        if lines.len() == 3 {
            let sat = Satellite {
                name: lines[0].clone(),
                line_1: lines[1].clone(),
                line_2: lines[2].clone(),
            };
            satellites_vec.push(sat);
        }
    }
    Ok(satellites_vec)
}