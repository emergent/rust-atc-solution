fn main() {
    let n = read::<usize>();
    let s = read::<String>().chars().collect::<Vec<_>>();
    let mut max = 0;
    let mut x = 0;
    for c in s {
        if c == 'I' {
            x += 1;
            max = std::cmp::max(x, max);
        } else if c == 'D' {
            x -= 1;
        }
    }
    println!("{}", max);
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
