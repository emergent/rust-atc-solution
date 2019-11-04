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
        println!("AC");
    } else {
        println!("WA");
    }
}

fn main() {
    let s = read::<String>().chars().collect::<Vec<char>>();
    let slen = s.len();
    let mut ans = false;
    let mut c_count = 0;
    for i in 0..slen {
        if i == 0 {
            if s[i] != 'A' {
                break;
            }
        } else if i >= 2 && i <= slen - 2 && s[i] == 'C' {
            c_count += 1;
        } else if s[i] < 'a' || 'z' < s[i] {
            yn(false);
            return;
        }
    }
    if c_count == 1 {
        ans = true;
    }
    yn(ans);
}
