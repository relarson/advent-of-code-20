use aoc_util;
use aoc_util::get_input;

use std::collections::HashMap;
use regex::Regex;

fn main() {
    println!("Part 1: {}", check_passports(is_valid_part_1));
    println!("Part 2: {}", check_passports(is_valid_part_2));
}

fn check_passports<F>(validator: F) -> u32
    where F: Fn(&HashMap<&str, &str>) -> bool {
    let input = get_input("4");

    let mut passport_info = HashMap::new();
    let mut valid_count = 0;

    for line in &input {
        if  line.is_empty() {
            // eval prior and clear
            if validator(&passport_info) {
                valid_count += 1;
            }
            passport_info.clear();
        } else {
            for key_value in line.split_whitespace() {
                let parts: Vec<&str> = key_value.split(":").collect();
                passport_info.insert(parts[0], parts[1]);
            }
        }
    }
    valid_count
}

fn is_valid_part_1(passport_info: &HashMap<&str, &str>) -> bool {
    (passport_info.len() == 8) || (passport_info.len() == 7 && !passport_info.contains_key("cid"))
}

fn is_valid_part_2(passport_info: &HashMap<&str, &str>) -> bool {
    if !is_valid_part_1(passport_info) {
        return false
    }

    let byr = passport_info.get("byr").unwrap_or(&"0").parse::<u32>().unwrap();
    if  (byr < 1920) || (byr > 2002) {
        return false
    }

    let iyr = passport_info.get("iyr").unwrap_or(&"0").parse::<u32>().unwrap();
    if  (iyr < 2010) || (iyr > 2020) {
        return false
    }

    let eyr = passport_info.get("eyr").unwrap_or(&"0").parse::<u32>().unwrap();
    if  (eyr < 2020) || (eyr > 2030) {
        return false
    }

    let ecl = passport_info.get("ecl").unwrap_or(&"");
    if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl) {
        return false
    }

    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    if !hcl_re.is_match(passport_info.get("hcl").unwrap_or(&"")) {
        return false
    }

    let pid_re = Regex::new(r"^\d{9}$").unwrap();
    if !pid_re.is_match(passport_info.get("pid").unwrap_or(&"")) {
        return false
    }

    let hgt_re = Regex::new(r"^(\d{2,3})(in|cm)$").unwrap();
    let hgt_cap = hgt_re.captures(passport_info.get("hgt").unwrap_or(&""));

    match hgt_cap {
        Some(caps) => {
            if caps.get(2).unwrap().as_str() == "in" {
                let inches = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                return (inches >= 59) && (inches <= 76)
            } else {
                let cms = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                return (cms >= 150) && (cms <= 193)
            }
        },
        None => return false
    }
}