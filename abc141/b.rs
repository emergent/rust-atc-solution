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
    let s = read::<String>().chars().collect::<Vec<char>>();
    let mut ans = true;
    for (i, &c) in s.iter().enumerate() {
        if (i + 1) % 2 != 0 {
            if c == 'R' || c == 'U' || c == 'D' {
                continue;
            } else {
                ans = false;
                break;
            }
        } else {
            if c == 'L' || c == 'U' || c == 'D' {
                continue;
            } else {
                ans = false;
                break;
            }
        }
    }
    yn(ans);
}
