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

fn even(x: u64) -> bool {
    x % 2 == 0
}

fn odd(x: u64) -> bool {
    x % 2 == 1
}

fn main() {
    let v = read_vec::<u64>();
    let a = v[0];
    let b = v[1];

    if a == 0 && b == 0 {
        println!("0"); return;
    } else if a == b {
        println!("{}", a); return;
    }

    let mut ans = 0;
    if odd(a) && odd(b) {
        ans = a ^ (((b - a) / 2) % 2);
    } else if odd(a) && even(b) {
        ans = a ^ b ^ (((b - a - 1) / 2) % 2);
    } else if even(a) && odd(b) {
        ans = ((b - a + 1) / 2) % 2;
    } else if even(a) && even(b) {
        ans = b ^ (((b - a) / 2) % 2);
    }
    println!("{}", ans);
}
