fn main() {
    let s = read::<String>().chars().collect::<Vec<_>>();
    let mut v = vec![];
    let mut last_c = s[0];
    let mut count = 1;

    for i in 1..s.len() {
        if last_c == s[i] {
            count += 1;
        } else {
            let s2 = format!("{}{}", last_c, count);
            v.push(s2);
            last_c = s[i];
            count = 1;
        }
    }
    let s3 = format!("{}{}", last_c, count);
    v.push(s3);
    println!("{}", v.join(""));
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
