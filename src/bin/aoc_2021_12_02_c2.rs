use anyhow::{self, Context};
use aoc_2021::input::get_input_file_bufreader;
use std::{io::prelude::*, str::FromStr};
use thiserror::Error;

enum Action {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Action {
    type Err = ActionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tokens = input.trim().split(' ');
        let token_count = tokens.clone().count();

        if token_count != 2 {
            return Err(ActionError::ParseWrongNumTokens {
                input: input.to_string(),
                num_tokens_found: token_count,
            });
        }

        let action = tokens.next().ok_or_else(|| ActionError::TokenIteration {
            input: input.to_string(),
        })?;

        let parameter = tokens.next().ok_or_else(|| ActionError::TokenIteration {
            input: input.to_string(),
        })?;

        let parameter = parameter
            .parse::<i64>()
            .map_err(|e| ActionError::ParseInt {
                input: input.to_string(),
                parse_err: e,
            })?;

        match action {
            "forward" => Ok(Self::Forward(parameter)),
            "down" => Ok(Self::Down(parameter)),
            "up" => Ok(Self::Up(parameter)),

            unknown_action => Err(ActionError::UnknownAction(unknown_action.to_string())),
        }
    }
}

#[derive(Debug, Error)]
enum ActionError {
    /// For when the wrong number of tokens is found in an action string
    #[error("Found {num_tokens_found} tokens when trying to parse string {input:?}, expected 2")]
    ParseWrongNumTokens {
        input: String,
        num_tokens_found: usize,
    },

    /// For when a weird error comes up while trying to iterate through split tokens. Should never
    /// come up, but we have it in case.
    #[error("Could not advance iterator through space-delimited tokens in string {input:?}")]
    TokenIteration { input: String },

    /// For when we're unable to parse an integer in a input string
    #[error(
        "Could not parse 64-bit signed integer from input string {input:?} due to {parse_err}"
    )]
    ParseInt {
        input: String,
        parse_err: std::num::ParseIntError,
    },

    /// For when an unknown action is parsed
    #[error("Unknown action: {0:?}")]
    UnknownAction(String),
}

#[derive(Default)]
struct Submarine {
    pub horiz_pos: i64,
    pub depth: i64,
    pub aim: i64,
}

impl Submarine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&mut self, action: Action) {
        match action {
            Action::Forward(dist) => {
                self.horiz_pos += dist;
                self.depth += dist * self.aim;
            }

            Action::Down(delta) => self.aim += delta,
            Action::Up(delta) => self.aim -= delta,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut sub = Submarine::new();
    let input_buf = get_input_file_bufreader(2021, 12, 2, 1)
        .context("Could not open challenge input file for reading")?;

    for line in input_buf.lines() {
        let line = line.context("Could not read line from challenge input file")?;

        let action = line
            .parse::<Action>()
            .context("Could not parse line from challenge input file as an action")?;

        sub.execute(action);
    }

    println!("Final submarine location:");
    println!("\tHorizontal position: {}", sub.horiz_pos);
    println!("\tDepth: {}", sub.depth);
    println!("\tAim: {}", sub.aim);
    println!(
        "\tProduct of horizontal position and depth: {}",
        sub.horiz_pos * sub.depth
    );

    Ok(())
}
