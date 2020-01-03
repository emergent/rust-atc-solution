use std::cmp::*;
const MOD: usize = 1000000007;

// implementation of combination with mod
// ref: https://qiita.com/ofutonfuton/items/92b1a6f4a7775f00b6ae
fn mod_power(x: usize, n: usize) -> usize {
    let mut ret = 1;
    let mut xx = x;
    let mut nn = n;
    while nn > 0 {
        if nn & 1 > 0 {
            ret = ret * xx % MOD;
        }
        xx = xx * xx % MOD;
        nn = nn >> 1;
    }
    ret
}

fn combinations(n: usize, k: usize) -> usize {
    if n == 0 && k == 0 {
        return 1;
    }
    if n < k {
        return 0;
    }

    let mut fac = vec![1usize; n + 1];
    let mut ifac = vec![1usize; n + 1];
    for i in 0..n {
        fac[i + 1] = fac[i] * (i + 1) % MOD;
        ifac[i + 1] = ifac[i] * mod_power(i + 1, MOD - 2) % MOD;
    }
    let tmp = ifac[n - k] * ifac[k] % MOD;
    tmp * fac[n] % MOD
}

fn main() {
    let v = read_vec::<usize>();
    let x = v[0];
    let y = v[1];

    if (x + y) % 3 != 0 || max(x, y) / min(x, y) > 2 {
        println!("0");
        return;
    }

    let nx = (x + y) / 3;
    let ny = (nx + y - x) / 2; // ((x - y)*(-1)+nx) / 2
    let ans = combinations(nx, ny) % MOD;
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
