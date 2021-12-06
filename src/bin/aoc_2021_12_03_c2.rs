use anyhow::{self, Context};
use aoc_2021::input::get_input_file_bufreader;
use std::io::prelude::*;

fn get_input_as_grid() -> anyhow::Result<Vec<Vec<u8>>> {
    let input_buf = get_input_file_bufreader(2021, 12, 3, 1)
        .context("Could not open challenge input file for reading")?;

    let mut rows: Vec<Vec<u8>> = Vec::new();

    for line in input_buf.lines() {
        let line = line.context("Could not read line from challenge input file")?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut row = Vec::new();

        for digit in line.chars() {
            match digit {
                '1' => row.push(1),
                '0' => row.push(0),
                c => return Err(anyhow::anyhow!("Illegal character {:?}", c)),
            }
        }

        rows.push(row);
    }

    Ok(rows)
}

fn get_digit_counts_for_col(col: usize, grid: &[Vec<u8>]) -> (usize, usize) {
    let mut counts = (0, 0);

    for row in grid.iter() {
        let digit = row[col];
        if digit == 0 {
            counts.0 += 1;
        } else if digit == 1 {
            counts.1 += 1;
        }
    }

    counts
}

fn get_rating(
    grid: &[Vec<u8>],
    filter_digit_decider: impl Fn(usize, usize) -> u8,
) -> anyhow::Result<u64> {
    let mut grid = grid.to_owned();
    let mut col = 0;
    let num_cols = grid[0].len();

    while grid.len() > 1 {
        let (zeros, ones) = get_digit_counts_for_col(col, &grid);

        let filter_digit = filter_digit_decider(zeros, ones);

        grid = grid
            .into_iter()
            .filter(|row| row[col] == filter_digit)
            .collect();

        col = (col + 1) % num_cols;
    }

    let mut rating = 0u64;

    for (i, digit) in grid[0].iter().rev().enumerate() {
        rating |= (*digit as u64) << i;
    }

    Ok(rating)
}

fn get_oxygen_generator_rating(grid: &[Vec<u8>]) -> anyhow::Result<u64> {
    get_rating(grid, |zeros, ones| if ones >= zeros { 1 } else { 0 })
}

fn get_co2_scrubber_rating(grid: &[Vec<u8>]) -> anyhow::Result<u64> {
    get_rating(grid, |zeros, ones| if zeros <= ones { 0 } else { 1 })
}

fn main() -> anyhow::Result<()> {
    let grid = get_input_as_grid().context("Could not get input as grid")?;
    let oxygen_generator_rating =
        get_oxygen_generator_rating(&grid).context("Could not get oxygen generator rating")?;
    let co2_scrubber_rating =
        get_co2_scrubber_rating(&grid).context("Could not get CO2 scrubber rating")?;

    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    println!("CO₂ scrubber rating:     {}", co2_scrubber_rating);
    println!("Life support rating:     {}", life_support_rating);

    Ok(())
}
