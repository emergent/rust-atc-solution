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

use std::collections::HashMap;
fn combi(n: u64, k: u64) -> u64 {
    //println!("== {} {}", n, k);
    match k {
        0 => 1,
        1 => n,
        _ => combi(n, k - 1) * (n - k + 1) / k,
    }
}
fn main() {
    let n = read::<usize>();
    let mut v = HashMap::new();
    let mut count: u64 = 0;
    for _ in 0..n {
        let mut s = read::<String>().chars().collect::<Vec<char>>();
        s.sort();
        let s2 = s.into_iter().collect::<String>();

        let count = v.entry(s2).or_insert(0);
        *count += 1;
    }

    let mut ans = 0;
    for &val in v.values() {
        if val >= 2 {
            ans += combi(val, 2);
        }
    }
    println!("{}", ans);
}
