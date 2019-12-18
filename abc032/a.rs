fn gcd(x: u64, y: u64) -> u64 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn main() {
    let a = read::<u64>();
    let b = read::<u64>();
    let n = read::<u64>();
    let l = lcm(a, b);
    for i in 1.. {
        if l * i >= n {
            println!("{}", l * i);
            break;
        }
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
