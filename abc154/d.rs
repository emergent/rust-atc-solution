fn main() {
    let v = read_vec::<usize>();
    let n = v[0];
    let k = v[1];
    let ps = read_vec::<u32>();
    let exps = ps
        .iter()
        .map(|&p| (p + 1) as f64 / 2.0)
        .collect::<Vec<f64>>();
    let mut s = vec![0.0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + exps[i];
    }
    //println!("{:?}", s);

    let mut ans = 0.0;
    for i in 0..n - k + 1 {
        let c = s[i + k] - s[i];
        //println!("{} {}", i, c);
        ans = f64::max(ans, c);
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
