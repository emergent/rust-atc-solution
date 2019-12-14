use std::collections::HashSet;
fn main() {
    let mut s = read::<String>()
        .chars()
        .collect::<HashSet<char>>()
        .into_iter()
        .collect::<Vec<_>>();
    s.sort();
    //println!("{:?}", s);
    for i in 97..122 + 1 {
        let c: char = std::char::from_u32(i as u32).unwrap();
        if !s.contains(&c) {
            println!("{}", c);
            return;
        }
    }
    println!("None");
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
