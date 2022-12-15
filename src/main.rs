mod day1;
mod day1_ai;
mod day2;
mod day3;

fn main() {
    println!("Day 1:");
    println!("======");
    // day1::day1();
    let day1 = day1::day1().unwrap();
    println!("Part 1: {}", day1.0);
    println!("Part 2: {}", day1.1);
    println!();

    // Assertion based on correct values
    assert_eq!(day1.0, 67016);
    assert_eq!(day1.1, 200116);

    // ------------------------------------

    // println!("Day 1 - AI:");
    // println!("======");
    // day1_ai::day1_ai();
    // println!();

    println!("Day 2:");
    println!("======");
    let day2 = day2::day2().unwrap();
    println!("Part 1: {}", day2.0);
    println!("Part 2: {}", day2.1);
    println!();

    // Assertion based on correct values
    assert_eq!(day2.0, 8392);
    assert_eq!(day2.1, 10116);

    // ------------------------------------

    println!("Day 3:");
    println!("======");
    let day3 = day3::day3().unwrap();
    println!("Part 1: {}", day3.0);
    println!("Part 2: {}", day3.1);
    println!();

    // Assertion based on correct values
    assert_eq!(day3.0, 8139);
    assert_eq!(day3.1, 2668);

    // ------------------------------------
}
