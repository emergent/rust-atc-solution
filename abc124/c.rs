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
    let s = read::<String>().chars().collect::<Vec<char>>();
    let n = s.len();

    let mut t0_count = 0;
    let mut t1_count = 0;

    for i in 0..n {
        if i % 2 == 0 {
            if s[i] == '0' {
                t1_count += 1;
            } else {
                t0_count += 1;
            }
        } else {
            if s[i] == '1' {
                t1_count += 1;
            } else {
                t0_count += 1;
            }
        }
    }
    println!("{}", std::cmp::min(t0_count, t1_count));
}
