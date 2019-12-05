use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    let n = read::<usize>();
    for _ in 0..n {
        let b = read::<String>();
        let count = hm.entry(b).or_insert(0);
        *count += 1;
    }
    let m = read::<usize>();
    for _ in 0..m {
        let r = read::<String>();
        let count = hm.entry(r).or_insert(0);
        *count -= 1;
    }
    println!("{}", std::cmp::max(*hm.values().max().unwrap(), 0));
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
