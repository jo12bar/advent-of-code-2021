use anyhow::{Context, Error};
use aoc_2021::input::get_input_file_bufreader;
use std::io::prelude::*;

fn main() -> Result<(), Error> {
    let input_buf =
        get_input_file_bufreader(2021, 12, 1, 1).context("Could not open challenge input file")?;

    let mut prev_measurement = None;
    let mut num_increases = 0u64;

    for measurement in input_buf.lines() {
        let measurement = measurement.context("Could not read line from input file")?;
        let measurement = measurement.trim();
        let measurement = measurement
            .parse::<i64>()
            .context("Could not read input line as a 64-bit int")?;

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
