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

// I'm lazy
fn get_priority(c: char) -> Option<i32> {
    match c {
        'a' => Some(1),
        'b' => Some(2),
        'c' => Some(3),
        'd' => Some(4),
        'e' => Some(5),
        'f' => Some(6),
        'g' => Some(7),
        'h' => Some(8),
        'i' => Some(9),
        'j' => Some(10),
        'k' => Some(11),
        'l' => Some(12),
        'm' => Some(13),
        'n' => Some(14),
        'o' => Some(15),
        'p' => Some(16),
        'q' => Some(17),
        'r' => Some(18),
        's' => Some(19),
        't' => Some(20),
        'u' => Some(21),
        'v' => Some(22),
        'w' => Some(23),
        'x' => Some(24),
        'y' => Some(25),
        'z' => Some(26),
        'A' => Some(27),
        'B' => Some(28),
        'C' => Some(29),
        'D' => Some(30),
        'E' => Some(31),
        'F' => Some(32),
        'G' => Some(33),
        'H' => Some(34),
        'I' => Some(35),
        'J' => Some(36),
        'K' => Some(37),
        'L' => Some(38),
        'M' => Some(39),
        'N' => Some(40),
        'O' => Some(41),
        'P' => Some(42),
        'Q' => Some(43),
        'R' => Some(44),
        'S' => Some(45),
        'T' => Some(46),
        'U' => Some(47),
        'V' => Some(48),
        'W' => Some(49),
        'X' => Some(50),
        'Y' => Some(51),
        'Z' => Some(52),
        _ => None
    }
}