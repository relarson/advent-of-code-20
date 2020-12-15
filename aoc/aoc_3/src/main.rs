use aoc_util;
use aoc_util::get_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Part 1: {}", check_with_slope(3, 1));
}

fn part2() {
    let one_one = check_with_slope(1, 1);
    let three_one = check_with_slope(3, 1);
    let five_one = check_with_slope(5, 1);
    let seven_one = check_with_slope(7, 1);
    let one_two = check_with_slope(1, 2);
    println!("Part 2: {}", one_one * three_one * five_one * seven_one * one_two);
}

fn check_with_slope(right: usize, down: usize) -> u32 {
    let input = get_input("3");
    let mut trees = 0;
    let mut col = 0;
    let width = input[0].len();
    for i in (0..input.len()).step_by(down) {
        let row = &input[i];
        if '#' == row.chars().collect::<Vec<_>>()[col] {
            trees += 1;
        }
        col = (col + right) % width;
    }
    trees
}
