use std::collections::*;
fn main() {
    let _ = read::<usize>();
    let v = read_vec::<u32>();
    let mut hm = HashMap::new();
    let mut over3200 = 0;

    for a in v {
        if a >= 3200 {
            over3200 += 1;
        }
        let color = match a {
            1...399 => 0,
            400...799 => 1,
            800...1199 => 2,
            1200...1599 => 3,
            1600...1999 => 4,
            2000...2399 => 5,
            2400...2799 => 6,
            2800...3199 => 7,
            _ => 8,
        };
        let count = hm.entry(color).or_insert(0);
        *count += 1;
    }
    let colors = hm.keys().count();
    if colors == 1 && over3200 > 0 {
        println!("1 {}", over3200);
    } else if over3200 > 0 {
        println!("{} {}", colors - 1, colors - 1 + over3200);
    } else {
        println!("{} {}", colors, colors);
    }
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
