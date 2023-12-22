fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let count = s.chars().filter(|&c| c == '1').count();
    println!("{}", count);
}
