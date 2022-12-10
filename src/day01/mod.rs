pub fn run_part1(input: String) {
    let top_three_elves = get_top_three_elves(input);
    let top_elf = top_three_elves[0];
    println!("{}", top_elf);
}

pub fn run_part2(input: String) {
    let top_three_elves = get_top_three_elves(input);
    let total: i32 = top_three_elves.clone().into_iter().sum();
    println!("{}", total)
}

fn get_top_three_elves(input: String) -> Vec<i32> {

    // One blank line (Two new lines together) separates each elf
    let elf_calories_strs = input.split("\n\n");
    let mut total_calories = Vec::<i32>::new();

    for elf_calories_str in elf_calories_strs {
        let total = elf_calories_str.lines()
            .map(|s| -> i32 { s.parse().unwrap() }) // Convert to i32
            .sum();

        total_calories.push(total)
    }

    total_calories.sort();
    let top_three_elves = total_calories.into_iter().rev().take(3).collect();
    return top_three_elves
}