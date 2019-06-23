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
    let a = read::<u32>();
    let _ = read::<u32>();
    let _ = read::<u32>();
    let _ = read::<u32>();
    let e = read::<u32>();
    let k = read::<u32>();

    if e - a <= k {
        println!("Yay!");
    } else {
        println!(":(");
    }
}
