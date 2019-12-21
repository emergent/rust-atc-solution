fn main() {
    let mut s = read::<String>();
    let n = read::<usize>();
    for _ in 0..n {
        let lr = read_vec::<usize>();
        let l = lr[0];
        let r = lr[1];

        let s0 = s[0..l - 1].to_string();
        let s1 = s[l - 1..r].chars().rev().collect::<String>();
        let s2 = s[r..].to_string();

        s = vec![s0, s1, s2].join("");
    }
    println!("{}", s);
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
