type ReadResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn read_line<T, F>(formatter: F) -> ReadResult<T>
where
    F: FnOnce(&str) -> T,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    Ok(formatter(s.trim()))
}

pub fn read_lines<T, F>(n: usize, mut formatter: F) -> ReadResult<Vec<T>>
where
    F: FnMut(&str) -> T,
{
    (0..n).map(|_| read_line(&mut formatter)).collect()
}

pub fn read_string() -> ReadResult<String> {
    read_line(|s| s.to_string())
}

pub fn read_numbers<T: std::str::FromStr>(n: usize) -> ReadResult<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_line(|s| {
        s.split_whitespace()
            .map(|e| e.parse::<T>().unwrap())
            .take(n)
            .collect()
    })
}

pub fn read_number<T: std::str::FromStr>() -> ReadResult<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_numbers(1).map(|mut v| v.pop().unwrap())
}
