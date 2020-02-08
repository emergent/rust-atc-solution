use std::cmp::{max, min};
fn keta(x: u64) -> u64 {
    let mut x = x;
    let mut keta = 0;
    if x == 0 {
        return 0;
    }
    while x > 0 {
        keta += 1;
        x /= 10;
    }
    keta
}

fn main() {
    let n = read::<u64>();
    let mut max_i = 0;
    for i in 1..n + 1 {
        if i * i >= n {
            max_i = i;
            break;
        }
    }

    let mut ans = 10;
    for i in 1..max_i + 1 {
        if n % i == 0 {
            ans = min(ans, max(keta(i), keta(n / i)));
        }
    }
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
