fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

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

fn main() {
    let n = read::<usize>();
    let vs = read_vec::<i32>();
    let costs = read_vec::<i32>();

    let mut ans = 0;
    for i in 0..n {
        let p = vs[i] - costs[i];
        if p > 0 {
            ans += p;
        }
    }
    println!("{}", ans);
}
