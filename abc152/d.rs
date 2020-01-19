fn top(x: usize) -> usize {
    let mut top = 0;
    let mut y = x;
    while y > 0 {
        top = y;
        y /= 10;
    }
    top
}

use std::collections::HashMap;
fn main() {
    let n = read::<usize>();
    let mut hm = HashMap::new();
    for i in 1..n + 1 {
        let t = top(i);
        let b = i % 10;
        let c = hm.entry((t, b)).or_insert(0);
        *c += 1;
    }

    let mut count = 0;
    for i in 0..10 {
        for j in 0..10 {
            let cij = *hm.entry((i, j)).or_insert(0);
            let cji = *hm.entry((j, i)).or_insert(0);
            count += cij * cji;
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
