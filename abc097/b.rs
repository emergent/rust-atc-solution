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
    let n = read::<u32>();

    let mut ans = 0;
    let mut i = 2;
    if n <= 3 {
        println!("1");
    } else {
        while i * i <= n {
            let mut x = i;
            while x * i <= n {
                x *= i;
            }
            ans = std::cmp::max(ans, x);

            i += 1;
        }
        println!("{}", ans);
    }
}
