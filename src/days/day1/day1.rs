use std::fs;

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
