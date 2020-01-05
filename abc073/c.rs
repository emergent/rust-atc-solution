use std::collections::HashMap;
fn main() {
    let n = read::<usize>();
    let mut hm = HashMap::new();
    for _ in 0..n {
        let a = read::<u32>();
        let c = hm.entry(a).or_insert(0);
        *c += 1;
    }
    let mut count = 0;
    for &v in hm.values() {
        if v % 2 != 0 {
            count += 1;
        }
    }
    println!("{}", count);
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
