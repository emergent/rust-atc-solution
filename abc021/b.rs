use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let v = read_vec::<u32>();
    let a = v[0];
    let b = v[1];
    let k = read::<usize>();
    let mut ps = read_vec::<u32>().into_iter().collect::<HashSet<_>>();
    ps.insert(a);
    ps.insert(b);
    let p_count = ps.iter().count();
    yn(k + 2 == p_count);
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
        println!("YES");
    } else {
        println!("NO");
    }
}
