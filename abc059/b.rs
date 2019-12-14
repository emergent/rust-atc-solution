fn main() {
    let a = read::<String>().chars().collect::<Vec<_>>();
    let b = read::<String>().chars().collect::<Vec<_>>();
    if a.len() > b.len() {
        println!("GREATER");
        return;
    } else if a.len() < b.len() {
        println!("LESS");
        return;
    } else {
        for i in 0..a.len() {
            if a[i] > b[i] {
                println!("GREATER");
                return;
            } else if a[i] < b[i] {
                println!("LESS");
                return;
            } else {
                continue;
            }
        }
    }
    println!("EQUAL");
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
