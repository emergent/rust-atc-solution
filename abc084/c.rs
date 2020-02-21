use std::cmp::max;
fn main() {
    let n = read::<usize>();

    let mut ansv = vec![];
    for i in 0..n-1 {
        let v = read_vec::<u32>();
        let c = v[0];
        let s = v[1];
        let f = v[2];

        for j in 0..i {
            ansv[j] = ((max(s, ansv[j]) + f - 1) / f) * f + c;
        }

        ansv.push(s+c);
    }

    for a in ansv {
        println!("{}", a);
    }
    println!("0");
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
