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

fn main() {
    let v = read_vec::<u32>();
    let n = v[0];
    let k = v[1];

    let ans = (1..n + 1)
        .map(|i| {
            if i >= k {
                1.0f64
            } else {
                1.0f64 / 2.0f64.powi(((k as f64) / (i as f64)).log2().ceil() as i32)
            }
        })
        .fold(0.0, |acc, x| acc + x)
        / (n as f64);

    println!("{}", ans);
}
