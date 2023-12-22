fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s = s.trim().to_string();

    let strings = vec!["dream", "dreamer", "erase", "eraser"];

    while !s.is_empty() {
        let mut matched = false;
        for string in &strings {
            if s.ends_with(string) {
                s = s[..s.len() - string.len()].to_string();
                matched = true;
                break;
            }
        }
        if !matched {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
