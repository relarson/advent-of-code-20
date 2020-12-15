use aoc_util;
use aoc_util::get_numeric_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let nums : Vec<u32> = get_numeric_input("1");

    for i in 0..(nums.len()-1) {
        for j in (i+1)..nums.len() {
            if (nums[i] + nums[j]) == 2020 {
                println!("Part 1: {}", nums[i] * nums[j]);
                return
            }
        }
    }
    println!("Part 1: Answer Not found");
}

fn part2() {
    let nums : Vec<u32> = get_numeric_input("1");

    for i in 0..(nums.len()-2) {
        for j in (i+1)..(nums.len()-1) {
            for k in (j+1)..nums.len() {
                if (nums[i] + nums[j] + nums[k]) == 2020 {
                    println!("Part 2: {}", nums[i] * nums[j] * nums[k]);
                    return
                }
            }
        }
    }
    println!("Part 2: Answer Not found");
}