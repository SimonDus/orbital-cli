use std::env;
use std::fs;

pub fn parse_tle_file(path: &str) -> Result<Satellite, Box<dyn std::error::Error>> {
    let tle_content = fs::read_to_string("./stations.txt");

}