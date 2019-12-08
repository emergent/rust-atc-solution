fn main() {
    let v = read_vec::<i32>();
    let a = v[0];
    let b = v[1];
    let n = format!("{}{}", a.to_string(), b.to_string())
        .parse::<i32>()
        .unwrap();
    for i in 1..400 {
        if i * i == n {
            yn(true);
            return;
        }
    }
    yn(false);
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
