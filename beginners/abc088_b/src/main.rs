fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<i32>().unwrap();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut cards = s
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    cards.sort_by(|a, b| b.cmp(a));

    let (a, b) = cards.iter().enumerate().fold((0, 0), |(a, b), (i, &card)| {
        if i % 2 == 0 {
            (a + card, b)
        } else {
            (a, b + card)
        }
    });

    println!("{}", a - b);
}
