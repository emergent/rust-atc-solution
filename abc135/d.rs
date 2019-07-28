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

fn main() {
    let s = read::<String>().chars().collect::<Vec<char>>();
    let n = s.len() as usize;

    let mut dp = vec![vec![0i64; 13]; n + 1];

    dp[0][0] = 1;
    for i in 0..n {
        if s[i] == '?' {
            for j in 0..10 {
                for k in 0..13 {
                    dp[i + 1][(k * 10 + j) % 13] += dp[i][k]
                }
            }
        } else {
            let c = s[i] as usize - '0' as usize;
            for k in 0..13 {
                dp[i + 1][(k * 10 + c) % 13] += dp[i][k];
            }
        }

        for j in 0..13 {
            dp[i + 1][j] %= 1000000007;
        }
    }
    println!("{}", dp[n][5]);
}
