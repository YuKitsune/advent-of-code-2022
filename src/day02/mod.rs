use std::collections::HashMap;
use std::iter::Map;

trait HasScore {
    fn to_score(&self) -> i32;
}

enum Shape {
    Rock,
    Paper,
    Scissors
}

impl HasScore for Shape {
    fn to_score(&self) -> i32 {
        return match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum Result {
    Win(Shape),
    Lose(Shape),
    Draw(Shape)
}

impl HasScore for Result {
    fn to_score(&self) -> i32 {
        return match &self {
            Result::Win(shape) => shape.to_score() + 6,
            Result::Lose(shape) => shape.to_score() + 0,
            Result::Draw(shape) => shape.to_score() + 3,
        }
    }
}

pub fn run(input: String) {

    let score: i32 = input.lines()
        .map(|l| {
            let split: Vec<&str> = l.split(" ").collect();
            return (to_shape(split[0]), to_shape(split[1]))
        })
        .map(|r| to_result(r))
        .map(|r| r.to_score())
        .sum();

    println!("Total score: {}", score);
}

fn to_shape(value: &str) -> Shape {
    return match value {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("unexpected value: {}", value)
    }
}

fn to_result(round: (Shape, Shape)) -> Result {
    return match round {
        (Shape::Rock, Shape::Rock) => Result::Draw(Shape::Rock),
        (Shape::Rock, Shape::Paper) => Result::Win(Shape::Paper),
        (Shape::Rock, Shape::Scissors) => Result::Lose(Shape::Scissors),

        (Shape::Paper, Shape::Rock) => Result::Lose(Shape::Rock),
        (Shape::Paper, Shape::Paper) => Result::Draw(Shape::Paper),
        (Shape::Paper, Shape::Scissors) => Result::Win(Shape::Scissors),

        (Shape::Scissors, Shape::Rock) => Result::Win(Shape::Rock),
        (Shape::Scissors, Shape::Paper) => Result::Lose(Shape::Paper),
        (Shape::Scissors, Shape::Scissors) => Result::Draw(Shape::Scissors),
    }
}