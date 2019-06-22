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

fn c_or_d(x: u64, c: u64, d: u64) -> u64 {
    let cd = lcm(c,d);
    (x / c) + (x / d) - (x / cd)
}

fn gcd(x: u64, y: u64) -> u64 {
    if x == 0 { y } else { gcd(y % x, x) }
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn main() {
    let v = read_vec::<u64>();
    let a = v[0];
    let b = v[1];
    let c = v[2];
    let d = v[3];

    let total = b - a + 1;
    let b_cord = c_or_d(b, c, d);
    let a_cord = c_or_d(a - 1, c, d);
    let ans = total - (b_cord - a_cord);
    println!("{}", ans);
}
