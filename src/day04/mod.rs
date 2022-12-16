
struct Range {
    min: i32,
    max: i32
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        return self.max >= other.max && self.min <= other.min;
    }

    fn overlaps_with(&self, other: &Range) -> bool {
        return self.min <= other.max && other.min <= self.max
    }
}

trait IntoRange {
    fn into_range(self) -> Range;
}

impl IntoRange for &str {
    fn into_range(self) -> Range {
        let (start, end) = self.split_once("-").unwrap();
        return Range {
            min: start.parse::<i32>().unwrap(),
            max: end.parse::<i32>().unwrap()
        }
    }
}

pub fn run_part1(input: String) {
    let total = input.lines()
        .map(|pair: &str| -> (Range, Range) {
            let (first, second) = pair.split_once(",").unwrap();
            return (first.into_range(), second.into_range())
        })
        .filter(|(r1, r2)| -> bool {
            return r1.contains(r2) || r2.contains(r1)
        })
        .count();
    println!("{}", total)
}

pub fn run_part2(input: String) {
    let total = input.lines()
        .map(|pair: &str| -> (Range, Range) {
            let (first, second) = pair.split_once(",").unwrap();
            return (first.into_range(), second.into_range())
        })
        .filter(|(r1, r2)| -> bool {
            return r1.overlaps_with(r2)
        })
        .count();
    println!("{}", total)
}