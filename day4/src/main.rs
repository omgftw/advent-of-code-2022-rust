use std::ops::RangeInclusive;

fn full_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    if a.start() <= b.start() && a.end() >= b.end() || b.start() <= a.start() && b.end() >= a.end()
    {
        return true;
    }
    false
}

fn any_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    if a.start() <= b.end() && a.end() >= b.start() {
        return true;
    }
    false
}

fn main() {
    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");
    let mut entire_overlaps = 0;
    let mut any_overlaps = 0;

    for line in input.lines() {
        let ranges = line.split(',');
        let ranges = ranges
            .map(|range| {
                let mut range = range.split('-');
                let start = range.next().unwrap().parse::<i32>().unwrap();
                let end = range.next().unwrap().parse::<i32>().unwrap();
                start..=end
            })
            .collect::<Vec<_>>();
        let [elf1, elf2] = ranges.as_slice() else { todo!() };
        if full_overlap(elf1, elf2) {
            entire_overlaps += 1;
        }
        if any_overlap(elf1, elf2) {
            any_overlaps += 1;
        }
    }

    println!("Part 1: {}", entire_overlaps);
    println!("Part 2: {}", any_overlaps);
}
