use std::cmp::{max, min};
fn main() {
    let v = read_vec::<i32>();
    let a = v[0];
    let b = v[1];
    let c = v[2];
    let x = v[3];
    let y = v[4];

    let mut ans = vec![];
    ans.push(min(x, y) * 2 * c + max(x - y, 0) * a + max(y - x, 0) * b);
    ans.push(a * x + b * y);
    ans.push(max(x, y) * c * 2);
    println!("{}", ans.iter().min().unwrap());
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
        println!("Yes");
    } else {
        println!("No");
    }
}
