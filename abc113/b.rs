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

fn main() {
    let _n = read::<usize>();

    let v = read_vec::<f64>();
    let t = v[0];
    let a = v[1];

    let hs = read_vec::<f64>();
    let mut min = 10000000.0;
    let mut mini = 0;
    for (i, temp) in hs
        .into_iter()
        .map(|h| (a - (t - 0.006 * h)).abs())
        .enumerate()
    {
        if min > temp {
            min = temp;
            mini = i
        }
    }
    println!("{}", mini + 1);
}
