fn main() {
    let v = read_vec::<i64>();
    let n = v[0];
    let m = v[1];
    if (n - m).abs() >= 2 {
        println!("0");
        return;
    }

    let mut ans = 1;
    let div = 1000000007;
    let x = std::cmp::min(n, m) + 1;
    for i in 1..x {
        ans *= i;
        ans %= div;
        ans *= i;
        ans %= div;
    }
    if n != m {
        ans *= x;
        ans %= div;
    } else {
        ans *= 2;
        ans %= div;
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
