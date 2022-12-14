use std::collections::HashMap;

pub fn run_part1(input: String) {

    let sum: i32 = input.lines()
        .map(|line: &str| -> (&str, &str) {
            let split = line.len() / 2;
            return (&line[0..split], &line[split..line.len()])
        })
        .map(|line: (&str, &str)| -> char {
            let (c1, c2) = line;
            return find_shared_character(vec!(c1, c2)).unwrap()
        })
        .map(|ch: char| -> i32 { get_priority(ch).unwrap() })
        .sum();

    println!("{}", sum)
}

pub fn run_part2(input: String) {

    let mut total_priority = 0;
    let lines = input.lines();
    let total_lines = lines.clone().count();
    for i in (0..total_lines).step_by(3) {
        let group_lines = lines.clone().skip(i).take(3).collect();
        let shared_character = find_shared_character(group_lines).unwrap();
        let priority = get_priority(shared_character).unwrap();
        total_priority += priority
    }

    println!("{}", total_priority)
}

fn find_shared_character(strings: Vec<&str>) -> Option<char> {
    let mut map = HashMap::<char, usize>::new();

    for s in &strings {
        let mut current_str_map = HashMap::<char, usize>::new();
        for c in s.chars() {
            if !current_str_map.contains_key(&c){
                *map.entry(c).or_default() += 1
            }

            *current_str_map.entry(c).or_default() += 1;
        }
    }

    for (character, count) in map {
        if count == strings.len() {
            return Some(character);
        }
    }

    return None
}

fn get_priority(c: char) -> Option<i32> {
    match c {
        'a'..='z' => Some((c as u8 - 'a' as u8 + 1) as i32),
        'A'..='Z' => Some((c as u8 - 'A' as u8 + 27) as i32),
        _ => None,
    }
}