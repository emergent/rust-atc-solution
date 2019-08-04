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
    if n < 10 {
        println!("{}", n);
    } else if n < 100 {
        println!("{}", 9);
    } else if n < 1000 {
        println!("{}", n - 99 + 9);
    } else if n < 10000 {
        println!("{}", 909);
    } else if n < 100000 {
        println!("{}", n - 9999 + 909);
    } else {
        println!("{}", 90909);
    }
}
