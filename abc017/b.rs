fn main() {
    let mut v = read::<String>().chars().collect::<Vec<_>>();
    loop {
        if let Some(c) = v.pop() {
            if c == 'h' {
                if let Some(c2) = v.pop() {
                    if c2 == 'c' {
                        continue;
                    } else {
                        yn(false);
                        return;
                    }
                } else {
                    yn(false);
                    return;
                }
            } else if c == 'o' || c == 'k' || c == 'u' {
                continue;
            } else {
                yn(false);
                return;
            }
        } else {
            break;
        }
    }
    yn(true);
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
        println!("YES");
    } else {
        println!("NO");
    }
}
