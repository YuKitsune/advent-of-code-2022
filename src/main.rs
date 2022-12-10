mod day01;
mod day02;

use std::fs;
use std::io::{Read, stdin};

use clap::{arg, ArgMatches, Command};

fn cli() -> Command {
    Command::new("advent-of-code-2022")
        .about("Advent of Code 2022")
        .subcommand_required(true)
        .subcommand(
            Command::new("day01")
                .about("Day 1: Calorie Counting")
                .arg(arg!(-i --input <PATH> "The path to the input data.")),
        )
        .subcommand(
            Command::new("day02")
                .about("Day 1: Rock Paper Scissors")
                .subcommand(
                    Command::new("part1")
                        .arg(arg!(-i --input <PATH> "The path to the input data."))
                    .about("Part 1"))
                .subcommand(
                    Command::new("part2")
                        .arg(arg!(-i --input <PATH> "The path to the input data."))
                        .about("Part 2"))
                .subcommand_required(true)
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("day01", sub_matches)) => {
            let input = get_input(sub_matches).unwrap();
            day01::run(input)
        }

        Some(("day02", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("part1", sub_matches)) => {
                    let input = get_input(sub_matches).unwrap();
                    day02::run_part1(input)
                },
                Some(("part2", sub_matches)) => {
                    let input = get_input(sub_matches).unwrap();
                    day02::run_part2(input)
                },
                _ => unreachable!(),
            }
        }

        // If all subcommands are defined above, anything else is unreachabe!()
        _ => unreachable!(),
    }
}

fn get_input(sub_matches: &ArgMatches) -> Result<String, Box<dyn std::error::Error>> {

    // Try reading from stdin first
    if atty::isnt(atty::Stream::Stdin) {
        let mut buffer = String::new();
        stdin().lock().read_to_string(&mut buffer).unwrap();

        return Ok(buffer)
    }

    // Otherwise, try using the --input flag
    if let Some(input_path) = sub_matches.get_one::<String>("input") {
        let input = fs::read_to_string(input_path).expect("Couldn't find input file");
        return Ok(input)
    }

    return Err(Box::from("input was not provided via the --input argument or stdin"))
}