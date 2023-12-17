fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let a = s.trim().parse::<i32>().unwrap();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let (b, c) = {
        let mut ws = s.trim().split_whitespace();
        (
            ws.next().unwrap().parse::<i32>().unwrap(),
            ws.next().unwrap().parse::<i32>().unwrap(),
        )
    };

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string();

    println!("{} {}", a + b + c, s);
}
