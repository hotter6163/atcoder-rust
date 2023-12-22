fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let (n, y) = {
        let mut ws = s.trim().split_whitespace();
        let n = ws.next().unwrap().parse::<i32>().unwrap();
        let y = ws.next().unwrap().parse::<i32>().unwrap();
        (n, y)
    };

    for a in 0..n + 1 {
        for b in 0..n + 1 - a {
            let c = n - a - b;
            if 10000 * a + 5000 * b + 1000 * c == y {
                println!("{} {} {}", a, b, c);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
