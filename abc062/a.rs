fn main() {
    let v = read_vec::<u32>();
    let x = v[0];
    let y = v[1];

    let g1 = vec![1, 3, 5, 7, 8, 10, 12];
    let g2 = vec![4, 6, 9, 11];
    let g3 = vec![2];

    yn(g1.contains(&x) && g1.contains(&y)
        || g2.contains(&x) && g2.contains(&y)
        || g3.contains(&x) && g3.contains(&y));
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
