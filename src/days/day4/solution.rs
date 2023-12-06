use std::fs;

fn get_input() -> Vec<(Vec<i32>, Vec<i32>)> {
    let data = fs::read_to_string("./src/days/day4/input.txt")
        .expect("Could not read input file for day 4");

    data.lines()
        .map(|line| line.split(": ").skip(1).collect::<Vec<&str>>()[0])
        .map(|line| line.split(" | ").collect::<Vec<&str>>())
        .map(|game| {
            game.iter()
                .map(|line| {
                    line.split(" ")
                        .filter(|num| *num != "")
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .map(|game| (game[0].clone(), game[1].clone()))
        .collect()
}

fn game_score(winning_cards: &Vec<i32>, guesses: &Vec<i32>) -> i32 {
    let mut corrects = 0;

    for card in winning_cards {
        if guesses.contains(&card) {
            corrects += 1;
        }
    }

    if corrects == 0 {
        0
    } else {
        1 << (corrects - 1)
    }
}

pub fn part1() -> i32 {
    let games = get_input();

    games
        .iter()
        .map(|(winning, guesses)| game_score(winning, guesses))
        .sum()
}
