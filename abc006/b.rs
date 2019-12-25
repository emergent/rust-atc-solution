fn main() {
    let n = read::<u64>();
    let mut a1 = 0;
    let mut a2 = 0;
    let mut a3 = 1;
    let div = 10007;
    for _ in 0..n - 1 {
        let tmp = (a1 + a2 + a3) % div;
        a1 = a2;
        a2 = a3;
        a3 = tmp;
    }
    println!("{}", a1 % div);
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
