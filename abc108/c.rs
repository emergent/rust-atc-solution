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
    let v = read_vec::<i64>();
    let n = v[0];
    let k = v[1];

    if k % 2 != 0 {
        let ks = n / k;
        println!("{}", ks * ks * ks);
    } else {
        let ks = n / k;
        let ks_rem = {
            if n >= (n / k) * k + k / 2 {
                n / k + 1
            } else {
                n / k
            }
        };
        println!("{}", ks * ks * ks + ks_rem * ks_rem * ks_rem);
    }
}
