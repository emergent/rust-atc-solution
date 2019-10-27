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

use std::f64::consts::PI;
fn main() {
    let v = read_vec::<f64>();
    let a = v[0];
    let b = v[1];
    let x = v[2];
    let t = x / a;

    if t >= (a * b) / 2.0 {
        let c = (2.0 * ((a * b) - t)) / a;
        let ans = (c / a).atan();
        println!("{}", ans * 360.0 / (2.0 * PI));
    } else {
        let c = t * 2.0 / b;
        let ans = (b / c).atan();
        println!("{}", ans * 360.0 / (2.0 * PI));
    }
}
