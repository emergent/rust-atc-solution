fn main() {
    let n = read::<usize>();
    let a = read_vec::<u32>();
    let mut l = 0;
    let mut r = 0;

    let mut count = 0;
    loop {
        if l == r {
            count += 1;
            if r < n - 1 {
                r += 1;
            } else {
                break;
            }
        } else {
            if a[r] > a[r - 1] {
                count += r - l + 1;
                if r < n - 1 {
                    r += 1;
                } else {
                    break;
                }
            } else {
                l = r;
            }
        }
    }
    println!("{}", count);
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
