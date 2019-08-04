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
    let v = read_vec::<usize>();
    let n = v[0];
    let m = v[1];
    let mut xs = read_vec::<i32>();
    if n >= m {
        println!("0");
        return;
    }
    xs.sort();
    //println!("{:?}", xs);
    let mut diffs = vec![0i32; m - 1];
    for i in 0..m - 1 {
        diffs[i] = xs[i + 1] - xs[i];
    }
    diffs.sort();
    //println!("{:?}", diffs);
    for _ in 0..(n - 1) {
        let _ = diffs.pop().unwrap();
    }
    let ans = diffs.iter().sum::<i32>();
    println!("{}", ans);
}
