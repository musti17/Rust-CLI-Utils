use clap::{Parser, Subcommand};
use crate::commands::calc;

#[derive(Parser)]
#[command(name = "rusty-toolbox")]
#[command(about = "A CLI toolbox written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands{
    Calc(calc::CalcArgs)
    //add more commands as you deem feasible
}
