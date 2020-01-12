fn main() {
    let v = read_vec::<i32>();
    let n = v[0];
    let k = v[1];
    let m = v[2];
    let a = read_vec::<i32>();

    let mut x = m * n;
    for ai in a {
        x -= ai;
    }
    x = std::cmp::max(x, 0);

    if x > k {
        println!("-1");
    } else {
        println!("{}", x);
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
