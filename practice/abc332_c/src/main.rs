extern crate read;

use read::read_tuple;

fn main() {
    let (_, m) = read_tuple!(usize, usize);
    let schedule = read::read_line(|s| Ok(s.to_string())).unwrap();

    let required = schedule
        .chars()
        .collect::<Vec<char>>()
        .split(|&c| c == '0')
        .map(|v| {
            let total = v.iter().filter(|&&c| c == '1' || c == '2').count();
            let logo = v.iter().filter(|&&c| c == '2').count();
            (total, logo)
        })
        .map(|(total, logo)| (total - m).max(logo))
        .max()
        .unwrap();
    println!("{}", required)
}
