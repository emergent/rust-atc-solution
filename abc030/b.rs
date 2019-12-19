fn main() {
    let v = read_vec::<u32>();
    let n = (v[0] % 12) as f32;
    let m = v[1] as f32;

    let n_deg = m * 0.5 + n * 30.0;
    let m_deg = m * 6.0;
    let mut deg = (n_deg - m_deg).abs();

    if deg >= 180.0 {
        deg = 360.0 - deg;
    }
    println!("{}", deg);
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
