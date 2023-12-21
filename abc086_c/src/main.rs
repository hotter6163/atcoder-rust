struct Point {
    t: i32,
    x: i32,
    y: i32,
}

impl Point {
    fn new(t: i32, x: i32, y: i32) -> Point {
        Point { t, x, y }
    }

    /// Determines if it's possible to move to the next point.
    /// The movement is possible if the total time difference is greater than or equal to
    /// the sum of the differences in x and y coordinates, and both have the same parity.
    fn can_move(&self, next: &Point) -> bool {
        let dt = next.t - self.t;
        let dx = (next.x - self.x).abs();
        let dy = (next.y - self.y).abs();
        let d = dx + dy;
        dt >= d && dt % 2 == d % 2
    }
}

fn read<T: std::str::FromStr>() -> Result<T, Box<dyn std::error::Error>> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    s.trim().parse().map_err(|_| "Parse error".into())
}

fn read_line<T: std::str::FromStr>() -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s)?;
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().map_err(|_| "Parse error".into()))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = read::<usize>()?;
    let mut points = Vec::new();
    points.push(Point::new(0, 0, 0));
    for _ in 0..n {
        let line = read_line::<i32>()?;
        points.push(Point::new(line[0], line[1], line[2]));
    }

    let mut can_move = true;
    for i in 0..n {
        if !points[i].can_move(&points[i + 1]) {
            can_move = false;
            break;
        }
    }

    println!("{}", if can_move { "Yes" } else { "No" });
    Ok(())
}
