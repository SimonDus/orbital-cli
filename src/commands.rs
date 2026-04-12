use crate::{cli::PositionArgs};
use crate::domain::ground_station::GroundStation;
use crate::domain::position_request::PositionRequest;
use crate::tle::parser::parse_tle_file;
use crate::domain::selected_satellite_request::SelectedSatelliteRequest;
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



            for sat in &pos_request.satellites{
                let selected = SelectedSatelliteRequest {
                    satellite: sat.clone(),
                    ground_station: pos_request.ground_station.clone(),
                    observation_time: pos_request.observation_time.clone()

                };

                println!("{:?}", &selected)
            }
            
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
