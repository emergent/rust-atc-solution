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
    let mut v = Vec::new();
    let mut ans = true;
    for i in 0..n {
        let s = read::<String>();
        if i != 0 {
            let last: String = v.pop().unwrap();
            if last.chars().last().unwrap() != s.chars().nth(0).unwrap() {
                ans = false;
                break;
            }
            v.push(last);
            if v.contains(&s) {
                ans = false;
                break;
            }
        }
        v.push(s);
    }
    yn(ans);
}
