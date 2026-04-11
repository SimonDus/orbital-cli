use crate::cli::PositionArgs;
use crate::domain::ground_station::GroundStation;
use crate::tle::parser::parse_tle_file;

pub fn run_position(args: &PositionArgs) {
    //println!("{:?}", parse_tle_file(args));

    match parse_tle_file(args) {
        Ok(satellites) => {
            println!("Found satellite(s):");
            for sat in satellites {
                println!("- {}", sat.name);
                println!("  line 1: {}", sat.line_1);
                println!("  line 2: {}", sat.line_2);
            }
            let gs: GroundStation = GroundStation {
                latitude: args.lat,
                longitude: args.lon,
                altitude: args.alt,
            };

            println!("{:?}", gs);
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
