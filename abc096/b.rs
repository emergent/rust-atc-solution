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
    let mut b = read_vec::<u32>().into_iter().collect::<BinaryHeap<u32>>();
    let k = read::<u32>();

    for _ in 0..k {
        if let Some(i) = b.pop() {
            b.push(i * 2);
        }
    }
    println!("{}", b.iter().fold(0, |acc, x| acc + x));
}
