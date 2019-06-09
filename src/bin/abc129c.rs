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
    let v = read_vec::<u64>();
    let n = v[0] as usize;
    let m = v[1];
    let mut oks = vec![true; n + 1];

    for _ in 0..m {
        let ai = read::<usize>();
        oks[ai] = false;
    }

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for now in 0..n {
        for next in now + 1..std::cmp::min(n, now + 2) + 1 {
            if oks[next] {
                dp[next] += dp[now];
                dp[next] %= 1000000007;
            }
        }
    }

    println!("{}", dp[n]);
}
