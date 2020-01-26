use std::collections::VecDeque;
fn main() {
    let n = read::<usize>();
    let aa = read_vec::<u32>();
    let mut b = VecDeque::new();

    for i in 0..n {
        if (i % 2 == 0 && n % 2 == 0) || (i % 2 != 0 && n % 2 != 0) {
            b.push_back(aa[i]);
        } else {
            b.push_front(aa[i]);
        }
    }
    println!(
        "{}",
        b.into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
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
