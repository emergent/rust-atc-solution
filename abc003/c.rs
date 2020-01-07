fn main() {
    let v = read_vec::<usize>();
    let n = v[0];
    let k = v[1];
    let mut rs = read_vec::<u32>();
    rs.sort();
    let rs2 = rs[n - k..].into_iter().collect::<Vec<_>>();
    let mut c: f64 = 0.0;
    for &r in rs2 {
        c = (c + r as f64) / 2.0;
    }
    println!("{}", c);
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
