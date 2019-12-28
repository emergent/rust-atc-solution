fn main() {
    let v = read_vec::<i32>();
    let _a = (0, 0);
    let b = (v[2] - v[0], v[3] - v[1]);
    let c = (v[4] - v[0], v[5] - v[1]);

    let ans = (b.0 * c.1 - b.1 * c.0).abs() as f64 / 2.0;
    println!("{}", ans);
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
