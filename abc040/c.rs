fn main() {
    let n = read::<usize>();
    let aa = read_vec::<i64>();

    let mut ansv = vec![1000000000000000i64; n];
    ansv[0] = 0;
    for i in 0..n - 1 {
        if i < n - 2 {
            ansv[i + 2] = std::cmp::min(ansv[i + 2], ansv[i] + (aa[i + 2] - aa[i]).abs());
        }
        ansv[i + 1] = std::cmp::min(ansv[i + 1], ansv[i] + (aa[i + 1] - aa[i]).abs());
        //println!("{:?}", ansv);
    }
    println!("{}", ansv[n - 1])
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
