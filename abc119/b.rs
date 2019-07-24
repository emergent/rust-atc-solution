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

    let mut vs = vec![0f64; n];
    for _ in 0..n {
        let v = read_vec::<String>();
        if v[1] == "JPY" {
            vs.push(v[0].parse::<f64>().unwrap());
        } else {
            vs.push(v[0].parse::<f64>().unwrap() * 380000.0);
        }
    }
    let ans = vs.iter().sum::<f64>();
    println!("{}", ans);
}
