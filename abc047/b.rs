use std::cmp::{max, min};
fn main() {
    let v = read_vec::<i32>();
    let w = v[0];
    let h = v[1];
    let n = v[2];
    let xya = read_vec2::<i32>(n as u32);

    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = w;
    let mut y2 = h;

    for p in xya {
        match p[2] {
            1 => x1 = max(p[0], x1),
            2 => x2 = min(p[0], x2),
            3 => y1 = max(p[1], y1),
            _ => y2 = min(p[1], y2),
        }
    }

    if x1 >= x2 || y1 >= y2 {
        println!("0");
    } else {
        println!("{}", (x2 - x1) * (y2 - y1));
    }
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
