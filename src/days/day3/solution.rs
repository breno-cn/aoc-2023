use core::num;
use std::{collections::HashSet, fs};

struct Board {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

fn get_input() -> Board {
    let data = fs::read_to_string("./src/days/day3/input.txt")
        .expect("Could not read input file for day 3");

    let data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    Board {
        width: data[0].len(),
        height: data.len(),
        data,
    }
}

impl Board {
    pub fn is_valid_position(&self, i: i32, j: i32) -> bool {
        i >= 0 && i < self.height as i32 && j >= 0 && j < self.width as i32
    }

    pub fn is_position_adjacent(&self, row: i32, col: i32) -> bool {
        for i in row - 1..row + 2 {
            for j in col - 1..col + 2 {
                if self.is_valid_position(i, j) {
                    if is_symbol(self.data[i as usize][j as usize]) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn number_is_adjacent(&self, left: i32, right: i32, row: i32) -> bool {
        for pos in left..right {
            if self.is_position_adjacent(row, pos) {
                return true;
            }
        }

        false
    }

    pub fn get_leftmost_position(&self, row: i32, col: i32) -> i32 {
        let mut left = col;
        while left > 0
            && self.is_valid_position(row, left - 1)
            && self.data[row as usize][(left - 1) as usize].is_digit(10)
        {
            left -= 1;
        }

        left
    }

    pub fn get_number(&self, row: i32, left_position: i32) -> u32 {
        let mut number = Vec::new();
        let mut left = left_position;

        while left < self.width as i32 && self.data[row as usize][left as usize].is_digit(10) {
            number.push(self.data[row as usize][left as usize]);
            left += 1;
        }

        vec_to_int(number).unwrap()
    }

    pub fn gear_ratio(&self, row: i32, col: i32) -> Option<u32> {
        let mut leftmost_positions = HashSet::new();

        for i in row - 1..row + 2 {
            for j in col - 1..col + 2 {
                if self.is_valid_position(i, j) && self.data[i as usize][j as usize].is_digit(10) {
                    let leftmost_position = self.get_leftmost_position(i, j);
                    leftmost_positions.insert((i, leftmost_position));
                }
            }
        }

        if leftmost_positions.len() != 2 {
            return None;
        }

        let result = leftmost_positions
            .iter()
            .map(|(row, col)| self.get_number(*row, *col))
            .collect::<Vec<u32>>();

        Some(result[0] * result[1])
    }
}

pub fn vec_to_int(digits: impl IntoIterator<Item = char>) -> Option<u32> {
    const RADIX: u32 = 10;

    digits
        .into_iter()
        .map(|c| c.to_digit(RADIX))
        .try_fold(0, |ans, i| i.map(|i| ans * RADIX + i))
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

pub fn part1() -> u32 {
    let board = get_input();
    let mut result = 0;

    // let mut i = 0;
    for i in 0..board.height {
        let mut left = 0;
        let mut right = 0;
        while left < board.width {
            if board.data[i][left].is_digit(10) {
                // find where the number ends
                right = left;
                while right < board.width && board.data[i][right].is_digit(10) {
                    right += 1;
                }

                if board.number_is_adjacent(left as i32, right as i32, i as i32) {
                    let number = board.data[i][left..right].to_vec();

                    result += vec_to_int(number).unwrap();
                }

                left = right;
            } else {
                left += 1;
            }
        }
    }

    result
}

pub fn part2() -> u32 {
    let board = get_input();
    let mut result = 0;

    for i in 0..board.height {
        for j in 0..board.width {
            if board.data[i][j] == '*' {
                // verify if it's a gear and sum the result with it's gear ratio
                if let Some(gear_ratio) = board.gear_ratio(i as i32, j as i32) {
                    result += gear_ratio;
                }
            }
        }
    }

    result
}
