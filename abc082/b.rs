fn main() {
    let mut s = read::<String>().chars().collect::<Vec<char>>();
    let mut t = read::<String>().chars().collect::<Vec<char>>();
    s.sort();
    t.sort_by_key(|&c| -1 * (c as i32));
    let len = std::cmp::min(s.len(), t.len());

    for i in 0..len {
        if s[i] < t[i] {
            yn(true);
            return;
        } else if s[i] == t[i] {
            continue;
        } else {
            yn(false);
            return;
        }
    }
    yn(s.len() < t.len());
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
