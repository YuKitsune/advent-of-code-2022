mod day01;

use std::{fs, io};
use std::io::Read;

use clap::{arg, ArgMatches, Command};

fn cli() -> Command {
    Command::new("aoc")
        .about("Advent of Code 2022")
        .subcommand_required(true)
        .subcommand(
            Command::new("day01")
                .about("Day 1: Calorie Counting")
                .arg(arg!(-i --input "The path to the input data.")),
        )
}

fn get_input(sub_matches: &ArgMatches) -> String {

    // Default to stdin
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    // Todo: This locks up if nothing is in stdin
    if let Ok(buffer_size) = stdin.read_to_string(&mut buffer) {
        if buffer_size > 0 {
            return buffer
        }
    }

    if let Some(input_path) = sub_matches.get_one::<String>("input") {
        let input = fs::read_to_string(input_path).expect("Couldn't find input file");
        return input
    }

    panic!("Missing input")
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("day01", sub_matches)) => {
            let input = get_input(sub_matches);
            day01::run(input)
        }

        // If all subcommands are defined above, anything else is unreachabe!()
        _ => unreachable!(),
    }
}

