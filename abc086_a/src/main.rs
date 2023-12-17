fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let (a, b) = {
        let mut ws = s.split_whitespace();
        let a: u32 = ws.next().unwrap().parse().unwrap();
        let b: u32 = ws.next().unwrap().parse().unwrap();
        (a, b)
    };

    if a % 2 == 0 || b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
