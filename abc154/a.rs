fn main() {
    let v = read_vec::<String>();
    let s = v[0].clone();
    let t = v[1].clone();
    let v2 = read_vec::<u32>();
    let a = v2[0];
    let b = v2[1];
    let u = read::<String>();

    if u == s {
        println!("{} {}", a - 1, b);
    } else {
        println!("{} {}", a, b - 1);
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
