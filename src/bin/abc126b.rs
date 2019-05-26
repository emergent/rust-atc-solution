fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn is_m(i : u32) -> bool {
    i >= 1 && i <= 12
}

fn main() {
    let s = read::<String>();
    let (s1, s2) = s.split_at(2);
    let i1 = s1.parse::<u32>().unwrap();
    let i2 = s2.parse::<u32>().unwrap();

    let ans = match (is_m(i1), is_m(i2)) {
        (true, true) => "AMBIGUOUS",
        (true, false) => "MMYY",
        (false, true) => "YYMM",
        (false, false) => "NA",
    };
    println!("{}", ans);
}
