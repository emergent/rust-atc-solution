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

fn rotate(s: String) -> String {
    let mut s1 = s.chars().collect::<Vec<char>>();
    let c1 = s1.pop().unwrap();
    s1.insert(0, c1);
    s1.into_iter().collect::<String>()
}

fn main() {
    let s = read::<String>();
    let t = read::<String>();

    let mut ss = s.clone();
    loop {
        if ss == t {
            yn(true);
            break;
        }
        ss = rotate(ss);
        if ss == s {
            yn(false);
            break;
        }
    }
}
