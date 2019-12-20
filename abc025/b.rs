fn main() {
    let v = read_vec::<i32>();
    let n = v[0];
    let a = v[1];
    let b = v[2];

    let mut distance = 0;
    for _ in 0..n {
        let vv = read_vec::<String>();
        let s = &vv[0];
        let mut d = vv[1].parse::<i32>().unwrap();

        if d < a {
            d = a;
        }
        if d > b {
            d = b;
        }
        if s == "West" {
            d = -d;
        }
        distance += d;
    }

    if distance == 0 {
        println!("0");
    } else if distance > 0 {
        println!("East {}", distance);
    } else {
        println!("West {}", -distance);
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
