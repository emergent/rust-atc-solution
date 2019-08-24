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
    let k = v[1] as u64;
    let div = 1000000007;
    let a = read_vec::<u64>();

    let mut at_count = 0;
    let mut top_count = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if a[i] > a[j] {
                at_count += 1;
            } else if a[i] < a[j] {
                top_count += 1;
            }
        }
    }

    let ks: u64 = k * (k + 1) / 2;
    let ts: u64 = (k - 1) * k / 2;
    let mut ans: u64 = 0;
    for _ in 0..at_count {
        ans += ks;
        ans %= div;
    }
    for _ in 0..top_count {
        ans += ts;
        ans %= div;
    }
    println!("{}", ans);
}
