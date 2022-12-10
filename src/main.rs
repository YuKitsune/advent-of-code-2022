mod day01;
mod day02;

use std::fs;
use std::io::{Read, stdin};

use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "advent-of-code-2022")]
#[command(bin_name = "advent-of-code-2022")]
#[command(about = "Advent of Code 2022", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Day01(Part),
    Day02(Part),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Part {
    #[command(subcommand)]
    command: PartCommands,
}

#[derive(Debug, Subcommand)]
#[command(args_conflicts_with_subcommands = true)]
enum PartCommands {
    Part1(Arguments),
    Part2(Arguments)
}

#[derive(Debug, Args)]
struct Arguments {
    #[arg(short, long)]
    input: Option<String>,
}

trait HasInput {
    fn get_input(&self) -> Result<String, Box<dyn std::error::Error>>;
}

impl HasInput for Arguments {
    fn get_input(&self) -> Result<String, Box<dyn std::error::Error>> {

        // Try reading from stdin first
        if atty::isnt(atty::Stream::Stdin) {
            let mut buffer = String::new();
            stdin().lock().read_to_string(&mut buffer).unwrap();

            return Ok(buffer)
        }

        // Otherwise, try using the --input flag
        if let Some(input_path) = &self.input {
            let input = fs::read_to_string(input_path).expect("Couldn't find input file");
            return Ok(input)
        }

        return Err(Box::from("input was not provided via the --input argument or stdin"))
    }
}


fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Day01(part) => {
            match part.command {
                PartCommands::Part1(args) => {
                    let input = args.get_input().unwrap();
                    day01::run_part1(input)
                },
                PartCommands::Part2(args) => {
                    let input = args.get_input().unwrap();
                    day01::run_part2(input)
                }
            }
        }
        Commands::Day02(part) => {
            match part.command {
                PartCommands::Part1(args) => {
                    let input = args.get_input().unwrap();
                    day02::run_part1(input)
                },
                PartCommands::Part2(args) => {
                    let input = args.get_input().unwrap();
                    day02::run_part2(input)
                }
            }
        }
    }
}
