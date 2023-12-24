#[macro_export]
macro_rules! read_tuple {
    ($($t:ty),*) => {
        {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let mut iter = s.trim().split_whitespace();
            (
                $(
                    iter.next().unwrap().parse::<$t>().unwrap(),
                )*
            )
        }
    };
}
