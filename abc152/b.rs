fn main() {
    let v = read_vec::<usize>();
    let a = v[0];
    let b = v[1];
    if a >= b {
        let x = vec![b; a];
        println!(
            "{}",
            x.into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    } else {
        let x = vec![a; b];
        println!(
            "{}",
            x.into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
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
