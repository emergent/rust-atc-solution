fn main() {
    let n = read::<u32>();
    if n % 3 == 0 {
        yn(true);
        return;
    } else {
        let mut m = n;
        while m > 0 {
            if m % 10 == 3 {
                yn(true);
                return;
            }
            m /= 10;
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
        println!("YES");
    } else {
        println!("NO");
    }
}
