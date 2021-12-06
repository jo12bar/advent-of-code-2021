use anyhow::{Context, Error};
use aoc_2021::input::get_input_file_bufreader;
use std::io::prelude::*;

fn main() -> Result<(), Error> {
    let input_buf =
        get_input_file_bufreader(2021, 12, 1, 2).context("Could not open challenge input file")?;

    let input_vec = input_buf
        .lines()
        .map(|line_or_err| {
            line_or_err
                .context("Could not read line from input file")
                .and_then(|line| {
                    line.trim()
                        .parse::<i64>()
                        .context("Could not read input line as a 64-bit signed integer")
                })
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Could not grab input as a list of 64-bit signed integers")?;

    let mut prev_measurement = None;
    let mut num_increases = 0u64;

    for indiv_measurements in input_vec.as_slice().windows(3) {
        let measurement = indiv_measurements[0] + indiv_measurements[1] + indiv_measurements[2];

        if let Some(prev_meas) = prev_measurement {
            if measurement > prev_meas {
                num_increases += 1;
            }
        }
        prev_measurement = Some(measurement);
    }

    println!("Number of depth measurement increases: {}", num_increases);
    Ok(())
}
