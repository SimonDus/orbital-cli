use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "sat-track")]
#[command(about = "Orbital CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Position(PositionArgs),
}

#[derive(Args, Debug)]
pub struct PositionArgs {
    #[arg(long)]
    pub tle: String,

    #[arg(long)]
    pub lat: f64,

    #[arg(long)]
    pub lon: f64,

    #[arg(long)]
    pub alt: f64,
}

pub fn parse() -> Cli {
    Cli::parse()
}
