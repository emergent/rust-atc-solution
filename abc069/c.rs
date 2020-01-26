fn main() {
    let n = read::<usize>();
    let v = read_vec::<u32>();
    let mut b4 = 0;
    let mut b2 = 0;
    for a in v {
        if a % 4 == 0 {
            b4 += 1;
        } else if a % 2 == 0 {
            b2 += 1;
        }
    }
    yn(b4 >= n / 2 || b2 >= n - 2 * b4);
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
