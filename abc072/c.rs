use std::collections::HashMap;
fn main() {
    let n = read::<usize>();
    let a = read_vec::<i32>();
    let mut hs = HashMap::new();
    for i in 0..n {
        let count = hs.entry(&a[i]).or_insert(0);
        *count += 1;
    }

    let mut ans = 0;
    for &k in hs.keys() {
        let mut c = 0;
        if let Some(&c0) = hs.get(k) {
            c += c0;
        }
        let k1 = *k + 1;
        if let Some(&c1) = hs.get(&k1) {
            c += c1;
        }
        let k2 = *k - 1;
        if let Some(&c2) = hs.get(&k2) {
            c += c2;
        }
        ans = std::cmp::max(ans, c);
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
