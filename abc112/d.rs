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

use std::collections::HashSet;
fn get_divisors(x: i64) -> Vec<i64> {
    let mut hs = HashSet::new();
    for i in 1.. {
        if x % i == 0 {
            hs.insert(i);
            hs.insert(x / i);
        }
        if i * i > x {
            break;
        }
    }
    hs.into_iter().collect()
}

fn main() {
    let v = read_vec::<i64>();
    let n = v[0];
    let m = v[1];

    let mut divisors = get_divisors(m);
    divisors.sort();
    for d in divisors.iter().rev() {
        if m >= n * d {
            println!("{}", d);
            break;
        }
    }
}
