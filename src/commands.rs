use crate::cli::PositionArgs;
use crate::domain::ground_station::GroundStation;
use crate::domain::position_request::PositionRequest;
use crate::tle::parser::parse_tle_file;
use chrono::{Utc};

pub fn run_position(args: &PositionArgs) {
    match parse_tle_file(args) {
        Ok(satellites) => {
            let gs: GroundStation = GroundStation {
                latitude: args.lat,
                longitude: args.lon,
                altitude: args.alt,
            };

            let pos_request: PositionRequest = PositionRequest {
                satellites,
                ground_station: gs,
                observation_time: Utc::now(),
            };

            println!("{:?}", pos_request);
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
