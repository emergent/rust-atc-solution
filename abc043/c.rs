fn main() {
    let n = read::<i64>();
    let v = read_vec::<i64>();
    let sum = v.iter().sum::<i64>();

    if sum % n == 0 {
        let mut ans = 0;
        let ave = sum / n;
        for a in v {
            ans += (a - ave) * (a - ave);
        }
        println!("{}", ans);
    } else {
        let ave1 = sum / n;
        let ave2 = sum / n + 1;
        let mut ans1 = 0;
        let mut ans2 = 0;
        for a in v {
            ans1 += (a - ave1) * (a - ave1);
            ans2 += (a - ave2) * (a - ave2);
        }
        println!("{}", std::cmp::min(ans1, ans2));
    }
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
