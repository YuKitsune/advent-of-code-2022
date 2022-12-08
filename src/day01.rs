pub fn run(input: String) {

    // One blank line (Two new lines together) separates each elf
    let elf_calories = input.split("\n\n");

    let mut max_calories = 0;
    let mut elf_with_max_calories = 0;

    for (i, elf) in elf_calories.enumerate() {
        let total_calories = elf.lines()
            .map(|s| -> i32 { s.parse().unwrap() }) // Convert to i32
            .fold(0, |acc, i| acc + i); // Accumulate (Now THIS is cool!)

        if total_calories > max_calories {
            max_calories = total_calories;
            elf_with_max_calories = i + 1;
        }
    }

    println!("The elf in position {} has the most amount of calories ({})", elf_with_max_calories, max_calories)
}