fn main() {
    let s = read::<String>().chars().collect::<Vec<_>>();
    let len = s.len();
    let mut n = 0;
    let mut m = len;
    for i in 0..len {
        if s[i] == 'A' {
            n = i;
            break;
        }
    }
    for j in (0..len).rev() {
        if s[j] == 'Z' {
            m = j;
            break;
        }
    }
    println!("{}", m - n + 1);
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
