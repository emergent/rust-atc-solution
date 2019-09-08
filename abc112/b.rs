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
    let v = read_vec::<u32>();
    let n = v[0];
    let t = v[1];

    let ct = read_vec2::<u32>(n)
        .into_iter()
        .filter(|ref v| v[1] <= t)
        .map(|v| v[0])
        .collect::<Vec<u32>>();
    let ans = ct.into_iter().min();
    match ans {
        None => println!("TLE"),
        Some(a) => println!("{}", a),
    }
}
