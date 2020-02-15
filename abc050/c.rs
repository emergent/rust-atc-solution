fn main() {
    let n = read::<usize>();
    let mut v = read_vec::<u32>();
    v.sort();

    let mut e = 0;
    if n % 2 != 0 {
        for i in 0..n {
            //println!("{} {} {}", i, v[i], (i as u32 + 1) / 2 * 2);
            if (i as u32 + 1) / 2 * 2 != v[i] {
                println!("0");
                return;
            }
        }
        e = (v.len() - 1) / 2;
    } else {
        for i in 0..n {
            //println!("{} {} {}", i, v[i], (i as u32 + 2) / 2 * 2);
            if (i as u32 + 2) / 2 * 2 - 1 != v[i] {
                println!("0");
                return;
            }
        }
        e = v.len() / 2;
    }
    let mut ans = 1;
    for _ in 0..e {
        ans *= 2;
        ans %= 1000000007;
    }
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
