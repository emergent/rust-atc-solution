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

fn canadd(s: &str, adds: &Vec<&str>) -> bool {
    if s.len() < 5 {
        return false;
    }

    let mut retb = false;

    for &a in adds {
        if s == a {
            return true;
        } else if s.starts_with(a) {
            retb |= canadd(s.split_at(a.len()).1, adds);
        } else {
            retb |= false;
        }
    }

    retb
}

fn main() {
    let s = read::<String>();

    let adds = vec!["dream", "dreamer", "erase", "eraser"];

    match canadd(&s, &adds) {
        true => println!("YES"),
        false => println!("NO"),
    }
}
