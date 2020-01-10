fn main() {
    let n = read::<usize>();
    let mut v = read_vec::<i64>();
    v.insert(0, 0);
    v.push(0);
    let mut diffs = vec![];
    let mut diffs2 = vec![];
    for i in 0..n + 1 {
        diffs.push((v[i] - v[i + 1]).abs());
        if i != n {
            diffs2.push((v[i] - v[i + 2]).abs());
        }
    }
    let mut all = diffs.iter().sum::<i64>();
    for i in 0..n {
        println!("{}", all - diffs[i] - diffs[i + 1] + diffs2[i]);
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
