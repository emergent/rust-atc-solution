fn main() {
    let n = read::<usize>();
    let mut v = vec![];
    for _ in 0..n {
        let xy = read_vec::<f64>();
        v.push((xy[0], xy[1]));
    }
    let mut distances = 0.0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            distances += ((v[i].0 - v[j].0).powi(2) + (v[i].1 - v[j].1).powi(2)).sqrt();
        }
    }
    println!("{}", distances * 2.0 / (n as f64));
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
