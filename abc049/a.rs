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
        println!("vowel");
    } else {
        println!("consonant");
    }
}

fn main() {
    let c = read::<String>().chars().nth(0).unwrap();

    yn(c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u');
}
