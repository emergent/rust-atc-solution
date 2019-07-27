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

fn main() {
    let n = read::<usize>();

    let mut a = read_vec::<u64>();
    let b = read_vec::<u64>();

    let mut count = 0;
    for i in 0.. n {
        if a[i] > b[i] {
            count += b[i];
        } else {
            let rest = b[i] - a[i];
            count += a[i];

            if a[i+1] > rest {
                count += rest;
                a[i+1] -= rest;
            } else {
                count += a[i+1];
                a[i+1] = 0;
            }
        }
    }
    println!("{}", count);
}
