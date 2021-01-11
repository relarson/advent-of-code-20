use aoc_util;
use aoc_util::get_input;

use std::cmp;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let input = get_input("5");
    let mut max_id = 0;
    for bsp in &input {
        let id = find_seat_id(bsp);
        max_id = cmp::max(max_id, id);
    }
    max_id
}

fn part2() -> u32 {
    let input = get_input("5");
    let mut ids = Vec::new();
    for bsp in &input {
        let id = find_seat_id(bsp);
        ids.push(id);
    }
    ids.sort();
    let mut prev = ids[0];
    for id in ids[1..].into_iter() {
        if id - prev > 1 {
            return id - 1;
        } else {
            prev = *id;
        }
    }
    panic!("Did not find my seat!");
}

fn find_seat_id(bsp: &str) -> u32 {
    let row = traverse_bsp_section(&bsp[0..7], 0, 127, 'F', 'B');
    let col = traverse_bsp_section(&bsp[7..10], 0, 7, 'L', 'R');

    (row * 8) + col
}

fn traverse_bsp_section(bsp: &str, min: u32, max: u32, low_ch: char, high_ch: char) -> u32 {
    let mut curr_min = min;
    let mut curr_max = max;
    for c in bsp.chars() {
        if low_ch == c {
            // lower half
            curr_max = (curr_max + curr_min) / 2;
        } else if high_ch == c {
            // upper half
            curr_min = (curr_max + curr_min + 1) / 2;
        } else {
            panic!("Invalid BSP character {}", c);
        }
    }
    if curr_min != curr_max {
        panic!("Didn't find a result for bsp: {:?} Converged to: {} - {}", bsp, curr_min,
               curr_max);
    }
    curr_min
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_seat() {
        assert_eq!(find_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(find_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(find_seat_id("BBFFBBFRLL"), 820);
    }
}

