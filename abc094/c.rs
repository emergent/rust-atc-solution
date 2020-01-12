fn main() {
    let n = read::<usize>();
    let xs = read_vec::<u32>();
    let mut ys = xs.clone();
    ys.sort();
    let m1 = ys[n / 2 - 1];
    let m2 = ys[n / 2];

    for i in 0..n {
        if xs[i] <= m1 {
            println!("{}", m2);
        } else {
            println!("{}", m1);
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
