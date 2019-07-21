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

fn divisor1(x: u64) -> (u64, u64, u64, u64) {
    let j = (x as f64).sqrt().ceil() as u64;
    let x1 = j;
    let y2 = j;
    let x2 = j * j - x;

    if x2 == 0 {
        return (j, 0, 0, j);
    }

    let mut i = (x2 as f64).sqrt().floor() as u64;
    while i > 0 {
        if x2 % i == 0 && i <= 1000000000 {
            let y = x2 / i;
            if y <= 1000000000 {
                return (x1, y, i, y2);
            }
        }
        i -= 1;
    }
    (0, 0, 0, 0)
}

fn main() {
    let s = read::<u64>();
    let xys = divisor1(s);

    println!("{} {} {} {} 0 0", xys.0, xys.1, xys.2, xys.3);
}
