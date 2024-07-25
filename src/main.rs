use aoc::{commands::Runnable, days};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Advent of Code 2023")]
#[command(about = "My AoC 2023 solution, while learning Rust")]
#[command(version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day01(days::day_01::Command),
}

impl Runnable for Commands {
    fn run(&self) {
        match self {
            Commands::Day01(cmd) => cmd.run(),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    cli.command.run();
}
