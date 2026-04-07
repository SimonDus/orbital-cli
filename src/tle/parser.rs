use crate::cli::PositionArgs;
use crate::domain::Satellite;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use itertools::Itertools;

#[allow(unused)]
pub fn parse_full_tle_file(args: PositionArgs) -> Result<Vec<Satellite>, Error> {
    let tle_file = File::open(&args.tle)?;
    let reader = BufReader::new(tle_file);
    let mut satellites_vec: Vec<Satellite> = Vec::new();

    for chunk in &reader.lines().chunks(3) {
        let lines: Vec<String> = chunk.filter_map(Result::ok).collect();

        if lines.len() == 3 {
            let sat = Satellite {
                name: lines[0].clone().trim().to_string(),
                line_1: lines[1].clone().trim().to_string(),
                line_2: lines[2].clone().trim().to_string(),
            };
            satellites_vec.push(sat);
        }
    }
    Ok(satellites_vec)
}


fn tle_file_reader(args: &PositionArgs) -> Result<BufReader<File>, Error> {
    let tle_file = File::open(&args.tle)?;
    let reader = BufReader::new(tle_file);
    Ok(reader)
}

pub fn parse_tle_file(args: &PositionArgs) -> Result<Vec<Satellite>, String> {
    let reader = tle_file_reader(args).map_err(|e| e.to_string())?;
    let mut satellites_vec: Vec<Satellite> = Vec::new();

    for chunk in &reader.lines().chunks(3) {
        let lines: Vec<String> = chunk.filter_map(Result::ok).collect();

        if lines.len() == 3 && lines[0].trim() == args.name {
            let sat = Satellite {
                name: lines[0].trim().to_string(),
                line_1: lines[1].trim().to_string(),
                line_2: lines[2].trim().to_string(),
            };
            satellites_vec.push(sat);
        }
    }


    if satellites_vec.is_empty() {
        Err(format!("No satellite found with the given name {}", args.name))
    } else {
        Ok(satellites_vec)
    }
}