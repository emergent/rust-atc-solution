fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

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

fn main() {
    let v = read_vec::<usize>();
    let n = v[0];
    let k = v[1];
    let s = read::<String>();

    let s2 = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == k - 1 {
                (c as u8 + 32) as char
            } else {
                c
            }
        })
        .collect::<String>();
    println!("{}", s2);
}
