pub fn run(input: String) {

    // One blank line (Two new lines together) separates each elf
    let elf_calories_strs = input.split("\n\n");
    let mut total_calories = Vec::<i32>::new();

    for elf_calories_str in elf_calories_strs {
        let total = elf_calories_str.lines()
            .map(|s| -> i32 { s.parse().unwrap() }) // Convert to i32
            .fold(0, |acc, c| acc + c); // Accumulate (Now THIS is cool!)

        total_calories.push(total)
    }

    total_calories.sort();
    let top_three_elves = total_calories.iter().rev().take(3);

    println!("Top three elves: ");
    for (i, calories) in top_three_elves.clone().enumerate() {
        println!("{}: {}", i + 1, calories);
    }

    let total_of_top_three: i32 = top_three_elves.clone().into_iter().sum();
    println!("Total: {}", total_of_top_three)
}