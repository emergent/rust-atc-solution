use std::collections::HashSet;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn main() {
    let n = read::<i32>();
    let mut v = Vec::new();
    for _ in 1..n+1 {
        let d = read::<i32>();
        v.push(d);
    }

    let hs = v.into_iter().map(|i| i).collect::<HashSet<i32>>();
    let count = hs.len();
    println!("{}", count);
}
