use aoc_util;
use aoc_util::get_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = get_input("2");
    let mut valid_count = 0;
    for line in &input {
        let components : Vec<&str> = line.split(|c| c == '-' || c == ' ' || c == ':').collect();
        let min = components[0].parse::<u32>().expect("Should be a number");
        let max = components[1].parse::<u32>().expect("Should be a number");
        let char = components[2].chars().next().expect("Should have a char");
        let password = components[4]; // there is a : and space next to each other

        let count = password.chars().fold(0, |acc, c| if c == char { acc + 1 } else { acc } );
        if (min <= count) && (count <= max) {
            valid_count += 1;
        }
    }
    println!("Part 1: {}", valid_count)
}

fn part2() {
    let input = get_input("2");
    let mut valid_count = 0;
    for line in &input {
        let components : Vec<&str> = line.split(|c| c == '-' || c == ' ' || c == ':').collect();
        let first = components[0].parse::<u32>().expect("Should be a number") - 1;
        let second = components[1].parse::<u32>().expect("Should be a number") - 1;
        let char = components[2].chars().next().expect("Should have a char");
        let password = components[4]; // there is a : and space next to each other

        let chars: Vec<char> = password.chars().collect();
        // only matches if exactly one spot matches, so they cannot have same value. (both true
        // OR both false is a fail)
        if (char == chars[first as usize]) != (char == chars[second as usize]) {
            valid_count += 1;
        }
    }
    println!("Part 2: {}", valid_count)
}
