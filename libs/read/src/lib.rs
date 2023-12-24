mod read_tuple;

use std::error::Error;
use std::io;

pub fn read_line<T, F>(parser: F) -> Result<T, Box<dyn Error>>
where
    F: Fn(&str) -> Result<T, std::io::Error>,
{
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let result = parser(&line)?;
    Ok(result)
}

pub fn read_lines<T, F>(n: i32, parser: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: Fn(&str) -> Result<T, std::io::Error>,
{
    let mut lines = Vec::new();
    for _ in 0..n {
        let line = read_line(&parser)?;
        lines.push(line);
    }
    Ok(lines)
}
