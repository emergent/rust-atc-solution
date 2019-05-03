fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn sum_digits(i: i32) -> i32 {
    i.to_string().chars().map(|j| j as i32 - 48).fold(0, |sum, x| sum + x)
}

fn main() {
    let v = read_vec::<i32>();
    let n = v[0];
    let a = v[1];
    let b = v[2];

    let mut sumall = 0;
    for i in 1..n+1 {
        let s = sum_digits(i);
        if s >= a && s <= b {
            sumall += i;
        }
    }
    println!("{}", sumall);
}
