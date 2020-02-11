use std::collections::{HashMap, HashSet};
fn main() {
    let n = read::<usize>();
    let mut v = vec![];
    for _ in 0..n {
        v.push(read::<u32>());
    }
    let mut v2 = v
        .clone()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    v2.sort();
    let mut hm = HashMap::new();
    for (i, &val) in v2.iter().enumerate() {
        hm.insert(val, i);
    }
    let ansv = v
        .into_iter()
        .map(|a| hm.get(&a).unwrap())
        .collect::<Vec<_>>();
    for a in ansv {
        println!("{}", a);
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
