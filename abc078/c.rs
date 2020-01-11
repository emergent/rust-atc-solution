fn main() {
    let v = read_vec::<u32>();
    let n = v[0];
    let m = v[1];
    if m == 0 {
        println!("{}", n * 100);
        return;
    }
    let pow2m = {
        let mut mm = m;
        let mut a = 1;
        while mm > 0 {
            a *= 2;
            mm -= 1;
        }
        a
    };
    let ans = (1900 * m + 100 * (n - m)) * pow2m;
    println!("{}", ans);
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
