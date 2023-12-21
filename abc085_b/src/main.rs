use std::collections::HashSet;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i32>().unwrap();

    let mut rice_cakes: HashSet<i32> = HashSet::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        rice_cakes.insert(s.trim().parse::<i32>().unwrap());
    }

    println!("{}", rice_cakes.len());
}
