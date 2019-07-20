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
    let v = read_vec::<usize>();
    let n = v[0];
    let q = v[1];
    let s = read::<String>().chars().collect::<Vec<char>>();
    let qs = read_vec2::<usize>(q as u32);

    let mut dp = vec![0u32; n+1];
    for r in 1..n {
        if s[r] == 'C' &&
            s[r - 1] == 'A' {
            dp[r] = dp[r - 1] + 1;
        } else {
            dp[r] = dp[r - 1];
        }
    }

    //println!("{:?}", dp);

    for lr in qs {
        let l = lr[0];
        let r = lr[1];
        let ans = {
            if l == 1 {
                dp[r - 1]
            } else if l >= 2 && s[l - 1] == 'C' &&
                s[l - 2] == 'A' {
                dp[r-1] - dp[l-2] - 1
            } else {
                dp[r-1] - dp[l-2]
            }
        };
        println!("{}", ans);
    }
}
