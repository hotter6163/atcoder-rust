mod read_tuple;

use std::error::Error;
use std::io;
use std::str::FromStr;

type ReadResult<T> = Result<T, Box<dyn Error + 'static>>;

pub fn read_number<T: FromStr>() -> ReadResult<T>
where
    <T as FromStr>::Err: Error + 'static,
{
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    line.trim().parse::<T>().map_err(|e| e.into())
}

pub fn read_numbers<T: FromStr>() -> ReadResult<Vec<T>>
where
    <T as FromStr>::Err: Error + 'static,
{
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    line.trim()
        .split_whitespace()
        .map(|s| s.parse::<T>().map_err(|e| e.into()))
        .collect()
}

pub fn read_string() -> ReadResult<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}
