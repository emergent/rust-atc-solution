fn main() {
    let v = read_vec::<i32>();
    let ta = (v[0], v[1]);
    let tb = (v[2], v[3]);
    let t = v[4];
    let vv = v[5];
    let n = read::<usize>();

    for _ in 0..n {
        let g = read_vec::<i32>();

        let t2x = (g[0] - ta.0).abs() as f64;
        let t2y = (g[1] - ta.1).abs() as f64;
        let t3x = (tb.0 - g[0]).abs() as f64;
        let t3y = (tb.1 - g[1]).abs() as f64;

        if (t2x * t2x + t2y * t2y).sqrt() + (t3x * t3x + t3y * t3y).sqrt() <= (vv * t) as f64 {
            yn(true);
            return;
        }
    }
    yn(false);
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
        println!("YES");
    } else {
        println!("NO");
    }
}
