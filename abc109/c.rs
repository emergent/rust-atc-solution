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

fn gcd(x: u32, y: u32) -> u32 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn main() {
    let v = read_vec::<u32>();
    let n = v[0] as usize;
    let x = v[1];
    let mut xs = read_vec::<u32>();
    xs.push(x);
    xs.sort();

    let mut diffs = Vec::new();
    for i in 0..n {
        diffs.push(xs[i + 1] - xs[i]);
    }

    let first = diffs[0];
    let ans = diffs.into_iter().fold(first, |acc, x| gcd(acc, x));
    println!("{}", ans);
}
