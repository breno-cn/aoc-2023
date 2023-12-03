use std::fs;

fn get_input() -> Vec<Vec<char>> {
    let data = fs::read_to_string("./src/days/day3/input.txt")
        .expect("Could not read input file for day 3");

    data.lines().map(|line| line.chars().collect()).collect()
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

fn is_valid_position(i: i32, j: i32, board_width: usize, board_height: usize) -> bool {
    i >= 0 && i < board_height as i32 && j >= 0 && j < board_width as i32
}

fn is_position_adjacent(
    board: &Vec<Vec<char>>,
    row: i32,
    col: i32,
    board_width: usize,
    board_height: usize,
) -> bool {
    for i in row - 1..row + 2 {
        for j in col - 1..col + 2 {
            if is_valid_position(i, j, board_width, board_height) {
                if is_symbol(board[i as usize][j as usize]) {
                    return true;
                }
            }
        }
    }

    false
}

fn number_is_adjacent(
    board: &Vec<Vec<char>>,
    left: i32,
    right: i32,
    row: i32,
    board_width: usize,
    board_height: usize,
) -> bool {
    for pos in left..right {
        if is_position_adjacent(board, row, pos, board_width, board_height) {
            return true;
        }
    }

    false
}

pub fn part_1() -> u32 {
    let board = get_input();
    let board_width = board.len();
    let board_height = board[0].len();
    let mut result = 0;

    // let mut i = 0;
    for i in 0..board_height {
        let mut left = 0;
        let mut right = 0;
        while left < board_width {
            if board[i][left].is_digit(10) {
                // find where the number ends
                right = left;
                while right < board_width && board[i][right].is_digit(10) {
                    right += 1;
                }

                if number_is_adjacent(
                    &board,
                    left as i32,
                    right as i32,
                    i as i32,
                    board_width,
                    board_height,
                ) {
                    let number = board[i][left..right].to_vec();

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
