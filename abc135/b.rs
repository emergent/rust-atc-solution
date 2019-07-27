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
        println!("YES");
    } else {
        println!("NO");
    }
}

fn main() {
    let n = read::<usize>();

    let v = read_vec::<u32>();
    let mut v2 = v.clone();
    v2.sort();

    let mut diff_count = 0;
    for i in 0..n {
        if v[i] != v2[i] {
            diff_count += 1;
        }
    }
    yn(diff_count == 0 || diff_count == 2)
}
