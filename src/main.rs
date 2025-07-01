mod cli;
mod commands;

use clap::Parser;
use cli::*;
use commands::*;

fn main() {
    let cli = Cli::parse();
     
     match &cli.command {
        Commands::Calc(args) => calc::run(args),
     }
}
