fn main() {
    let mut s = read_string();
    let mut start_index = None;

    // ABCという文字列が一番最初に現れるインデックスを取得
    while let Some(i) = get_first_index(&s, "ABC", start_index) {
        start_index = if i > 3 { Some(i - 3) } else { None };
        s = s[..i].to_string() + &s[i + 3..];
    }

    println!("{}", s);
}

fn read_string() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_first_index(s: &str, target: &str, start_index: Option<usize>) -> Option<usize> {
    if s.len() < 3 {
        return None;
    }

    let start_index = start_index.unwrap_or(0);
    for i in start_index..s.len() - target.len() + 1 {
        if &s[i..i + target.len()] == target {
            return Some(i);
        }
    }
    None
}
