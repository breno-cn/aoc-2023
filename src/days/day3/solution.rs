use std::fs;

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
    fn is_valid_position(&self, i: i32, j: i32) -> bool {
        i >= 0 && i < self.height as i32 && j >= 0 && j < self.width as i32
    }

    fn is_position_adjacent(&self, row: i32, col: i32) -> bool {
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

    fn number_is_adjacent(&self, left: i32, right: i32, row: i32) -> bool {
        for pos in left..right {
            if self.is_position_adjacent(row, pos) {
                return true;
            }
        }

        false
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
