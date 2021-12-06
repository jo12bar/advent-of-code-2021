use anyhow::{self, Context};
use aoc_2021::input::get_input_file_bufreader;
use std::{io::prelude::*, iter::repeat};

fn main() -> anyhow::Result<()> {
    let input_buf = get_input_file_bufreader(2021, 12, 3, 1)
        .context("Could not open challenge input file for reading")?;

    let mut cols: Vec<Vec<u8>> = Vec::new();

    for line in input_buf.lines() {
        let line = line.context("Could not read line from challenge input file")?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if cols.is_empty() {
            cols = repeat(Vec::new()).take(line.len()).collect();
        }

        for (i, digit) in line.chars().enumerate() {
            match digit {
                '1' => cols[i].push(1),
                '0' => cols[i].push(0),
                c => return Err(anyhow::anyhow!("Illegal character {:?}", c)),
            }
        }
    }

    let most_common_digits = cols.into_iter().map(|col| {
        (
            col.iter().filter(|digit| **digit == 0).count(),
            col.iter().filter(|digit| **digit == 1).count(),
        )
    });

    let mut bitmask: u64 = 0;
    let mut gamma_rate: u64 = 0;
    let mut num_cols: usize = 0;

    for (col, (n_zero, n_one)) in most_common_digits.rev().enumerate() {
        if n_zero < n_one {
            gamma_rate |= 1 << col;
        }

        bitmask |= 1 << col;

        num_cols += 1;
    }

    let epsilon_rate = (!gamma_rate) & bitmask;
    let power_consumption = gamma_rate * epsilon_rate;

    println!("Gamma rate:\t{0}\t({0:01$b})", gamma_rate, num_cols);
    println!("Epsilon rate:\t{0}\t({0:01$b})", epsilon_rate, num_cols);
    println!("Power consumption:\t{0}", power_consumption);

    Ok(())
}
