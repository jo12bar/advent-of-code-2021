use std::{fs::File, io::BufReader};
use thiserror::Error;

pub fn get_input_file(year: u32, month: u8, day: u8, challenge_no: u8) -> Result<File, InputError> {
    let file_path = format!(
        "input/{:04}_{:02}_{:02}/c{}.txt",
        year, month, day, challenge_no
    );

    File::open(file_path).map_err(|source| InputError::InputOpenError { source })
}

pub fn get_input_file_bufreader(
    year: u32,
    month: u8,
    day: u8,
    challenge_no: u8,
) -> Result<BufReader<File>, InputError> {
    Ok(BufReader::new(get_input_file(
        year,
        month,
        day,
        challenge_no,
    )?))
}

/// Enumerates all possible errors that may be encountered when this library tries to deal with
/// challenge inputs.
#[derive(Error, Debug)]
pub enum InputError {
    /// Represents a failure to open a file as challenge input.
    #[error("Could not open challenge input.")]
    InputOpenError { source: std::io::Error },

    /// Represents all other cases of [`std::io::Error`].
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
