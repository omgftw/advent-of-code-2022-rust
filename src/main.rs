mod day1;
mod day2;
mod day1_ai;
mod day2_ai;
mod day3_ai;
mod day11_ai;

fn main() {
    println!("Day 1:");
    println!("======");
    // day1::day1();
    let day1 = day1::main().unwrap();
    println!("Part 1: {}", day1.0);
    println!("Part 2: {}", day1.1);
    println!();

    // Assertion based on correct values
    assert_eq!(day1.0, 67016);
    assert_eq!(day1.1, 200116);

    // ------------------------------------

    println!("Day 2:");
    println!("======");
    let day2 = day2::main().unwrap();
    println!("Part 1: {}", day2.0);
    println!("Part 2: {}", day2.1);
    println!();

    // Assertion based on correct values
    assert_eq!(day2.0, 8392);
    assert_eq!(day2.1, 10116);

    // ------------------------------------

    println!("Day 1 - AI:");
    println!("===========");
    let day1_ai = day1_ai::main().unwrap();
    println!("Part 1: {}", day1_ai.0);
    println!("Part 2: {}", day1_ai.1);
    println!();

    // Assertion based on correct values
    assert_eq!(day1_ai.0, 67016);
    assert_eq!(day1_ai.1, 200116);

    // ------------------------------------

    println!("Day 2 - AI:");
    println!("===========");
    let day2 = day2_ai::main().unwrap();
    println!("Part 1: {}", day2.0);
    println!("Part 2: {}", day2.1);
    println!();

    // Assertion based on correct values
    assert_eq!(day2.0, 8392);
    assert_eq!(day2.1, 10116);

    // ------------------------------------

    println!("Day 3 - AI:");
    println!("===========");
    let day3 = day3_ai::main().unwrap();
    println!("Part 1: {}", day3.0);
    println!("Part 2: {}", day3.1);
    println!();

    // Assertion based on correct values
    assert_eq!(day3.0, 8139);
    assert_eq!(day3.1, 2668);

    // ------------------------------------

    // println!("Day 11 - AI:");
    // println!("============");
    // day11_ai::main();
    // println!();

    // ------------------------------------
}
