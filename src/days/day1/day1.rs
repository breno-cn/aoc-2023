use std::{collections::HashMap, fs};

fn get_input() -> Vec<String> {
    let data =
        fs::read_to_string("./src/days/day1/input.txt").expect("Could not read file for day 1");

    data.split("\n").map(str::to_string).collect()
}

pub fn part1() -> u32 {
    let input = get_input();

    input
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<char>>()
        })
        .map(|line| {
            (
                line.first().unwrap().to_digit(10).unwrap(),
                line.last().unwrap().to_digit(10).unwrap(),
            )
        })
        .map(|(digit1, digit2)| digit1 * 10 + digit2)
        .sum()
}

pub fn part2() -> u32 {
    let input = get_input();

    let digits = HashMap::from([
        ("three", '3'),
        ("seven", '7'),
        ("eight", '8'),
        ("four", '4'),
        ("five", '5'),
        ("nine", '9'),
        ("one", '1'),
        ("two", '2'),
        ("six", '6'),
    ]);

    input
        .iter()
        .map(|line| {
            let mut left = 0;
            let mut new_line: Vec<char> = Vec::new();

            while left < line.len() {
                for i in 3..6 {
                    let right = if left + i < line.len() { left + i } else { line.len() };

                    if digits.keys().any(|s| *s == line[left..right].to_string()) {
                        let digit = digits.get(&line[left..right]).unwrap();
                        new_line.push(*digit);
                    } else {
                        let current_char = line.chars().nth(left).unwrap();
                        new_line.push(current_char);
                    }
                }

                left += 1;
            }
            
            new_line
        })
        .map(|line| {
            line.iter()
                .filter(|c| c.is_digit(10))
                .map(char::clone)
                .collect::<Vec<char>>()
        })
        .map(|line| {
            (
                line.first().unwrap().to_digit(10).unwrap(),
                line.last().unwrap().to_digit(10).unwrap(),
            )
        })
        .map(|(digit1, digit2)| digit1 * 10 + digit2)
        .sum()
}
