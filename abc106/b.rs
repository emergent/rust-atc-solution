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

fn dividers(n: u32) -> u32 {
    let mut count = 0;
    for i in 1..n + 1 {
        if n % i == 0 {
            count += 1
        }
    }
    count
}

fn main() {
    let n = read::<u32>();
    let mut count = 0;
    for i in 1..n + 1 {
        if i % 2 != 0 && dividers(i) == 8 {
            count += 1;
        }
    }
    println!("{}", count);
}
