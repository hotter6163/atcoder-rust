fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let numbers: Vec<u32> = s
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let min = numbers
        .iter()
        .map(|x| {
            let mut count = 0;
            let mut y = *x;
            while y % 2 == 0 {
                count += 1;
                y /= 2;
            }
            count
        })
        .min()
        .unwrap();

    println!("{}", min);
}
