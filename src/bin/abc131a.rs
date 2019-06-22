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
    let n = read::<String>();
    let v = n.chars().map(|c| c as i8 + 48).collect::<Vec<i8>>();
    let mut f = false;
    for i in 0..3 {
        if v[i] == v[i + 1] {
            f = true;
        }
    }
    if f {
        println!("Bad");
    } else {
        println!("Good");
    }
}
