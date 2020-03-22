use std::collections::HashMap;

fn main() {
    let _n = read::<usize>();
    let v = read_vec::<u64>();
    let mut hm = HashMap::new();

    for &vi in &v {
        let c = hm.entry(vi).or_insert(0);
        *c += 1;
    }

    let total: u64 = {
        let mut t = 0;
        for (_, v) in &hm {
            t += v * (v - 1) / 2;
        }
        t
    };

    for &vi in &v {
        let c = hm.entry(vi).or_insert(0);
        if *c >= 2 {
            println!(
                "{}",
                total - (*c * (*c - 1) / 2) + ((*c - 1) * (*c - 2) / 2)
            );
        } else {
            println!("{}", total);
        }
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
