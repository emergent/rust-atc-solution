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
    let b = read::<u32>();
    let c = read::<u32>();
    let d = read::<u32>();
    let e = read::<u32>();

    let v = vec![a, b, c, d, e];
    let mut min = 10;
    for i in &v {
        let i10 = i % 10;
        if i10 > 0 && i10 < min {
            min = i10;
        }
    }
    let time_min = v
        .iter()
        .map(|&i| if i % 10 == 0 { i } else { (i / 10 + 1) * 10 })
        .fold(0, |sum, x| sum + x)
        - (10 - min);

    println!("{}", time_min);
}
