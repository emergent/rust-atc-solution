use std::collections::HashMap;
fn main() {
    let v = read_vec::<u32>();
    let _ = v[0];
    let m = v[1];

    let mut count_ac = 0;
    let mut count_pn = 0;
    let mut hm = HashMap::new();
    for _ in 0..m {
        let vv = read_vec::<String>();
        let p = vv[0].parse::<usize>().unwrap();
        let s = vv[1].clone();

        let h = hm.entry(p).or_insert(0);
        if *h >= 0 {
            if s == "AC" {
                count_pn += *h;
                count_ac += 1;
                *h = -1;
            } else {
                *h += 1;
            }
        }
    }
    println!("{} {}", count_ac, count_pn);
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
