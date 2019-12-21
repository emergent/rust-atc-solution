fn main() {
    let v = read_vec::<i32>();
    let n = v[0];
    let s = v[1];
    let t = v[2];
    let mut w = read::<i32>();

    let mut count = 0;
    if s <= w && w <= t {
        count += 1;
    }

    for _ in 2..n + 1 {
        w += read::<i32>();
        if s <= w && w <= t {
            count += 1;
        }
    }
    println!("{}", count);
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
