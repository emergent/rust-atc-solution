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
    let n = read::<usize>();
    let s = read::<String>();

    let mut maxlen = 0;
    for i in 0..n - 2 {
        for j in i + 1 + maxlen..n {
            if n - j < j - i || n - j < maxlen {
                break;
            }
            let sub = &s[i..j];
            if sub.len() > maxlen {
                let rest = &s[j..];
                if rest.contains(&sub) {
                    maxlen = sub.len();
                } else {
                    break;
                }
            }
        }
    }
    println!("{}", maxlen);
}
