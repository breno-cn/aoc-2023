use std::{collections::HashMap, fs};

fn get_input() -> Vec<String> {
    let data =
        fs::read_to_string("./src/days/day1/input.txt").expect("Could not read file for day 1");

    data.split("\n").map(str::to_string).collect()
}

fn get_digits_part1(line: &String) -> u32 {
    // find leftmost digit
    let mut left = 0;
    let mut leftmost_digit = 0;

    while left < line.len() {
        let current_char = line.chars().nth(left).unwrap();
        if current_char.is_digit(10) {
            leftmost_digit = current_char.to_digit(10).unwrap();
            break;
        }

        left += 1;
    }

    // find rightmost digit
    let mut right: i32 = (line.len() - 1) as i32;
    let mut rightmost_digit = 0;

    while right >= 0 {
        let current_char = line.chars().nth(right as usize).unwrap();
        if current_char.is_digit(10) {
            rightmost_digit = current_char.to_digit(10).unwrap();
            break;
        }

        right -= 1;
    }

    leftmost_digit * 10 + rightmost_digit
}

fn get_digits_part2(line: &String) -> u32 {
    let digits: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    // find leftmost digit
    let mut left = 0;
    let mut leftmost_digit = 0;

    'leftmost: while left < line.len() {
        let current_char = line.chars().nth(left).unwrap();
        if current_char.is_digit(10) {
            leftmost_digit = current_char.to_digit(10).unwrap();
            break 'leftmost;
        }

        for i in 3..6 {
            let right = if left + i < line.len() {
                left + i
            } else {
                line.len()
            };

            if digits.keys().any(|s| *s == line[left..right].to_string()) {
                leftmost_digit = *digits.get(&line[left..right]).unwrap();
                break 'leftmost;
            }
        }

        left += 1;
    }

    // find rightmost digit
    let mut right: i32 = (line.len() - 1) as i32;
    let mut rightmost_digit = 0;

    'rightmost: while right >= 0 {
        let current_char = line.chars().nth(right as usize).unwrap();
        if current_char.is_digit(10) {
            rightmost_digit = current_char.to_digit(10).unwrap();
            break 'rightmost;
        }

        for i in 2..5 {
            let left = if (right as i32 - i) < 0 { 0 } else { right - i };

            let current_string = line[left as usize..right as usize + 1].to_string();
            if digits.keys().any(|s| *s == current_string) {
                rightmost_digit = *digits.get(current_string.as_str()).unwrap();
                break 'rightmost;
            }
        }

        right -= 1;
    }

    leftmost_digit * 10 + rightmost_digit
}

pub fn part1() -> u32 {
    let input = get_input();

    input.iter().map(get_digits_part1).sum()
}

pub fn part2() -> u32 {
    let input = get_input();

    input.iter().map(get_digits_part2).sum()
}
