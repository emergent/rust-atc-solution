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
    let n = read::<u64>();
    let a = read::<u64>();
    let b = read::<u64>();
    let c = read::<u64>();
    let d = read::<u64>();
    let e = read::<u64>();

    let v = vec![a, b, c, d, e];
    let min = *v.iter().min().unwrap();
    if n <= min {
        println!("5");
    } else {
        let time = if n % min == 0 { n / min } else { n / min + 1 };
        println!("{}", time + 4);
    }
}
