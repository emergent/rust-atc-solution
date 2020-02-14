fn main() {
    let v = read_vec::<i32>();
    let (sx, sy) = (v[0], v[1]);
    let (tx, ty) = (v[2], v[3]);
    let y = ty - sy;
    let x = tx - sx;

    for (t, c) in vec![
        (y, 'U'),
        (x, 'R'),
        (y, 'D'),
        (x, 'L'),
        (1, 'L'),
        (y + 1, 'U'),
        (x + 1, 'R'),
        (1, 'D'),
        (1, 'R'),
        (y + 1, 'D'),
        (x + 1, 'L'),
        (1, 'U'),
    ] {
        for _ in 0..t {
            print!("{}", c);
        }
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
