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
fn yn(result: bool) {
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_agct(c: char) -> bool {
    c == 'A' || c == 'C' || c == 'G' || c == 'T'
}

fn main() {
    let s = read::<String>();
    let mut max_count = 0;
    let mut count = 0;
    for c in s.chars() {
        if is_agct(c) {
            count += 1;
        } else {
            max_count = std::cmp::max(max_count, count);
            count = 0;
        }
    }
    max_count = std::cmp::max(max_count, count);
    println!("{}", max_count);
}
