fn main() {
    let v = read_vec::<u32>();
    let n = v[0];
    let t = v[1];
    let mut last_a = read::<u32>();
    let mut ans = 0;
    for _ in 0..n - 1 {
        let a = read::<u32>();
        if a - last_a > t {
            ans += t;
        } else {
            ans += a - last_a;
        }
        last_a = a;
    }
    ans += t;
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
