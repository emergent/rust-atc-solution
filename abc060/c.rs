fn main() {
    let v = read_vec::<u64>();
    let n = v[0];
    let t = v[1];
    let ts = read_vec::<u64>();
    let mut ts2 = vec![];
    for i in 1..n as usize {
        ts2.push(ts[i] - ts[i - 1]);
    }
    let mut ans = 0;
    for t1 in ts2 {
        if t1 > t {
            ans += t;
        } else {
            ans += t1;
        }
    }
    println!("{}", ans + t);
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
