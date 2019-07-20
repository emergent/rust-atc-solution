fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

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

fn main() {
    //let n = read::<u32>();
    let v = read_vec::<u64>();
    let w = v[0];
    let h = v[1];
    let x = v[2];
    let y = v[3];

    let half = (w * h) as f64 / 2.0;
    if 2 * x == w && 2 * y == h {
        println!("{} 1", half);
    } else {
        println!("{} 0", half);
    }
}
