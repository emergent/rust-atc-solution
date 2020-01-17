use std::collections::HashMap;
fn main() {
    let v = read_vec::<usize>();
    let _= v[0];
    let k = v[1];
    let aa = read_vec::<usize>();
    let mut hm = HashMap::new();
    for a in aa {
        let c = hm.entry(a).or_insert(0);
        *c += 1;
    }
    let count = hm.keys().len();
    if count > k {
        let mut v = hm.values().clone().map(|&i| i).collect::<Vec<usize>>();
        v.sort();
        let mut ans = 0;
        for i in 0..count-k {
            ans += v[i];
        }
        println!("{}", ans);
    } else {
        println!("0");
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
