use anyhow::{self, Context};
use aoc_2021::input::get_input_file;
use std::fmt::{self, Display};
use std::io::prelude::*;

fn main() -> anyhow::Result<()> {
    let (drawn_numbers, mut boards) =
        get_problem_input().context("Could not get problem input.")?;

    let mut last_winning_board_idx = None;
    let num_boards = boards.len();
    let mut num_wins = 0;

    'number_drawer_loop: for drawn_number in drawn_numbers {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            if !board.is_won() {
                board.draw_number(drawn_number);

                // board could've been won by the last .draw_number() call, so check again
                if board.is_won() {
                    num_wins += 1;

                    if num_wins == num_boards {
                        last_winning_board_idx = Some(board_idx);
                        break 'number_drawer_loop;
                    }
                }
            }
        }
    }

    if let Some(winning_board_idx) = last_winning_board_idx {
        println!("Winning board:");

        let board = &boards[winning_board_idx];

        println!(
            "\n                {}×{}",
            board.get_dimensions().0,
            board.get_dimensions().1
        );
        println!("{}", board);

        println!("\nScore: {}", board.score());
    }

    Ok(())
}

fn get_problem_input() -> anyhow::Result<(Vec<u16>, Vec<Board>)> {
    let mut input_file =
        get_input_file(2021, 12, 4, 2).context("Could not open problem input for reading.")?;

    let mut input = String::new();
    input_file
        .read_to_string(&mut input)
        .context("Could not read problem input to a string")?;

    // Figure out our line seperators that we need to split hunks by
    let line_seperator = if input.contains("\r\n") {
        "\r\n\r\n"
    } else {
        "\n\n"
    };

    let mut drawn_numbers = None;
    let mut boards = Vec::new();

    for hunk in input.split(line_seperator) {
        // First line is expected to be the list of drawn numbers
        if drawn_numbers.is_none() {
            drawn_numbers = Some(
                hunk.split(',')
                    .map(|n| n.parse::<u16>())
                    .collect::<Result<Vec<_>, _>>()
                    .context("Could not parse drawn number as a 16-bit unsigned integer")?,
            );
        } else {
            // Other hunks are expected to be boards
            boards.push(Board::new(hunk).with_context(|| {
                format!("Could not parse input hunk {:?} as a bingo board", hunk)
            })?);
        }
    }

    Ok((drawn_numbers.unwrap_or_default(), boards))
}

#[derive(Default, Debug)]
struct BoardCell {
    pub value: u16,
    pub drawn: bool,
}

impl Display for BoardCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.drawn {
            write!(f, "[{:>4}]", self.value)
        } else {
            write!(f, " {:>4} ", self.value)
        }
    }
}

#[derive(Debug)]
struct Board {
    /// The dimensions of the board, as a tuple of `(width, height)`.
    dimensions: (usize, usize),
    /// The board itself.
    board: Vec<Vec<BoardCell>>,

    last_number_drawn: Option<u16>,
}

impl Board {
    pub fn new(board_string: &str) -> anyhow::Result<Self> {
        let mut width = 0_usize;

        let mut board = Vec::new();

        for row in board_string.lines() {
            let mut board_row = Vec::with_capacity(0);

            for cell in row.split(' ') {
                let cell = cell.trim();
                if cell.is_empty() {
                    continue;
                }

                let cell = cell.parse::<u16>().with_context(|| {
                    format!("Could not parse {:?} as an unsigned 16-bit integer", cell)
                })?;

                board_row.push(BoardCell {
                    value: cell,
                    ..Default::default()
                });
            }

            if width == 0 {
                width = board_row.len();
            } else if board_row.len() != width {
                return Err(anyhow::anyhow!(
                    "Found a row in board with an unequal width."
                ));
            }

            board.push(board_row);
        }

        let height = board.len();

        Ok(Self {
            dimensions: (width, height),
            last_number_drawn: None,
            board,
        })
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    pub fn draw_number(&mut self, number: u16) {
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                if cell.value == number {
                    cell.drawn = true;
                }
            }
        }

        self.last_number_drawn = Some(number);
    }

    const REQUIRED_AMOUNT_IN_ROW_TO_WIN: usize = 5;

    pub fn is_won(&self) -> bool {
        // Check rows
        for row in self.board.iter() {
            let mut num_drawn = 0;

            for cell in row.iter() {
                if cell.drawn {
                    num_drawn += 1;
                } else {
                    num_drawn = 0;
                }

                if num_drawn >= Self::REQUIRED_AMOUNT_IN_ROW_TO_WIN {
                    return true;
                }
            }
        }

        // Check columns
        for col in 0..self.dimensions.0 {
            let mut num_drawn = 0;

            for row in 0..self.dimensions.1 {
                let cell = &self.board[row][col];

                if cell.drawn {
                    num_drawn += 1;
                } else {
                    num_drawn = 0;
                }

                if num_drawn >= Self::REQUIRED_AMOUNT_IN_ROW_TO_WIN {
                    return true;
                }
            }
        }

        false
    }

    pub fn score(&self) -> u64 {
        if let Some(last_drawn) = self.last_number_drawn {
            (last_drawn as u64)
                * self
                    .board
                    .iter()
                    .map(|row| {
                        row.iter()
                            .filter_map(|c| if c.drawn { None } else { Some(c.value as u64) })
                            .sum::<u64>()
                    })
                    .sum::<u64>()
        } else {
            0
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first_line = true;
        for row in self.board.iter() {
            let row_string = row
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<_>>()
                .join(" ");

            if first_line {
                first_line = false;
                write!(f, "{}", row_string)?;
            } else {
                write!(f, "\n{}", row_string)?;
            }
        }

        Ok(())
    }
}
