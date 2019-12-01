fn main() {
    let n = read::<usize>();
    let v = read_vec::<u64>();
    let div: u64 = 1000000007;

    let mut dp = vec![0u64; n + 1];
    let mut kind = vec![0u64; 3];

    dp[0] = 1;
    for i in 1..n + 1 {
        let count = kind.iter().filter(|&x| *x == v[i - 1]).count();

        dp[i] = dp[i - 1] * (count as u64);
        dp[i] %= div;

        for k in 0..3 {
            if kind[k] == v[i - 1] {
                kind[k] += 1;
                break;
            }
        }
    }
    println!("{}", dp[n]);
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
        println!("1");
    } else {
        println!("0");
    }
}
