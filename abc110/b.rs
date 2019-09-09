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
        println!("War");
    } else {
        println!("No War");
    }
}

fn main() {
    let v = read_vec::<i32>();
    let x = v[2];
    let y = v[3];

    let mut xs = read_vec::<i32>();
    xs.push(x);
    let mut ys = read_vec::<i32>();
    ys.push(y);
    let xmax = xs.iter().max().unwrap();
    let ymin = ys.iter().min().unwrap();
    yn(!(xmax < ymin))
}
