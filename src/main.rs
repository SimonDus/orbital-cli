mod cli;
pub mod domain;
mod tle;
use tle::parser::parse_tle_file;


fn main() {
    let cli = cli::parse();

    match &cli.command {
        cli::Commands::Position(args) => {
            println!("from main: {:?}", parse_tle_file(args));
        }
    }
}
