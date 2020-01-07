fn main() {
    let n = read::<usize>();
    let s = read::<String>().chars().collect::<Vec<_>>();
    let mut es = vec![0; n + 1];
    let mut ws = vec![0; n + 1];
    for i in 0..n {
        if s[i] == 'E' {
            es[i + 1] = es[i] + 1;
            ws[i + 1] = ws[i];
        } else {
            es[i + 1] = es[i];
            ws[i + 1] = ws[i] + 1;
        }
    }
    let mut ans = 1000000000;
    for i in 1..n + 1 {
        ans = std::cmp::min(ans, es[n] - es[i] + ws[i - 1]);
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
