use read;

fn main() {
    let numbers = read::read_numbers::<usize>(2).unwrap();
    let m = numbers[1];
    let schedule = read::read_string().unwrap();

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
