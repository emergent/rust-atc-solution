use std::cmp::*;

fn main() {
    let n = read::<usize>();
    let a1 = read_vec::<u32>();
    let a2 = read_vec::<u32>();

    let mut maxcount = 0;
    for i in 0..n {
        let mut count = 0;
        for j in 0..i + 1 {
            count += a1[j];
        }
        for k in i..n {
            count += a2[k];
        }
        maxcount = max(maxcount, count);
    }
    println!("{}", maxcount);
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
