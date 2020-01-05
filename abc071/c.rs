fn main() {
    let _ = read::<usize>();
    let mut a = read_vec::<u64>();
    a.sort();
    let mut b = 0;
    let mut v = vec![];
    while let Some(x) = a.pop() {
        if x != b {
            b = x;
        } else {
            b = 0;
            v.push(x);
            if v.len() == 2 {
                break;
            }
        }
    }
    if v.len() < 2 {
        println!("{}", 0);
    } else {
        println!("{}", v[0] * v[1]);
    }
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
