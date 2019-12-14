use std::collections::HashSet;

fn main() {
    let s = read::<String>().chars().collect::<Vec<_>>();
    let l1 = s.len();
    let hs = s.into_iter().collect::<HashSet<_>>();
    let l2 = hs.len();
    yn(l1 == l2);
}

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
#[allow(dead_code)]
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
#[allow(dead_code)]
fn yn(result: bool) {
    if result {
        println!("yes");
    } else {
        println!("no");
    }
}
