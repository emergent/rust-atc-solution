fn main() {
    let v = read_vec::<u64>();
    let n = v[0];
    let m = v[1];
    if n >= 2 && m >= 2 {
        println!("{}", (n - 2) * (m - 2));
    } else if n == 1 && m >= 2 {
        println!("{}", m - 2);
    } else if m == 1 && n >= 2 {
        println!("{}", n - 2);
    } else {
        // 1x1
        println!("{}", 1);
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
