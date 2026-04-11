mod cli;
mod commands;
mod domain;
mod tle;
use commands::run_position;

fn main() {
    let cli = cli::parse();

    match &cli.command {
        cli::Commands::Position(args) => {
            run_position(args);
        }
    }
}
