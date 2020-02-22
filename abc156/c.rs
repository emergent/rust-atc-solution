fn main() {
    let n = read::<usize>();
    let mut xs = read_vec::<i64>();
    xs.sort();

    let x0 = xs[0];
    let x1 = xs[n - 1];
    let mut ans = 1000000000000000;
    for p in x0..x1 + 1 {
        let a = xs.iter().map(|&x| (x - p) * (x - p)).sum::<i64>();
        ans = std::cmp::min(ans, a);
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
