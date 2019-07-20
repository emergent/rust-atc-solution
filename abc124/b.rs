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
    let n: usize = read();
    let v = read_vec::<u32>();

    let mut ans = 1;
    let mut maxi = v[0];
    for i in 1..n {
        if v[i] >= maxi {
            ans += 1;
        }
        maxi = std::cmp::max(maxi, v[i]);
    }
    println!("{}", ans);
}
