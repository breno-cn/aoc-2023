use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Game {
    reds: Vec<i32>,
    greens: Vec<i32>,
    blues: Vec<i32>,
}

fn process_game(game: &Vec<&str>) -> Game {
    let mut result = Game {
        reds: vec![],
        greens: vec![],
        blues: vec![],
    };

    for round in game {
        round
            .split(", ")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|guess| {
                let raw = guess.split(" ").collect::<Vec<&str>>();
                let ammount = raw[0].parse::<i32>().unwrap();
                let collor = raw[1];

                match collor {
                    "red" => result.reds.push(ammount),
                    "green" => result.greens.push(ammount),
                    "blue" => result.blues.push(ammount),
                    _ => todo!("this shouldn't happen..."),
                };
            });
    }

    result
}

fn get_input() -> Vec<Game> {
    let data = fs::read_to_string("./src/days/day2/input.txt")
        .expect("Could not read input file for day 2");

    let games: Vec<Game> = data
        .lines()
        .map(|line| line.split(": ").collect::<Vec<&str>>())
        .map(|line| line[1])
        .map(|game| game.split("; ").collect::<Vec<&str>>())
        .map(|game| process_game(&game))
        .collect();
    
    games
}

fn is_game_possible(red: i32, green: i32, blue: i32, game: &Game) -> bool {
    game.reds.iter().all(|ammount| *ammount <= red)
        && game.greens.iter().all(|ammount| *ammount <= green)
        && game.blues.iter().all(|ammount| *ammount <= blue)
}

pub fn part1() -> i32 {
    let games = get_input();
    let red_target = 12;
    let green_target = 13;
    let blue_target = 14;

    games
        .iter()
        .enumerate()
        .filter(|(_, game)| is_game_possible(red_target, green_target, blue_target, game))
        .map(|(index, _)| (index + 1) as i32)
        .sum()
}
