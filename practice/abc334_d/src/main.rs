#[macro_export]
macro_rules! read_tuple {
    ($($t:ty),*) => {
        {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let mut iter = s.trim().split_whitespace();
            (
                $(
                    iter.next().unwrap().parse::<$t>().unwrap(),
                )*
            )
        }
    };
}

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

fn main() {
    let (_, q) = read_tuple!(u32, u32);
    let mut sleds = read_numbers::<u64>().unwrap();
    sleds.sort();
    let mut required = Vec::new();
    for i in 0..sleds.len() {
        if i == 0 {
            required.push(sleds[i]);
        } else {
            required.push(required[i - 1] + sleds[i]);
        }
    }
    drop(sleds);

    let mut result = Vec::new();
    for _ in 0..q {
        let query = read_number::<u64>().unwrap();

        let mut left = 0;
        let mut right = required.len();
        while left < right {
            let mid = (left + right) / 2;
            if required[mid] == query {
                left = mid + 1;
                break;
            } else if required[mid] < query {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        result.push(left);
    }

    for r in result {
        println!("{}", r);
    }
}
