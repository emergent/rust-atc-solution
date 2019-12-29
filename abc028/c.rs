use std::collections::HashSet;
fn main() {
    let v = read_vec::<u32>();
    let mut hs = HashSet::new();

    for i in 0..3 {
        for j in i + 1..4 {
            for k in j + 1..5 {
                hs.insert(v[i] + v[j] + v[k]);
            }
        }
    }
    let mut v = hs.into_iter().collect::<Vec<_>>();
    v.sort();
    let _ = v.pop().unwrap();
    let _ = v.pop().unwrap();
    let ans = v.pop().unwrap();
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
