fn main() {
    let mut s = read::<String>().chars().collect::<Vec<char>>();
    let len = s.len();
    for i in 0..len {
        if i == 0 {
            if (s[i] as u8) >= 97 {
                s[i] = (s[i] as u8 - 32) as char;
            }
        } else {
            if (s[i] as u8) < 97 {
                s[i] = (s[i] as u8 + 32) as char;
            }
        }
    }
    println!("{}", s.into_iter().collect::<String>());
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
