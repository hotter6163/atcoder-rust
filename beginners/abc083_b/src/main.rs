fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let (n, a, b) = {
        let mut iter = s.split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        (n, a, b)
    };

    let count = (1..n + 1)
        .filter(|&i| {
            let sum = sum(i);
            a <= sum && sum <= b
        })
        .sum::<i32>();

    println!("{}", count);
}

fn sum(n: i32) -> i32 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
