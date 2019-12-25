use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let mut hm = HashMap::new();
    for _ in 0..n {
        let a = read::<String>();
        let count = hm.entry(a).or_insert(0);
        *count += 1;
    }
    let mut ans_k = "";
    let mut ans_v = 0;
    for (k, v) in &hm {
        if ans_v < *v {
            ans_k = k;
            ans_v = *v;
        }
    }
    println!("{}", ans_k);
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
