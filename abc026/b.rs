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
    let n = read::<usize>();
    let mut v = vec![];
    for _ in 0..n {
        v.push(read::<u32>());
    }
    v.sort();

    let mut r = 0;
    for i in 0..n {
        let j = n - i - 1;
        if i % 2 == 0 {
            r += v[j] * v[j];
        } else {
            r -= v[j] * v[j];
        }
    }
    let ans = r as f64 * std::f64::consts::PI;
    println!("{}", ans);
}
