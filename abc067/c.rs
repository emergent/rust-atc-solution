fn main() {
    let n = read::<usize>();
    let aa = read_vec::<i64>();
    let mut v = vec![0i64; n + 1];
    for i in 0..n {
        v[i + 1] = v[i] + aa[i];
    }

    let mut ans = 1152921504606846976;
    for i in 1..n {
        let xy = ((v[i] - v[0]) - (v[n] - v[i])).abs();
        ans = std::cmp::min(ans, xy);
    }
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
