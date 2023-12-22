fn read_input() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<i32>().unwrap()
}

fn main() {
    let a = read_input();
    let b = read_input();
    let c = read_input();
    let x = read_input();

    let div = x / 50;

    let count = (0..a + 1)
        .flat_map(|i| (0..b + 1).map(move |j| (i, j)))
        .flat_map(|(i, j)| (0..c + 1).map(move |k| (i, j, k)))
        .filter(|(i, j, k)| i * 10 + j * 2 + k == div)
        .count();

    println!("{}", count);
}
