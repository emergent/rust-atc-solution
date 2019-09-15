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

use std::collections::BinaryHeap;
fn main() {
    let v = read_vec::<u32>();
    let n = v[0] as usize;
    let m = v[1];
    let a = read_vec::<u64>();
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push(a[i]);
    }

    for _ in 0..m {
        if let Some(k) = heap.pop() {
            heap.push(k / 2);
        }
    }
    let ans = heap.iter().fold(0, |sum, x| sum + x);
    println!("{}", ans);
}
